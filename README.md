This crate exposes the EFF wordlists under a Rust API.
See EFF's [blog post](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) for more information about the wordlists.
It contains both `large` and `short` wordlists, each of them available via their respective modules.

## Examples of usage with the `large` wordlist:

You can ask for a random word
```
let word = eff_wordlist::large::random_word();
```

or use the list as it is
```
let (first_roll, first_word) = eff_wordlist::large::LIST[0];
assert_eq!(11111, first_roll);
assert_eq!("abacus", first_word);
```


