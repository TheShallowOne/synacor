pub fn sum_bytes(b: &[u8]) -> u64 {
    b.iter().map(|&b| b as u64).sum()
}
