use std::collections::HashMap;

use unicase::UniCase;

fn map_char_to_ipa(s: &str) -> &'static str {
    match s.trim_end_matches(|c: char| c.is_numeric()) {
        "AA" => "ɑ",
        "AE" => "æ",
        "AH" => "ʌ",
        "AO" => "ɔ",
        "AW" => "aʊ",
        "AY" => "aɪ",
        "B" => "b",
        "CH" => "tʃ",
        "D" => "d",
        "DH" => "ð",
        "EH" => "ɛ",
        "ER" => "ɝ",
        "EY" => "eɪ",
        "F" => "f",
        "G" => "ɡ",
        "HH" => "h",
        "IH" => "ɪ",
        "IY" => "i",
        "JH" => "dʒ",
        "K" => "k",
        "L" => "l",
        "M" => "m",
        "N" => "n",
        "NG" => "ŋ",
        "OW" => "oʊ",
        "OY" => "ɔɪ",
        "P" => "p",
        "R" => "ɹ",
        "S" => "s",
        "SH" => "ʃ",
        "T" => "t",
        "TH" => "θ",
        "UH" => "ʊ",
        "UW" => "u",
        "V" => "v",
        "W" => "w",
        "Y" => "j",
        "Z" => "z",
        "ZH" => "ʒ",
        _ => panic!("Unknown ARPABET character."),
    }
}

pub struct Dictionary<'txt> {
    map: HashMap<UniCase<&'txt str>, &'txt str>,
}

impl<'txt> Dictionary<'txt> {
    pub fn parse(txt: &'txt str) -> Option<Self> {
        Some(Self {
            map: txt
                .lines()
                .filter(|l| !l.starts_with(";;;"))
                .map(|l| {
                    let mut splits = l.splitn(2, "  ");
                    Some((splits.next()?.into(), splits.next()?))
                })
                .collect::<Option<_>>()?,
        })
    }

    pub fn look_up(&self, word: &str) -> Option<impl Iterator<Item = char> + Clone + 'txt> {
        let arpa_chars = self.map.get(&word.into())?;
        Some(
            arpa_chars
                .split_whitespace()
                .flat_map(|c| map_char_to_ipa(c).chars()),
        )
    }
}

pub const CMUDICT_07B: &str = include_str!("cmudict-0.7b.txt");
