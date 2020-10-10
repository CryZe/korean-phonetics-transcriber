use crate::hangul_builder::{self, Consonant, Position, Vowel};

pub fn convert(phonetics: impl IntoIterator<Item = char>) -> String {
    let mut builder = hangul_builder::Builder::new();

    let mut chars = phonetics.into_iter().peekable();

    while let Some(c) = chars.next() {
        match c {
            'n' => builder.push_consonant(Consonant::N),
            'ʌ' | 'ɔ' | 'ɒ' | 'ɑ' => builder.push_vowel(Vowel::Eo),
            'k' => builder.push_consonant(if builder.is_start_of_word() {
                Consonant::G
            } else {
                Consonant::K
            }),
            't' => {
                let cons = if chars.peek().copied() == Some('ʃ') {
                    chars.next();
                    Consonant::Ch
                } else if builder.is_start_of_word() {
                    Consonant::D
                } else {
                    Consonant::T
                };

                builder.push_consonant(cons)
            }
            'p' => builder.push_consonant(if builder.is_start_of_word() {
                Consonant::B
            } else {
                Consonant::P
            }),
            'b' => builder.push_consonant(if builder.is_start_of_word() {
                Consonant::Bb
            } else {
                Consonant::B
            }),
            'g' | 'ɡ' => builder.push_consonant(if builder.is_start_of_word() {
                Consonant::Gg
            } else {
                Consonant::G
            }),
            'd' => builder.push_consonant(if chars.peek().copied() == Some('ʒ') {
                chars.next();
                Consonant::J
            } else if builder.is_start_of_word() {
                Consonant::Dd
            } else {
                Consonant::D
            }),
            'ð' | 'θ' => builder.push_consonant(Consonant::D),
            'l' => builder.push_consonant(Consonant::L),
            'r' | 'ɹ' => {
                if chars.peek().is_some() {
                    builder.push_consonant(Consonant::L)
                }
            }
            'ə' | 'ɜ' | 'ɝ' => builder.push_vowel(Vowel::Eu),
            'a' | 'ɐ' => builder.push_vowel(Vowel::A),
            'ʊ' | 'u' => builder.push_vowel(Vowel::U),
            's' | 'z' => {
                builder.advance_to(Position::InitialConsonant);
                builder.push_consonant(Consonant::S)
            }
            'h' => builder.push_consonant(Consonant::H),
            'm' => builder.push_consonant(Consonant::M),
            'j' | 'ɪ' | 'y' | 'i' => {
                let mut vowel = Vowel::I;
                if let Some(c) = chars.peek() {
                    if let Some(v) = match c {
                        'ɛ' | 'æ' => Some(Vowel::Yae),
                        'a' | 'ɐ' => Some(Vowel::Ya),
                        'ʌ' | 'ɔ' | 'ɒ' | 'ɑ' => Some(Vowel::Yeo),
                        'e' => Some(Vowel::Ye),
                        'o' => Some(Vowel::Yo),
                        'ʊ' | 'u' => Some(Vowel::Yu),
                        _ => None,
                    } {
                        chars.next();
                        vowel = v;
                    }
                }
                builder.push_vowel(vowel)
            }
            'f' => builder.push_consonant(Consonant::P),
            'o' => builder.push_vowel(Vowel::O),
            'ŋ' => {
                builder.advance_to(Position::FinalConsonant);
                builder.push_consonant(Consonant::Ng)
            }
            'ʃ' | 'ʒ' => {
                builder.push_consonant(Consonant::S);
                // TODO: Suboptimal af
                if !matches!(
                    chars.peek().copied(),
                    Some('j') | Some('ɪ') | Some('y') | Some('i')
                ) {
                    builder.push_vowel(Vowel::I)
                }
            }
            'w' | 'v' => {
                if let Some(c) = chars.peek() {
                    if let Some(v) = match c {
                        'j' | 'ɪ' | 'y' | 'i' => Some(Vowel::Wi),
                        'ɛ' | 'æ' => Some(Vowel::Wae),
                        'a' | 'ɐ' => Some(Vowel::Wa),
                        'o' | 'ʌ' | 'ɔ' | 'ɒ' | 'ɑ' => Some(Vowel::Wo),
                        'e' => Some(Vowel::We),
                        _ => None,
                    } {
                        chars.next();
                        builder.push_vowel(v);
                    } else {
                        builder.push_vowel(Vowel::U);
                    }
                } else {
                    builder.push_consonant(Consonant::B);
                }
            }
            'e' => builder.push_vowel(Vowel::E),
            'ɛ' | 'æ' => builder.push_vowel(Vowel::Ae),
            'ʦ' => {
                builder.advance_to(Position::FinalConsonant);
                builder.push_consonant(Consonant::T);
                builder.push_consonant(Consonant::S);
            }
            'ˈ' | 'ː' | '\'' | 'ˌ' => {} // Explicitly ignored
            '|' => builder.push_space(),
            c => {
                println!("{} is unknown", c);
            }
        }
    }

    builder.finish()
}
