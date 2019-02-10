//! This crate exposes the EFF wordlists under a Rust API.
//! See EFF's [blog post](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) for more information about the wordlists.

mod eff_large_wordlist;

/// large wordlist
pub mod large {
    use rand::rngs::OsRng;
    use rand::seq::SliceRandom;

    /// The wordlist as an array of tuples - the first element being the dice roll and second the equivalent word
    pub const LIST: &'static [(u32, &'static str)] = crate::eff_large_wordlist::LARGE;

    pub fn random_word() -> &'static str {
        let mut rng = OsRng::new().expect("Not able to operate without random source.");
        LIST.choose(&mut rng).unwrap().1
    }


}
