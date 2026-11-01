use crate::bundle::{OfflineVerificationBundle, OfflineVerifier};

pub fn verify_bundle_json(bundle_json: &str) -> bool {
    if let Ok(bundle) = serde_json::from_str::<OfflineVerificationBundle>(bundle_json) {
        OfflineVerifier::verify_bundle(&bundle).is_ok()
    } else {
        false
    }
}
