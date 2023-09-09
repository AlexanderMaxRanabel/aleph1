use rand::*;

pub fn salt_generate(length: u8) -> String {
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let random_char = match rng.gen_range(0..3) {
                0 => rng.gen_range(48..58), // 0-9
                1 => rng.gen_range(65..91), // A-Z
                _ => rng.gen_range(97..123), // a-z
            };
            std::char::from_u32(random_char).unwrap()
        })
        .collect()
}