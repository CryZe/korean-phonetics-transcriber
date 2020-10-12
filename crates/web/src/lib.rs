#![recursion_limit = "256"]

use phonetics_to_hangul::{arpabet, hangul_builder, ipa_to_hangul};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct State {
    link: ComponentLink<Self>,
    dictionary: arpabet::Dictionary<'static>,
    builder: hangul_builder::Builder,
    pronunciations: String,
    hanguls: String,
    word: String,
}

enum Message {
    WordChanged(InputData),
    PronunciationChanged(InputData),
}

impl State {
    fn recalc_from_word(&mut self) {
        self.pronunciations.clear();
        self.hanguls.clear();

        let word = if self.word.is_empty() {
            "Example Text"
        } else {
            &self.word
        };

        for (i, word) in word.split_whitespace().enumerate() {
            if i != 0 {
                self.pronunciations.push(' ');
                self.hanguls.push(' ');
            }

            if let Some(pronunciation) = self.dictionary.look_up(word) {
                self.pronunciations.extend(pronunciation.clone());
                self.hanguls
                    .extend(ipa_to_hangul::convert(&mut self.builder, pronunciation));
            } else {
                self.pronunciations.push('?');
                self.hanguls.push('?');
            }
        }
    }

    fn recalc_from_pronunciation(&mut self) {
        self.word.clear();
        self.hanguls.clear();

        let pronunciations = if self.pronunciations.is_empty() {
            "ɪɡzæmpʌl tɛkst"
        } else {
            &self.pronunciations
        };

        for (i, pronunciation) in pronunciations.split_whitespace().enumerate() {
            if i != 0 {
                self.hanguls.push(' ');
            }

            self.hanguls.extend(ipa_to_hangul::convert(
                &mut self.builder,
                pronunciation.chars(),
            ));
        }
    }
}

impl Component for State {
    type Message = Message;

    type Properties = ();

    fn create((): (), link: ComponentLink<Self>) -> Self {
        let dictionary = arpabet::Dictionary::parse(arpabet::CMUDICT_07B).unwrap();
        let mut state = Self {
            link,
            dictionary,
            builder: hangul_builder::Builder::new(),
            pronunciations: String::new(),
            hanguls: String::new(),
            word: String::new(),
        };
        state.recalc_from_word();
        state
    }

    fn update(&mut self, message: Message) -> ShouldRender {
        match message {
            Message::WordChanged(change) => {
                self.word.clear();
                self.word.push_str(&change.value);
                self.recalc_from_word();
            }
            Message::PronunciationChanged(change) => {
                self.pronunciations.clear();
                self.pronunciations.push_str(&change.value);
                self.recalc_from_pronunciation();
            }
        }

        true
    }

    fn change(&mut self, (): ()) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>
                    {"English Text:"}
                    <div class="result">
                        <input
                            type="text"
                            placeholder="Example Text"
                            value={&self.word}
                            oninput=self.link.callback(|w| Message::WordChanged(w))
                        />
                    </div>
                </p>
                <p>
                    {"Pronunciation (IPA):"}
                    <div class="result">
                        <input
                            type="text"
                            placeholder="ɪɡzæmpʌl tɛkst"
                            value={&self.pronunciations}
                            oninput=self.link.callback(|w| Message::PronunciationChanged(w))
                        />
                    </div>
                </p>
                <p>
                    {"한글:"}
                    <div class="result">
                        <input
                            type="text"
                            placeholder="익샘펄 댘슽"
                            value={&self.hanguls}
                            readonly=true
                        />
                    </div>
                </p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::initialize();
    App::<State>::new().mount_to_body();
}
