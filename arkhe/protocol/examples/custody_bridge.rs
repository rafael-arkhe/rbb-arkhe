//! PUBLIC TEST KEYS ONLY. Recomputes a complete single-batch MMR and checks a v3 envelope.
#[cfg(feature = "rbb-pqc")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use arkhe_rbb_core::rbb::{
        keccak, mmr,
        trust::{verify_hybrid, HybridKey},
        Envelope, MLDSA_CONTEXT,
    };
    use fips204::{
        ml_dsa_44,
        traits::{KeyGen, SerDes},
    };
    use std::io::{self, Read};
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Request {
        records: Vec<String>,
        manifest_digest: String,
        envelope: Option<Envelope>,
    }
    let mut data = Vec::new();
    io::stdin().take(1_048_577).read_to_end(&mut data)?;
    if data.len() > 1_048_576 {
        return Err("RESOURCE_LIMIT".into());
    }
    let request: Request = serde_json::from_slice(&data)?;
    if request.records.is_empty() || request.records.len() > 128 {
        return Err("RESOURCE_LIMIT".into());
    }
    let mut frontier = mmr::Frontier::default();
    for record in request.records {
        let bytes: [u8; 32] = hex::decode(record)?
            .try_into()
            .map_err(|_| "record length")?;
        frontier.append(&bytes)?;
    }
    let root = frontier.root()?;
    if let Some(envelope) = request.envelope {
        let ed = ed25519_dalek::SigningKey::from_bytes(&[7; 32]);
        let (pk, _) = ml_dsa_44::KG::keygen_from_seed(&[7; 32]);
        let key = HybridKey {
            ed25519: ed.verifying_key().to_bytes(),
            mldsa: pk.into_bytes().to_vec(),
        };
        let manifest: [u8; 32] = hex::decode(request.manifest_digest)?
            .try_into()
            .map_err(|_| "manifest length")?;
        let h = &envelope.header;
        if h.mmr_root != root
            || h.total_leaf_count != frontier.count
            || h.first_leaf_index != 0
            || h.batch_leaf_count != frontier.count
            || h.sequence != 1
            || h.batch_manifest_digest != manifest
            || h.mmr_profile_digest != mmr::profile_digest()
            || h.ed25519_key_id != keccak(&key.ed25519)
            || h.mldsa_key_id != keccak(&key.mldsa)
        {
            return Err("CHECKPOINT_BINDING".into());
        }
        envelope.encode()?;
        verify_hybrid(
            &key,
            &h.checkpoint_id()?,
            &envelope.ed25519_signature,
            &envelope.mldsa_signature,
            MLDSA_CONTEXT,
        )?;
    }
    println!(
        "{}",
        serde_json::json!({"root":hex::encode(root),"count":frontier.count.to_string(),"profile":hex::encode(mmr::profile_digest()),"test_keys_public":true})
    );
    Ok(())
}
#[cfg(not(feature = "rbb-pqc"))]
fn main() {
    eprintln!("requires rbb-pqc; test only");
    std::process::exit(2);
}
