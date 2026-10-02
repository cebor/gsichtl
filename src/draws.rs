//! Deterministic draws from a seed.
//!
//! BLAKE3 serves as a mixer here, not for cryptography: its extendable output
//! turns any seed into an endless, reproducible stream of bytes.

pub(crate) struct Draws(blake3::OutputReader);

impl Draws {
    /// A stream for `seed`, separated from the other generators by `context`.
    pub(crate) fn new(context: &'static str, seed: &[u8]) -> Self {
        let mut hasher = blake3::Hasher::new_derive_key(context);
        hasher.update(seed);
        Self(hasher.finalize_xof())
    }

    /// A number in `0..n`; `0` when `n` is `0`.
    pub(crate) fn pick(&mut self, n: usize) -> usize {
        let mut bytes = [0u8; 4];
        self.0.fill(&mut bytes);
        u32::from_le_bytes(bytes) as usize % n.max(1)
    }

    /// A number in `0..n` other than `taken`.
    pub(crate) fn pick_other(&mut self, n: usize, taken: usize) -> usize {
        let i = self.pick(n.saturating_sub(1));
        if i >= taken { i + 1 } else { i }
    }

    /// True in `percent` of a hundred draws.
    pub(crate) fn chance(&mut self, percent: usize) -> bool {
        self.pick(100) < percent
    }
}
