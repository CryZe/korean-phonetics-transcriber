use anyhow::{Context, Result};
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
#[serde(untagged)]
enum ResponseResult<T> {
    Ok(T),
    Err(ErrorResponse),
}

#[derive(Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

#[derive(Deserialize)]
struct SearchResult {
    id: String,
}

#[derive(Deserialize)]
struct Entry {
    headword: HeadWordList,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum HeadWordList {
    Single(HeadWord),
    Multiple(Vec<HeadWord>),
}

#[derive(Deserialize)]
struct HeadWord {
    text: String,
    pronunciation: Pronunciation,
}

#[derive(Deserialize)]
struct Pronunciation {
    value: String,
}

pub struct Word {
    pub word: String,
    pub pronunciation: String,
}

pub struct Client {
    inner: reqwest::blocking::Client,
    user: String,
    password: String,
}

impl Client {
    pub fn new(user: String, password: String) -> Self {
        Self {
            inner: reqwest::blocking::Client::new(),
            user,
            password,
        }
    }

    fn call_api<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        let result: ResponseResult<T> = self
            .inner
            .get(url)
            .basic_auth(&self.user, Some(&self.password))
            .send()
            .context("Failed accessing the dictionary API.")?
            .json()
            .context("Failed parsing the response from the dictionary API.")?;

        match result {
            ResponseResult::Ok(t) => Ok(t),
            ResponseResult::Err(e) => anyhow::bail!("{}", e.message),
        }
    }

    pub fn lookup(&self, word: &str, language: &str) -> Result<Word> {
        let SearchResponse { results } = self
            .call_api(
                Url::parse_with_params(
                    "https://dictapi.lexicala.com/search",
                    &[("language", language), ("text", word)],
                )
                .unwrap(),
            )
            .context("Failed searching the word via the dictionary API.")?;

        let entry: Entry = self
            .call_api(
                Url::parse("https://dictapi.lexicala.com/entries/")
                    .unwrap()
                    .join(
                        &results
                            .get(0)
                            .context("The dictionary does not contain the word.")?
                            .id,
                    )
                    .unwrap(),
            )
            .context("Failed looking up the word's pronunciation via the dictionary API.")?;

        let headword = match entry.headword {
            HeadWordList::Single(s) => s,
            HeadWordList::Multiple(m) => m
                .into_iter()
                .next()
                .context("The word contains a list of words that is empty.")?,
        };

        Ok(Word {
            word: headword.text,
            pronunciation: headword
                .pronunciation
                .value
                .splitn(2, ',')
                .next()
                .unwrap()
                .to_owned(),
        })
    }
}
