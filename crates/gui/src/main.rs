#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cell::RefCell, rc::Rc};

use iui::{
    controls::{Entry, Label, VerticalBox},
    prelude::*,
};
use phonetics_to_hangul::{darpabet, ipa_to_hangul};
use LayoutStrategy::Compact;

struct State {
    dictionary: darpabet::Dictionary<'static>,
    ui: UI,
    hangul: Entry,
    pronunciation: Entry,
    word: String,
}

impl State {
    fn recalc(&mut self) {
        let mut pronunciations = String::new();
        let mut hanguls = String::new();

        for (i, word) in self.word.split_whitespace().enumerate() {
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

        self.pronunciation.set_value(&self.ui, &pronunciations);
        self.hangul.set_value(&self.ui, &hanguls);
    }
}

fn main() {
    let dictionary = darpabet::Dictionary::parse(darpabet::CMUDICT_07B).unwrap();

    let ui = UI::init().unwrap();
    let mut win = Window::new(&ui, "Phonetics to 한글", 300, 100, WindowType::NoMenubar);

    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let hangul = Entry::new(&ui);
    let pronunciation = Entry::new(&ui);

    let state = Rc::new(RefCell::new(State {
        dictionary,
        ui: ui.clone(),
        hangul: hangul.clone(),
        pronunciation: pronunciation.clone(),
        word: String::new(),
    }));

    let mut word = Entry::new(&ui);
    word.on_changed(&ui, move |word| {
        let mut state = state.borrow_mut();
        state.word = word;
        state.recalc();
    });

    vbox.append(&ui, Label::new(&ui, "Word:"), Compact);
    vbox.append(&ui, word, Compact);

    vbox.append(&ui, Label::new(&ui, "Pronunciation:"), Compact);
    vbox.append(&ui, pronunciation, Compact);

    vbox.append(&ui, Label::new(&ui, "한글:"), Compact);
    vbox.append(&ui, hangul, Compact);

    win.set_child(&ui, vbox);
    win.show(&ui);
    ui.main();
}
