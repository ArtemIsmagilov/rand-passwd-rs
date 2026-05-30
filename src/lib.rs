use rand::distr::{SampleString, slice::Choose};

const CHARSET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=',
    '[', ']', '{', '}', '|', ';', ':', ',', '.', '<', '>', '?',
];

fn generate_password(r: &mut impl rand::Rng, length: usize) -> String {
    let passwd_dist = Choose::new(CHARSET).unwrap();
    passwd_dist.sample_string(r, length)
}

pub fn rand_gen_pass(length: usize) -> String {
    generate_password(&mut rand::rng(), length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_seed_0() {
        let mut test_rng = StdRng::seed_from_u64(42);
        let passwd = generate_password(&mut test_rng, 0);
        assert_eq!(passwd, "");
    }

    #[test]
    fn test_seed_3() {
        let mut test_rng = StdRng::seed_from_u64(42);
        let passwd = generate_password(&mut test_rng, 3);
        assert_eq!(passwd, "LuV");
    }

    #[test]
    fn test_seed_10() {
        let mut test_rng = StdRng::seed_from_u64(42);
        let passwd = generate_password(&mut test_rng, 10);
        assert_eq!(passwd, "LuVv[4?j<D");
    }
}
