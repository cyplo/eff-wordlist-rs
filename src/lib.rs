mod eff_large_wordlist;
use eff_large_wordlist::LARGE;

pub mod large {
    use crate::LARGE;
    use rand::rngs::OsRng;
    use rand::seq::SliceRandom;

    pub fn random_word() -> &'static str {
        let mut rng = OsRng::new().expect("Not able to operate without random source.");
        LARGE.choose(&mut rng).unwrap().1
    }

    
}
