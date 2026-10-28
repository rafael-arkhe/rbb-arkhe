"""Independent eth-abi/PyCryptodome/BLAKE3 oracle; fixed public test data only."""
from pathlib import Path
import argparse
import json
import eth_abi
from Crypto.Hash import keccak
import blake3

ROOT = Path(__file__).resolve().parents[1]
FIELDS = [
    ('protocol_version','uint8'), ('profile_id','uint16'), ('anchor_chain_id','uint256'),
    ('anchor_genesis_hash','bytes32'), ('registry_address','address'), ('anchor_address','address'),
    ('org_id','bytes32'), ('power','uint8'), ('stream_id','bytes32'), ('sequence','uint64'),
    ('prev_checkpoint_id','bytes32'), ('first_leaf_index','uint64'), ('batch_leaf_count','uint64'),
    ('total_leaf_count','uint64'), ('mmr_root','bytes32'), ('batch_manifest_digest','bytes32'),
    ('policy_digest','bytes32'), ('declared_signed_at','uint64'), ('source_kind','uint8'),
    ('source_network_id','bytes32'), ('source_manifest_digest','bytes32'),
    ('ed25519_key_id','bytes32'), ('mldsa_key_id','bytes32'), ('keyset_version','uint64'),
    ('prev_present','bool'), ('source_manifest_present','bool'), ('mmr_profile_digest','bytes32'),
]
HEADER_TYPE = '(' + ','.join(t for _, t in FIELDS) + ')'
def k(data):
    h = keccak.new(digest_bits=256); h.update(data); return h.digest()
def h(domain, *parts):
    return blake3.blake3(domain + b''.join(parts)).digest()
def build_mmr(records):
    peaks = []
    for index, record in enumerate(records):
        current = h(b'AKG:MMR:LEAF:v3\0', index.to_bytes(8,'big'), record)
        height = 0
        while peaks and peaks[-1][0] == height:
            current = h(b'AKG:MMR:NODE:v3\0', peaks.pop()[1], current); height += 1
        peaks.append((height,current))
    bag = peaks[-1][1]
    for _, peak in reversed(peaks[:-1]): bag = h(b'AKG:MMR:BAG:v3\0', peak, bag)
    return h(b'AKG:MMR:ROOT:v3\0', len(records).to_bytes(8,'big'), bag)
def vectors():
    profile = blake3.blake3(b'arkhe-rbb-mmr-v3:blake3-256;leaf=index-u64be+record32;node=left+right;bag=right-fold;root=count-u64be+bag;domains=AKG:MMR:{LEAF,NODE,BAG,ROOT}:v3:NUL').digest()
    header = {}
    for i, (name, typ) in enumerate(FIELDS):
        if typ == 'bytes32': header[name] = bytes([i+1])*32
        elif typ == 'address': header[name] = bytes([i+1])*20
        elif typ == 'bool': header[name] = False
        else: header[name] = 1
    header.update(protocol_version=3, profile_id=1, anchor_chain_id=1337,
        prev_checkpoint_id=bytes(32), first_leaf_index=0, batch_leaf_count=1,
        total_leaf_count=1, declared_signed_at=1700000000, source_manifest_digest=bytes(32),
        source_kind=1, mmr_profile_digest=profile)
    value = tuple(header[name] for name,_ in FIELDS)
    raw = eth_abi.encode([HEADER_TYPE], [value])
    domain = k(b'AKG:CHECKPOINT:HEADER:v3\0')
    envelope = eth_abi.encode([HEADER_TYPE,'bytes','bytes'], [value, bytes([0x55])*64, bytes([0xaa])*2420])
    assert len(raw) == 864 and len(envelope) == 3488
    decoded = eth_abi.decode([HEADER_TYPE,'bytes','bytes'], envelope)
    assert eth_abi.encode([HEADER_TYPE,'bytes','bytes'], decoded) == envelope
    serialized = {name: list(v) if isinstance(v,bytes) else v for name,v in header.items()}
    serialized['anchor_chain_id'] = list((1337).to_bytes(32,'big'))
    mmr = [{'count':n, 'root_hex':build_mmr([bytes([i+1])*32 for i in range(n)]).hex()} for n in [1,2,3,5,7,8,9,13]]
    return {'format':'arkhe-evidence/3-candidate-rev2', 'fixture_only':True,
        'header':serialized, 'abi_types':[t for _,t in FIELDS],
        'header_abi_hex':raw.hex(), 'checkpoint_id_hex':k(domain+raw).hex(),
        'envelope_abi_hex':envelope.hex(), 'envelope_commitment_hex':k(envelope).hex(),
        'mmr_profile_digest_hex':profile.hex(), 'mmr':mmr}

if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--write',action='store_true'); args=parser.parse_args()
    path = ROOT/'fixtures'/'protocol-v3.json'; actual=vectors()
    if args.write:
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_text(json.dumps(actual,indent=2)+'\n',encoding='utf-8')
    else:
        assert json.loads(path.read_text(encoding='utf-8')) == actual, 'reference vector mismatch'
    print('PASS: independent Python ABI, Keccak and eight MMR reference vectors')
