use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LocalMmr {
    pub leaves: Vec<Digest32>,
    pub peaks: Vec<Digest32>,
}

impl LocalMmr {
    pub fn new() -> Self {
        Self { leaves: Vec::new(), peaks: Vec::new() }
    }

    pub fn append(&mut self, leaf_digest: Digest32) -> usize {
        let index = self.leaves.len();
        self.leaves.push(leaf_digest);
        self.recompute_peaks();
        index
    }

    fn recompute_peaks(&mut self) {
        let mut peaks = Vec::new();
        let mut n = self.leaves.len();
        let mut offset = 0;
        while n > 0 {
            let k = 63 - (n as u64).leading_zeros();
            let count = 1 << k;
            let peak_hash = self.compute_sub_peak(offset, count);
            peaks.push(peak_hash);
            offset += count;
            n -= count;
        }
        self.peaks = peaks;
    }

    fn compute_sub_peak(&self, offset: usize, count: usize) -> Digest32 {
        if count == 1 {
            return self.leaves[offset];
        }
        let half = count / 2;
        let left = self.compute_sub_peak(offset, half);
        let right = self.compute_sub_peak(offset + half, half);
        let mut h = blake3::Hasher::new();
        h.update(b"AKG:MMR:PEAK:v1\0");
        h.update(left.as_bytes());
        h.update(right.as_bytes());
        Digest32(*h.finalize().as_bytes())
    }

    pub fn bagged_root(&self) -> Digest32 {
        if self.peaks.is_empty() {
            return Digest32::ZERO;
        }
        let mut current = self.peaks[0];
        for peak in &self.peaks[1..] {
            let mut h = blake3::Hasher::new();
            h.update(b"AKG:MMR:BAG:v1\0");
            h.update(current.as_bytes());
            h.update(peak.as_bytes());
            current = Digest32(*h.finalize().as_bytes());
        }
        current
    }

    pub fn verify_historical_consistency(&self, previous_count: usize, previous_leaves: &[Digest32]) -> bool {
        if previous_count > self.leaves.len() {
            return false;
        }
        &self.leaves[..previous_count] == previous_leaves
    }
}
