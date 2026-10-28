// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.30;

/// @dev Test fixture only. No production governance, proof of possession or key lifecycle.
contract RegistryFixture {
    struct Org { address operator; uint8 power; bool active; }
    struct Keys { bytes32 ed; bytes32 ml; uint64 validFrom; uint64 validUntil; bool active; }
    mapping(bytes32 => Org) public organization;
    mapping(bytes32 => mapping(uint64 => Keys)) public keyset;
    function setOrg(bytes32 id, address operator, uint8 power, bool active) external {
        organization[id] = Org(operator, power, active);
    }
    function setKeys(bytes32 id, uint64 version, bytes32 ed, bytes32 ml, uint64 from, uint64 until, bool active) external {
        keyset[id][version] = Keys(ed, ml, from, until, active);
    }
}
