use arkhe_afir::{AfirReceipt, AfirSigner};
use arkhe_core::Digest32;
use arkhe_crypto::CryptoError;
use arkhe_spice::InferenceChain;

pub struct InferenceRouter {
    chain: InferenceChain,
    afir_signer: AfirSigner,
}

impl InferenceRouter {
    pub fn new(afir_signer: AfirSigner) -> Self {
        Self {
            chain: InferenceChain::new(),
            afir_signer,
        }
    }

    pub fn route_inference(
        &mut self,
        model_cid_digest: Digest32,
        input: &[u8],
        output: &[u8],
        duration_ns: u64,
    ) -> Result<AfirReceipt, CryptoError> {
        let input_digest = Digest32::blake3(input);
        let output_digest = Digest32::blake3(output);

        self.chain.append(output_digest);
        let inference_root = self.chain.root();

        let receipt = self.afir_signer.sign_inference(
            model_cid_digest,
            input_digest,
            output_digest,
            inference_root,
            1730000000,
            duration_ns,
        )?;

        Ok(receipt)
    }

    pub fn current_inference_root(&self) -> Digest32 {
        self.chain.root()
    }
}
