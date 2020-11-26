use crate::dict::Dictionary;
use crate::errors::MissingDictionnary;
use crate::tagger::{Tag, TaggedContent, Tags, UntaggedContent};
use crate::traits::{Find, FindWord};
use crate::types::*;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

pub struct TaggerBuilder<'a> {
    dict: Option<&'a Dictionary>,
    case_sensitive: bool,
    match_kind: MatchKind,
    word_matching: bool,
}

/// Create a new tagger with a given dictionnary
impl<'a> TaggerBuilder<'a> {
    pub fn dictionary(mut self, dict: &'a Dictionary) -> TaggerBuilder<'a> {
        self.dict = Some(dict);
        self
    }

    pub fn case_sensitive(mut self, case_sensitive: bool) -> TaggerBuilder<'a> {
        self.case_sensitive = case_sensitive;
        self
    }

    pub fn match_kind(mut self, match_kind: MatchKind) -> TaggerBuilder<'a> {
        self.match_kind = match_kind;
        self
    }

    pub fn word_matching(mut self, word_matching: bool) -> TaggerBuilder<'a> {
        self.word_matching = word_matching;
        self
    }

    pub fn build(self) -> Result<Tagger<'a>, MissingDictionnary> {
        match self.dict {
            Some(dict) => {
                info!("Building Tagger (AhoCorasick FSA)");

                let ac_fsa = AhoCorasickBuilder::new()
                    .match_kind(self.match_kind)
                    .ascii_case_insensitive(!self.case_sensitive)
                    .build(dict.terms());

                info!("Tagger builded");

                Ok(Tagger {
                    dict: dict,
                    finder: ac_fsa,
                })
            }
            None => Err(MissingDictionnary),
        }
    }
}

impl<'a> Default for TaggerBuilder<'a> {
    fn default() -> Self {
        TaggerBuilder {
            dict: None,
            case_sensitive: false,
            match_kind: MatchKind::LeftmostLongest,
            word_matching: true,
        }
    }
}

/// A struct used to tag text with a dictionary
pub struct Tagger<'a> {
    dict: &'a Dictionary,
    finder: AhoCorasick,
}

impl<'a> Tagger<'a> {
    /// Peform text annotation on a given text
    pub fn tag(&self, text: &'a str) -> Tags {
        let results = self.find_word(text);
        let mut peekable_result = results.iter().peekable();

        let mut tags: Vec<Tag> = vec![];

        let mut lock = false;

        while let Some((start, end, category)) = peekable_result.next() {
            if *start > 0 && lock == false {
                tags.push(tag!(&text[..*start], [0 => *start - 1]).into())
            }

            lock = true;

            tags.push(
                tag![&text[*start..*end], [*start => *end - 1], &self.dict.get_class(*category)],
            );

            if let Some((next_start, _, _)) = peekable_result.peek() {
                tags.push(tag![&text[*end..*next_start], [*end => *next_start - 1]]);
            }
        }

        // If no result is found in the input string the whole string is considered as UntaggedContent
        if lock == false {
            tags.push(tag![text, [0 => text.len()]]);
        }

        Tags(tags)
    }
}

impl<'a> Find for Tagger<'a> {
    fn find<S: Into<String>>(&self, input: S) -> Vec<(StartByte, EndByte, DictionaryIndex)> {
        let finder = &self.finder;
        let input = input.into();

        finder
            .find_iter(&input)
            .map(|result| (result.start(), result.end(), result.pattern()))
            .collect::<Vec<(StartByte, EndByte, DictionaryIndex)>>()
    }
}

impl<'a> FindWord for Tagger<'a> {}
