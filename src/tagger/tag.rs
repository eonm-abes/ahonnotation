use crate::types::*;

#[derive(Debug, Clone)]
/// Represents a set of tag
pub struct Tags(pub Vec<Tag>);

#[macro_export]
macro_rules! tag {
    ($text:expr, [$start:expr => $end:expr], $class:expr) => {
        Tag::Tagged(TaggedContent::new($text, $start, $end, $class))
    };

    ($text:expr, [$start:expr => $end:expr]) => {
        Tag::UnTagged(UntaggedContent::new($text, $start, $end))
    };
}

#[derive(Debug, Clone)]
pub enum Tag {
    Tagged(TaggedContent),
    UnTagged(UntaggedContent),
}

#[derive(Debug, Clone)]
/// Represents a tagged piece of text
pub struct TaggedContent {
    pub original_text: Term,
    pub start: StartByte,
    pub end: EndByte,
    pub class: String,
}

impl<'a> TaggedContent {
    pub fn new<S: Into<String>>(
        original_text: S,
        start: StartByte,
        end: EndByte,
        class: S,
    ) -> TaggedContent {
        TaggedContent {
            original_text: original_text.into(),
            start,
            end,
            class: class.into(),
        }
    }
}

/// Transforms a TaggedContent into a Tag
impl<'a> From<TaggedContent> for Tag {
    fn from(tagged_content: TaggedContent) -> Self {
        Tag::Tagged(tagged_content)
    }
}

/// Transforms a UntaggedContent into a Tag
impl<'a> From<UntaggedContent> for Tag {
    fn from(tagged_content: UntaggedContent) -> Self {
        Tag::UnTagged(tagged_content)
    }
}

#[derive(Debug, Clone)]
/// Represents an untagged piece of text
pub struct UntaggedContent {
    pub original_text: Term,
    pub start: StartByte,
    pub end: EndByte,
}

impl<'a> UntaggedContent {
    pub fn new<S: Into<Term>>(original_text: S, start: usize, end: usize) -> UntaggedContent {
        UntaggedContent {
            original_text: original_text.into(),
            start,
            end,
        }
    }
}
