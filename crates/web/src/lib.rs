use phonetics_to_hangul::{darpabet, ipa_to_hangul};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    dictionary: darpabet::Dictionary<'static>,
    word: String,
}

enum Message {
    WordChanged(InputData),
}

impl Component for Model {
    type Message = Message;

    type Properties = ();

    fn create((): (), link: ComponentLink<Self>) -> Self {
        let dictionary = darpabet::Dictionary::parse(darpabet::CMUDICT_07B).unwrap();
        Self {
            link,
            dictionary,
            word: String::new(),
        }
    }

    fn update(&mut self, Message::WordChanged(change): Message) -> ShouldRender {
        self.word = change.value;
        true
    }

    fn change(&mut self, (): ()) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut pronunciations = String::new();
        let mut hanguls = String::new();

        let word = if self.word.is_empty() {
            "Example Text"
        } else {
            &self.word
        };

        for (i, word) in word.split_whitespace().enumerate() {
            if i != 0 {
                pronunciations.push(' ');
                hanguls.push(' ');
            }

            if let Some(pronunciation) = self.dictionary.look_up(word) {
                pronunciations.extend(pronunciation.clone());
                hanguls.push_str(&ipa_to_hangul::convert(pronunciation));
            } else {
                pronunciations.push('?');
                hanguls.push('?');
            }
        }

        html! {
            <div>
                <p>
                    {"English Text: "}
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
                    {"Pronunciation (IPA): "}
                    <div class="result">
                        {&pronunciations}
                    </div>
                </p>
                <p>
                    {"한글: "}
                    <div class="result">
                        {&hanguls}
                    </div>
                </p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
}
