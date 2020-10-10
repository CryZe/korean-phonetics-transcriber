use unicode_normalization::UnicodeNormalization;

#[derive(Copy, Clone)]
pub enum Consonant {
    /// ㅂ
    B,
    /// ㅈ
    J,
    /// ㄷ
    D,
    /// ㄱ
    G,
    /// ㅅ
    S,
    /// ㅁ
    M,
    /// ㄴ
    N,
    /// ㅇ
    Ng,
    /// ㄹ
    L,
    /// ㅎ
    H,
    /// ㅋ
    K,
    /// ㅌ
    T,
    /// ㅊ
    Ch,
    /// ㅍ
    P,
    /// ㅃ
    Bb,
    /// ㄲ
    Gg,
    /// ㄸ
    Dd,
}

#[derive(Copy, Clone)]
pub enum Vowel {
    /// ㅐ
    Ae,
    /// ㅔ
    E,
    /// ㅗ
    O,
    /// ㅓ
    Eo,
    /// ㅏ
    A,
    /// ㅣ
    I,
    /// ㅜ
    U,
    /// ㅡ
    Eu,
    /// ㅟ
    Wi,
    /// ㅙ
    Wae,
    /// ㅘ
    Wa,
    /// ㅝ
    Wo,
    /// ㅞ
    We,
    /// ㅒ
    Yae,
    /// ㅑ
    Ya,
    /// ㅕ
    Yeo,
    /// ㅖ
    Ye,
    /// ㅛ
    Yo,
    /// ㅠ
    Yu,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Position {
    InitialConsonant,
    Vowel,
    FinalConsonant,
    SomeConsonant,
}

impl Position {
    fn advance(&mut self) {
        *self = match self {
            Position::InitialConsonant => Position::Vowel,
            Position::Vowel => Position::SomeConsonant,
            Position::SomeConsonant => Position::FinalConsonant,
            Position::FinalConsonant => Position::InitialConsonant,
        };
    }
}

pub struct Builder {
    buf: String,
    pos: Position,
    buffered_cons: Option<Consonant>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            buf: String::new(),
            pos: Position::InitialConsonant,
            buffered_cons: None,
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_start_of_word(&self) -> bool {
        (self.buf.is_empty() || self.buf.ends_with(' ')) && self.pos == Position::InitialConsonant
    }

    pub fn advance_to(&mut self, pos: Position) {
        while self.pos != pos {
            match self.pos {
                Position::InitialConsonant => {
                    self.buf.push('ᄋ');
                }
                Position::Vowel => {
                    self.buf.push('ᅳ');
                }
                _ => {}
            };
            self.pos.advance();
        }
    }

    pub fn push_consonant(&mut self, cons: Consonant) {
        if let Some(earlier_cons) = self.buffered_cons.take() {
            self.pos = match (earlier_cons, cons) {
                // Avoid some combinations
                (Consonant::P, Consonant::L) => Position::InitialConsonant,
                _ => Position::FinalConsonant,
            };
            self.push_consonant(earlier_cons);
        }

        if self.pos == Position::Vowel {
            self.advance_to(Position::SomeConsonant);
        }

        let c = match self.pos {
            Position::InitialConsonant => match cons {
                Consonant::B => 'ᄇ',
                Consonant::J => 'ᄌ',
                Consonant::D => 'ᄃ',
                Consonant::G => 'ᄀ',
                Consonant::S => 'ᄉ',
                Consonant::M => 'ᄆ',
                Consonant::N => 'ᄂ',
                Consonant::Ng => 'ᄋ',
                Consonant::L => 'ᄅ',
                Consonant::H => 'ᄒ',
                Consonant::K => 'ᄏ',
                Consonant::T => 'ᄐ',
                Consonant::Ch => 'ᄎ',
                Consonant::P => 'ᄑ',
                Consonant::Bb => 'ᄈ',
                Consonant::Gg => 'ᄁ',
                Consonant::Dd => 'ᄄ',
            },
            Position::Vowel => unreachable!(),
            Position::FinalConsonant => match cons {
                Consonant::B => 'ᆸ',
                Consonant::J => 'ᆽ',
                Consonant::D => 'ᆮ',
                Consonant::G => 'ᆨ',
                Consonant::S => 'ᆺ',
                Consonant::M => 'ᆷ',
                Consonant::N => 'ᆫ',
                Consonant::Ng => 'ᆼ',
                Consonant::L => 'ᆯ',
                Consonant::H => 'ᇂ',
                Consonant::K => 'ᆿ',
                Consonant::T => 'ᇀ',
                Consonant::Ch => 'ᆾ',
                Consonant::P => 'ᇁ',
                Consonant::Bb => panic!("ㅃ can't be in final consonant position"),
                Consonant::Gg => panic!("ㄲ can't be in final consonant position"),
                Consonant::Dd => panic!("ㄸ can't be in final consonant position"),
            },
            Position::SomeConsonant => {
                self.buffered_cons = Some(cons);
                self.pos = Position::InitialConsonant;
                return;
            }
        };
        self.buf.push(c);
        self.pos.advance();
    }

    fn finish_syllable(&mut self) {
        if let Some(earlier_cons) = self.buffered_cons.take() {
            self.pos = Position::FinalConsonant;
            self.push_consonant(earlier_cons);
        }

        self.advance_to(Position::InitialConsonant);
    }

    pub fn push_space(&mut self) {
        self.finish_syllable();
        self.buf.push(' ');
    }

    pub fn push_vowel(&mut self, vowel: Vowel) {
        if let Some(earlier_cons) = self.buffered_cons.take() {
            self.pos = Position::InitialConsonant;
            self.push_consonant(earlier_cons);
        }

        self.advance_to(Position::Vowel);
        self.buf.push(match vowel {
            Vowel::Ae => 'ᅢ',
            Vowel::E => 'ᅦ',
            Vowel::O => 'ᅩ',
            Vowel::Eo => 'ᅥ',
            Vowel::A => 'ᅡ',
            Vowel::I => 'ᅵ',
            Vowel::U => 'ᅮ',
            Vowel::Eu => 'ᅳ',
            Vowel::Wi => 'ᅱ',
            Vowel::Wae => 'ᅫ',
            Vowel::Wa => 'ᅪ',
            Vowel::Wo => 'ᅯ',
            Vowel::We => 'ᅰ',
            Vowel::Yae => 'ᅤ',
            Vowel::Ya => 'ᅣ',
            Vowel::Yeo => 'ᅧ',
            Vowel::Ye => 'ᅨ',
            Vowel::Yo => 'ᅭ',
            Vowel::Yu => 'ᅲ',
        });
        self.pos.advance();
    }

    pub fn finish(mut self) -> String {
        self.finish_syllable();
        self.buf.nfc().collect()
    }
}
