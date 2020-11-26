use crate::tagger::Tag;
use crate::types::*;

pub trait Annotate {
    fn annotate(&self) -> Vec<Tag>;
}

/// Find patterns in a string and return their positions in bytes
pub trait Find {
    fn find<S: Into<String>>(&self, input: S) -> Vec<FindResult>;
}

/// Find words in a string and return their positions in bytes
pub trait FindWord: Find {
    fn find_word<S: Into<String>>(&self, input: S) -> Vec<FindResult> {
        let input = input.into();

        let results = self.find(&input);

        results
            .into_iter()
            .filter(|(start, end, _)| {
                let previous_char = self.get_previous_char(*start, &input);
                let next_char = self.get_next_char(*end, &input);

                match (previous_char, next_char) {
                    (None, Some(next_char)) => {
                        if self.is_word_boundary(next_char) {
                            return true;
                        }
                    }
                    (Some(previous_char), None) => {
                        if self.is_word_boundary(previous_char) {
                            return true;
                        }
                    }
                    (Some(previous_char), Some(next_char)) => {
                        if self.is_word_boundary(previous_char) && self.is_word_boundary(next_char)
                        {
                            return true;
                        }
                    }
                    (None, None) => return true,
                };

                return false;
            })
            .collect::<Vec<(StartByte, EndByte, DictionaryIndex)>>()
    }

    fn is_word_boundary(&self, input: char) -> bool {
        input.is_whitespace() | input.is_ascii_punctuation()
    }

    fn get_previous_char(&self, index: usize, input: &str) -> Option<char> {
        let mut mindex = index;
        if index < 1 {
            None
        } else {
            while !input.is_char_boundary(mindex) {
                if mindex > 0 {
                    mindex -= 1
                } else {
                    return None;
                }
            }

            let chars = input[mindex..index].chars().collect::<Vec<char>>();

            chars.last().map(|c| c.clone())
        }
    }

    fn get_next_char(&self, index: usize, input: &str) -> Option<char> {
        let mut mindex = index;
        if index >= input.len() {
            None
        } else {
            while !input.is_char_boundary(mindex) {
                if mindex > 0 {
                    mindex += 1
                } else {
                    return None;
                }
            }

            let chars = input[mindex..index].chars().collect::<Vec<char>>();

            chars.first().map(|c| c.clone())
        }
    }
}

pub trait PrettyDisplay {
    fn pretty_display(&self) -> String;
}
