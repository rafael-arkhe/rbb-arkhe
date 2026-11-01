//! JNI Bridge para verificação nativa em dispositivos Android ArkheOS.

pub fn native_verify_offline(bundle_json: &str) -> bool {
    arkhe_mobile_verify::verify_bundle_json(bundle_json)
}
