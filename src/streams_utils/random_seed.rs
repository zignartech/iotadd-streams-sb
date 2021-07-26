use rand::Rng;
pub fn randomSeed() -> String {
  const CHARSET: &[u8] = b"0123456789abcdef";
  const SEED_LEN: usize = 64;
  let mut rng = rand::thread_rng();
  let seed: String = (0..SEED_LEN)
    .map(|_| {
      let idx = rng.gen_range(0, CHARSET.len());
      CHARSET[idx] as char
    })
    .collect();
  seed
}