//! This crate exposes the EFF wordlists under a Rust API.
//! See EFF's [blog post](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) for more information about the wordlists.
//! It contains both `large` and `short` wordlists, each of them available via their respective modules.
//!
//! # Examples:
//!
//! You can ask for a random word
//! ```
//! let word = eff_wordlist::large::random_word();
//! ```
//!
//! or use the list as it is
//! ```
//! let (first_roll, first_word) = eff_wordlist::large::LIST[0];
//! assert_eq!(11111, first_roll);
//! assert_eq!("abacus", first_word);
//! ```
//!

mod eff_large_wordlist;
mod eff_short_wordlist;

/// large wordlist
pub mod large {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    /// The wordlist as an array of tuples - the first element being the dice roll and second the equivalent word
    ///
    /// # Examples:
    ///
    /// ```
    /// let (first_roll, first_word) = eff_wordlist::large::LIST[0];
    /// assert_eq!(11111, first_roll);
    /// assert_eq!("abacus", first_word);
    /// ```
    pub const LIST: &[(u32, &str)] = crate::eff_large_wordlist::LARGE;

    /// Get a random word from the list
    /// # Examples:
    ///
    /// ```
    /// let word = eff_wordlist::large::random_word();
    /// ```
    pub fn random_word() -> &'static str {
        let mut rng = thread_rng();
        LIST.choose(&mut rng).unwrap().1
    }
}

/// short wordlist
pub mod short {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    /// The wordlist as an array of tuples - the first element being the dice roll and second the equivalent word
    ///
    /// # Examples:
    ///
    /// ```
    /// let (first_roll, first_word) = eff_wordlist::short::LIST[0];
    /// assert_eq!(1111, first_roll);
    /// assert_eq!("acid", first_word);
    /// ```
    pub const LIST: &[(u32, &str)] = crate::eff_short_wordlist::SHORT;

    /// Get a random word from the list
    /// # Examples:
    ///
    /// ```
    /// let word = eff_wordlist::short::random_word();
    /// ```
    pub fn random_word() -> &'static str {
        let mut rng = thread_rng();
        LIST.choose(&mut rng).unwrap().1
    }
}
