// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.30;

interface IArkheRegistry {
    function organization(bytes32 orgId) external view returns (address operator, uint8 power, bool active);
    function keyset(bytes32 orgId, uint64 version) external view returns
        (bytes32 edKeyId, bytes32 mlKeyId, uint64 validFrom, uint64 validUntil, bool active);
}

/// @notice Candidate v3 header is the sole authoritative checkpoint representation.
/// @dev RECORDED means account-authorized registration; it does not verify hybrid signatures.
contract ArkheCheckpointAnchor {
    struct Header {
        uint8 protocol_version; uint16 profile_id; uint256 anchor_chain_id;
        bytes32 anchor_genesis_hash; address registry_address; address anchor_address;
        bytes32 org_id; uint8 power; bytes32 stream_id; uint64 sequence;
        bytes32 prev_checkpoint_id; uint64 first_leaf_index; uint64 batch_leaf_count;
        uint64 total_leaf_count; bytes32 mmr_root; bytes32 batch_manifest_digest;
        bytes32 policy_digest; uint64 declared_signed_at; uint8 source_kind;
        bytes32 source_network_id; bytes32 source_manifest_digest; bytes32 ed25519_key_id;
        bytes32 mldsa_key_id; uint64 keyset_version; bool prev_present;
        bool source_manifest_present; bytes32 mmr_profile_digest;
    }
    struct Stream {
        bool exists; bool paused; uint64 lastSequence; uint64 leafCount; bytes32 lastId;
        bytes32 mmrProfile; bytes32 sourceNetwork; bytes32 policy; uint8 sourceKind;
    }
    struct Record { bool exists; bytes32 id; bytes32 commitment; bytes32 root; uint64 count; uint256 blockNumber; }

    bytes32 public constant DOMAIN_HASH = keccak256("AKG:CHECKPOINT:HEADER:v3\x00");
    IArkheRegistry public immutable registry;
    bytes32 public immutable genesisHash;
    address public immutable admin;
    mapping(bytes32 => Stream) public streams;
    mapping(bytes32 => mapping(uint64 => Record)) public records;

    error InvalidHeader(); error Unauthorized(); error SequenceConflict(); error StreamInactive();
    event Recorded(bytes32 indexed orgId, bytes32 indexed streamId, uint64 indexed sequence,
        bytes32 checkpointId, bytes32 commitment, bytes32 root, uint64 count);
    event StreamConfigured(bytes32 indexed orgId, bytes32 indexed streamId);
    event StreamPaused(bytes32 indexed streamKey, bool paused);

    constructor(address registry_, bytes32 genesis_) {
        if (registry_ == address(0) || genesis_ == bytes32(0)) revert InvalidHeader();
        registry = IArkheRegistry(registry_); genesisHash = genesis_; admin = msg.sender;
    }

    function streamKey(bytes32 org, bytes32 stream) public pure returns (bytes32) {
        return keccak256(abi.encode(org, stream));
    }
    function configureStream(bytes32 org, bytes32 stream, bytes32 mmrProfile, bytes32 sourceNetwork, bytes32 policy, uint8 sourceKind) external {
        if (msg.sender != admin) revert Unauthorized();
        bytes32 k = streamKey(org, stream);
        if (streams[k].exists || org == 0 || stream == 0 || mmrProfile == 0 || sourceNetwork == 0 || policy == 0 || sourceKind < 1 || sourceKind > 4) revert InvalidHeader();
        streams[k] = Stream(true, false, 0, 0, 0, mmrProfile, sourceNetwork, policy, sourceKind);
        emit StreamConfigured(org, stream);
    }
    function pauseStream(bytes32 k, bool paused) external {
        if (msg.sender != admin) revert Unauthorized();
        if (!streams[k].exists) revert StreamInactive();
        streams[k].paused = paused; emit StreamPaused(k, paused);
    }

    function checkpointId(Header calldata h) public pure returns (bytes32) {
        return keccak256(abi.encode(DOMAIN_HASH, h));
    }
    function envelopeCommitment(Header calldata h, bytes calldata ed, bytes calldata ml) public pure returns (bytes32) {
        return keccak256(abi.encode(h, ed, ml));
    }

    function record(Header calldata h, bytes calldata ed, bytes calldata ml) external returns (bytes32 id) {
        if (ed.length != 64 || ml.length != 2420) revert InvalidHeader();
        id = checkpointId(h);
        bytes32 commitment = envelopeCommitment(h, ed, ml);
        bytes32 k = streamKey(h.org_id, h.stream_id);
        Record storage existing = records[k][h.sequence];
        if (existing.exists) {
            if (existing.id != id || existing.commitment != commitment) revert SequenceConflict();
            return id; // A historical retry creates no record, even after suspension.
        }
        Stream storage s = streams[k];
        if (!s.exists || s.paused) revert StreamInactive();
        (address operator, uint8 power, bool active) = registry.organization(h.org_id);
        if (!active || msg.sender != operator) revert Unauthorized();
        (bytes32 edId, bytes32 mlId, uint64 validFrom, uint64 validUntil, bool keyActive) = registry.keyset(h.org_id, h.keyset_version);
        if (!keyActive || edId != h.ed25519_key_id || mlId != h.mldsa_key_id ||
            block.timestamp < validFrom || block.timestamp >= validUntil ||
            h.declared_signed_at < validFrom || h.declared_signed_at >= validUntil || h.declared_signed_at > block.timestamp) revert InvalidHeader();
        if (h.protocol_version != 3 || h.profile_id != 1 || h.anchor_chain_id != block.chainid ||
            h.anchor_genesis_hash != genesisHash || h.registry_address != address(registry) || h.anchor_address != address(this) ||
            h.power < 1 || h.power > 3 || h.power != power || h.keyset_version == 0 || h.declared_signed_at == 0 ||
            h.sequence == 0 || h.batch_leaf_count == 0 || h.mmr_root == 0 || h.batch_manifest_digest == 0 ||
            h.ed25519_key_id == 0 || h.mldsa_key_id == 0 || h.mmr_profile_digest != s.mmrProfile ||
            h.source_network_id != s.sourceNetwork || h.policy_digest != s.policy || h.source_kind != s.sourceKind ||
            h.prev_present != (h.sequence > 1) || h.prev_present != (h.prev_checkpoint_id != 0) ||
            h.source_manifest_present != (h.source_manifest_digest != 0) ||
            (h.source_kind == 2 && !h.source_manifest_present)) revert InvalidHeader();
        if (h.sequence != s.lastSequence + 1 || h.prev_checkpoint_id != s.lastId || h.first_leaf_index != s.leafCount ||
            h.first_leaf_index + h.batch_leaf_count != h.total_leaf_count) revert SequenceConflict();
        records[k][h.sequence] = Record(true, id, commitment, h.mmr_root, h.total_leaf_count, block.number);
        s.lastSequence = h.sequence; s.lastId = id; s.leafCount = h.total_leaf_count;
        emit Recorded(h.org_id, h.stream_id, h.sequence, id, commitment, h.mmr_root, h.total_leaf_count);
    }
}
