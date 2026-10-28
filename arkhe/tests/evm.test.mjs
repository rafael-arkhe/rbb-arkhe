import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
import solc from 'solc';
import ganache from 'ganache';
import { AbiCoder, BrowserProvider, ContractFactory, hexlify, keccak256, concat } from 'ethers';

const root = fileURLToPath(new URL('../', import.meta.url));
const vector = JSON.parse(readFileSync(root + 'fixtures/protocol-v3.json', 'utf8'));
const source = name => readFileSync(root + 'contracts/' + name, 'utf8');
const output = JSON.parse(solc.compile(JSON.stringify({
  language:'Solidity', sources:{
    'ArkheCheckpointAnchor.sol':{content:source('ArkheCheckpointAnchor.sol')},
    'RegistryFixture.sol':{content:source('fixtures/RegistryFixture.sol')},
  }, settings:{optimizer:{enabled:true,runs:200}, viaIR:true, evmVersion:'shanghai',
    outputSelection:{'*':{'*':['abi','evm.bytecode.object']}}},
})));
assert.deepEqual((output.errors ?? []).filter(e => e.severity === 'error'), []);
const artifact = (file,name) => output.contracts[file][name];
const anchorArtifact = artifact('ArkheCheckpointAnchor.sol','ArkheCheckpointAnchor');
const registryArtifact = artifact('RegistryFixture.sol','RegistryFixture');
const fields = anchorArtifact.abi.find(f => f.name === 'record').inputs[0].components;
assert.equal(fields.length,27);
assert.deepEqual(fields.map(f=>f.type),vector.abi_types);
const fixedHeader = Object.fromEntries(Object.entries(vector.header).map(([name,value]) =>
  [name, Array.isArray(value) ? (name === 'anchor_chain_id' ? BigInt(hexlify(Uint8Array.from(value))) : hexlify(Uint8Array.from(value))) : value]));
const ed='0x'+'55'.repeat(64), ml='0x'+'aa'.repeat(2420);

async function setup(t, sourceKind=1) {
  const local = ganache.provider({logging:{quiet:true},chain:{chainId:1337,hardfork:'shanghai'},wallet:{deterministic:true,totalAccounts:3}});
  t.after(async()=>{ await local.disconnect(); });
  const provider = new BrowserProvider(local);
  const admin = await provider.getSigner(0), operator = await provider.getSigner(1), stranger = await provider.getSigner(2);
  const registry = await new ContractFactory(registryArtifact.abi,registryArtifact.evm.bytecode.object,admin).deploy();
  await registry.waitForDeployment();
  const anchor = await new ContractFactory(anchorArtifact.abi,anchorArtifact.evm.bytecode.object,admin).deploy(await registry.getAddress(),fixedHeader.anchor_genesis_hash);
  await anchor.waitForDeployment();
  const header={...fixedHeader,registry_address:await registry.getAddress(),anchor_address:await anchor.getAddress(),source_kind:sourceKind};
  if(sourceKind===2) { header.source_manifest_present=true; header.source_manifest_digest='0x'+'77'.repeat(32); }
  await (await registry.setOrg(header.org_id,await operator.getAddress(),header.power,true)).wait();
  await (await registry.setKeys(header.org_id,1,header.ed25519_key_id,header.mldsa_key_id,1600000000,4102444800,true)).wait();
  await (await anchor.configureStream(header.org_id,header.stream_id,header.mmr_profile_digest,header.source_network_id,header.policy_digest,sourceKind)).wait();
  return {provider,anchor,registry,operator,stranger,header};
}
async function rejected(contract,header,edSig=ed,mlSig=ml) {
  await assert.rejects(contract.record.staticCall(header,edSig,mlSig));
}

test('Python, ethers and compiled Solidity produce identical header, ID and envelope commitment',async t=>{
  const {anchor}=await setup(t);
  const coder=AbiCoder.defaultAbiCoder();
  const tuple='tuple('+fields.map(f=>f.type+' '+f.name).join(',')+')';
  assert.equal(coder.encode([tuple],[fixedHeader]).slice(2),vector.header_abi_hex);
  assert.equal(coder.encode([tuple,'bytes','bytes'],[fixedHeader,ed,ml]).slice(2),vector.envelope_abi_hex);
  assert.equal(await anchor.checkpointId(fixedHeader),'0x'+vector.checkpoint_id_hex);
  assert.equal(await anchor.envelopeCommitment(fixedHeader,ed,ml),'0x'+vector.envelope_commitment_hex);
  const domain=await anchor.DOMAIN_HASH();
  assert.equal(keccak256(concat([domain,'0x'+vector.header_abi_hex])),'0x'+vector.checkpoint_id_hex);
});

test('records a checkpoint, emits its binding, and exact retry does not create a second event',async t=>{
  const {anchor,operator,header}=await setup(t); const writer=anchor.connect(operator);
  const id=await anchor.checkpointId(header), commitment=await anchor.envelopeCommitment(header,ed,ml);
  const receipt=await (await writer.record(header,ed,ml)).wait();
  assert.equal(receipt.status,1); assert.equal(receipt.logs.length,1);
  const event=anchor.interface.parseLog(receipt.logs[0]);
  assert.equal(event.args.checkpointId,id); assert.equal(event.args.commitment,commitment);
  const retry=await (await writer.record(header,ed,ml)).wait(); assert.equal(retry.logs.length,0);
  const record=await anchor.records(await anchor.streamKey(header.org_id,header.stream_id),1);
  assert.equal(record.exists,true); assert.equal(record.id,id); assert.equal(record.count,1n);
});

test('unauthorized account, network, contract and keyset substitutions are rejected',async t=>{
  const {anchor,operator,stranger,header}=await setup(t);
  await rejected(anchor.connect(stranger),header);
  for(const mutation of [
    {anchor_chain_id:1338},{anchor_genesis_hash:'0x'+'88'.repeat(32)},
    {anchor_address:await stranger.getAddress()},{registry_address:await stranger.getAddress()},
    {power:2},{profile_id:4},{keyset_version:2},{ed25519_key_id:'0x'+'99'.repeat(32)},
    {org_id:'0x'+'66'.repeat(32)},{mmr_profile_digest:'0x'+'44'.repeat(32)},
  ]) await rejected(anchor.connect(operator),{...header,...mutation});
});

test('zero timestamps, invalid intervals, unknown enum and signature lengths are rejected',async t=>{
  const {anchor,operator,header}=await setup(t); const writer=anchor.connect(operator);
  for(const mutation of [
    {declared_signed_at:0},{declared_signed_at:4102444799},{sequence:0},
    {total_leaf_count:2},{batch_leaf_count:0},{source_kind:255},
    {prev_present:true},{first_leaf_index:1},{mmr_root:'0x'+'00'.repeat(32)},
    {first_leaf_index:(1n<<64n)-1n,batch_leaf_count:2},
  ]) await rejected(writer,{...header,...mutation});
  await rejected(writer,header,'0x',ml); await rejected(writer,header,ed,ml+'00');
});

test('Fabric source requires a nonzero manifest and unambiguous presence flag',async t=>{
  const {anchor,operator,header}=await setup(t,2); const writer=anchor.connect(operator);
  await rejected(writer,{...header,source_manifest_present:false});
  await rejected(writer,{...header,source_manifest_present:false,source_manifest_digest:'0x'+'00'.repeat(32)});
  assert.equal((await (await writer.record(header,ed,ml)).wait()).status,1);
});

test('sequence order, predecessor, interval and sealed envelope conflicts are enforced',async t=>{
  const {anchor,operator,header}=await setup(t); const writer=anchor.connect(operator);
  await (await writer.record(header,ed,ml)).wait();
  await rejected(writer,{...header,mmr_root:'0x'+'33'.repeat(32)});
  await rejected(writer,header,ed,'0x'+'bb'.repeat(2420));
  const second={...header,sequence:2,prev_present:true,prev_checkpoint_id:await anchor.checkpointId(header),first_leaf_index:1,total_leaf_count:2};
  await rejected(writer,{...second,prev_checkpoint_id:'0x'+'22'.repeat(32)});
  await rejected(writer,{...second,sequence:3});
  assert.equal((await (await writer.record(second,ed,ml)).wait()).status,1);
});

test('suspension prevents new writes while historical retry remains readable and idempotent',async t=>{
  const {anchor,registry,operator,header}=await setup(t); const writer=anchor.connect(operator);
  await (await writer.record(header,ed,ml)).wait();
  await (await registry.setOrg(header.org_id,await operator.getAddress(),header.power,false)).wait();
  await (await anchor.pauseStream(await anchor.streamKey(header.org_id,header.stream_id),true)).wait();
  assert.equal(await writer.record.staticCall(header,ed,ml),await anchor.checkpointId(header));
  await rejected(writer,{...header,sequence:2,prev_present:true,prev_checkpoint_id:await anchor.checkpointId(header),first_leaf_index:1,total_leaf_count:2});
});

test('expired or revoked registry keyset cannot authorize a new checkpoint',async t=>{
  const {anchor,registry,operator,header}=await setup(t);
  await (await registry.setKeys(header.org_id,1,header.ed25519_key_id,header.mldsa_key_id,1600000000,1700000001,true)).wait();
  await rejected(anchor.connect(operator),header);
  await (await registry.setKeys(header.org_id,1,header.ed25519_key_id,header.mldsa_key_id,1600000000,4102444800,false)).wait();
  await rejected(anchor.connect(operator),header);
});

test('Rust signs a real hybrid checkpoint and its exact envelope is recorded by the EVM',async t=>{
  const {anchor,registry,operator,header}=await setup(t);
  const raw={...vector.header,registry_address:Array.from(Buffer.from(header.registry_address.slice(2),'hex')),
    anchor_address:Array.from(Buffer.from(header.anchor_address.slice(2),'hex'))};
  const binary=process.env.ARKHE_RBB_FIXTURE_BIN ?? resolve(root,'protocol/target/debug/examples/rbb_sign_fixture'+(process.platform==='win32'?'.exe':''));
  const signed=JSON.parse(execFileSync(binary,[],{input:JSON.stringify(raw),encoding:'utf8',maxBuffer:1024*1024}));
  assert.equal(signed.fixture_only,true); assert.equal(signed.hybrid_signatures_verified,true);
  const realHeader={...header,ed25519_key_id:hexlify(Uint8Array.from(signed.envelope.header.ed25519_key_id)),
    mldsa_key_id:hexlify(Uint8Array.from(signed.envelope.header.mldsa_key_id))};
  const realEd=hexlify(Uint8Array.from(signed.envelope.ed25519_signature));
  const realMl=hexlify(Uint8Array.from(signed.envelope.mldsa_signature));
  await (await registry.setKeys(realHeader.org_id,1,realHeader.ed25519_key_id,realHeader.mldsa_key_id,1600000000,4102444800,true)).wait();
  assert.equal(await anchor.checkpointId(realHeader),'0x'+signed.checkpoint_id_hex);
  assert.equal(await anchor.envelopeCommitment(realHeader,realEd,realMl),'0x'+signed.envelope_commitment_hex);
  const receipt=await (await anchor.connect(operator).record(realHeader,realEd,realMl)).wait();
  const event=anchor.interface.parseLog(receipt.logs[0]);
  assert.equal(event.args.checkpointId,'0x'+signed.checkpoint_id_hex);
  assert.equal(event.args.commitment,'0x'+signed.envelope_commitment_hex);
});
