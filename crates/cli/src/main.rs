use std::env;

use anyhow::{Context, Result};
use phonetics_to_hangul::{darpabet, ipa_to_hangul, word_lookup};
use structopt::StructOpt;

/// Turns a word's pronunciation into 한글 with pronunciation as close as
/// possible to the original word.
#[derive(StructOpt)]
#[structopt(name = "phonetics-to-hangul")]
struct Opt {
    /// The language of the word. Only used when the online dictionary is in use.
    #[structopt(short, long, default_value = "en")]
    lang: String,
    /// Switch to an online dictionary instead.
    #[structopt(short, long)]
    online: bool,
    /// The word to replicate the pronunciation of in 한글.
    word: String,
}

fn try_run() -> Result<()> {
    let opt: Opt = StructOpt::from_args();

    if !opt.online {
        let dictionary = darpabet::Dictionary::parse(darpabet::CMUDICT_07B)
            .context("Failed parsing the dictionary.")?;

        let pronunciation = dictionary
            .look_up(&opt.word)
            .context("The word is not in the dictionary.")?;

        println!("Word: {}", opt.word);

        print!("Pronunciation: ");
        for c in pronunciation.clone() {
            print!("{}", c);
        }
        println!();

        println!("한글: {}", ipa_to_hangul::convert(pronunciation));
    } else {
        let user = env::var("DICT_USER").context(
            "For online usage, you need to provide the \
            user name via the `DICT_USER` environment variable.",
        )?;

        let pass = env::var("DICT_PASS").context(
            "For online usage, you need to provide the \
            user's password via the `DICT_PASS` environment variable.",
        )?;

        let client = word_lookup::Client::new(user, pass);

        let word = client
            .lookup(&opt.word, &opt.lang)
            .context("Failed looking up the word.")?;

        let hangul = ipa_to_hangul::convert(word.pronunciation.chars());

        println!("Word: {}", word.word);
        println!("Pronunciation: {}", word.pronunciation);
        println!("한글: {}", hangul);
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_run() {
        for error in e.chain() {
            println!("{}", error);
        }
    }
}
