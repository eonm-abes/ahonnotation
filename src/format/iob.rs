use crate::tagger::Tags;
use crate::tagger::{Tag, TaggedContent, UntaggedContent};
use crate::traits::PrettyDisplay;
use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone)]
/// Inside Outside Beginning tagging format
pub struct IOB {
    tags: Vec<IOBTag>,
}

/// Tags of the IOB format
#[derive(Debug, Clone)]
pub enum IOBTag {
    Inside(TaggedContent),
    Outside(UntaggedContent),
    Beginning(TaggedContent),
}

impl<'a> PrettyDisplay for IOBTag {
    fn pretty_display(&self) -> String {
        match self {
            Self::Beginning(tag) => format!(
                "{:<55} {:<} {}{}",
                tag.original_text.trim().bold(),
                " ▍".red(),
                "B-".red().bold(),
                tag.class.red()
            ),
            Self::Inside(tag) => format!(
                "{:<55} {:<} {}{}",
                tag.original_text.trim().bold(),
                " ▍".yellow(),
                "I-".yellow().bold(),
                tag.class.yellow().bold()
            ),
            Self::Outside(tag) => format!(
                "{:<55} {:<} {}",
                tag.original_text.trim(),
                " ▍".dimmed(),
                "O".dimmed()
            ),
        }
    }
}

/// Creates a IOB inside tag
#[macro_export]
macro_rules! iob_i {
    ($text:expr, [$start:expr => $end:expr], $class:expr) => {
        IOBTag::Inside(TaggedContent::new($text, $start, $end, $class))
    };

    ($tagged_content:expr) => {
        IOBTag::Inside($tagged_content)
    };
}

/// Creates a IOB outside tag
#[macro_export]
macro_rules! iob_o {
    ($text:expr, [$start:expr => $end:expr]) => {
        IOBTag::Outside(UntaggedContent::new($text, $start, $end))
    };

    ($untagged_content:expr) => {
        IOBTag::Outside($untagged_content)
    };
}

/// Creates a IOB beginning tag
#[macro_export]
macro_rules! iob_b {
    ($text:expr, [$start:expr => $end:expr], $class:expr) => {
        IOBTag::Beginning(TaggedContent::new($text, $start, $end, $class))
    };

    ($tagged_content:expr) => {
        IOBTag::Outside($tagged_content)
    };
}

impl<'a> From<Tags> for IOB {
    fn from(tags: Tags) -> Self {
        let mut tgs: Vec<IOBTag> = vec![];

        for tag in tags.0 {
            match tag {
                Tag::Tagged(t) => tgs.append(&mut t.into()),
                Tag::UnTagged(t) => tgs.append(&mut t.into()),
            }
        }

        IOB { tags: tgs }
    }
}

impl<'a> fmt::Display for IOBTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inside(tag) => write!(f, "{} I-{}", tag.original_text.trim(), tag.class),
            Self::Outside(tag) => write!(f, "{} O", tag.original_text.trim()),
            Self::Beginning(tag) => write!(f, "{} B-{}", tag.original_text.trim(), tag.class),
        }
    }
}

impl<'a> fmt::Display for IOB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let results = self
            .tags
            .iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>();
        write!(f, "{}", results.join("\n"))
    }
}

impl<'a> PrettyDisplay for IOB {
    fn pretty_display(&self) -> String {
        let results = self
            .tags
            .iter()
            .map(|t| format!("{}", t.pretty_display()))
            .collect::<Vec<_>>();
        format!("{}", results.join("\n"))
    }
}

impl From<TaggedContent> for Vec<IOBTag> {
    fn from(tag: TaggedContent) -> Self {
        let mut sub_tags = tag
            .original_text
            .split_terminator(|c: char| c.is_ascii_whitespace())
            .collect::<Vec<&str>>()
            .into_iter();

        let mut iob_tags = vec![];
        let mut last_pos = tag.start;

        // Beginning tag
        while let Some(sub_tag) = sub_tags.next() {
            if !sub_tag.is_empty() {
                iob_tags.push(iob_b![sub_tag, [last_pos => sub_tag.len()], &tag.class]);

                last_pos += sub_tag.len();
                break;
            }

            last_pos += 1;
        }

        // Inside Tag
        while let Some(sub_tag) = sub_tags.next() {
            if !sub_tag.is_empty() {
                iob_tags.push(iob_i![sub_tag, [last_pos => sub_tag.len()], &tag.class]);

                last_pos += sub_tag.len();
            } else {
                last_pos += 1;
            }
        }

        iob_tags
    }
}

impl From<UntaggedContent> for Vec<IOBTag> {
    fn from(tag: UntaggedContent) -> Self {
        let mut sub_tags = tag
            .original_text
            .split_terminator(|c: char| c.is_ascii_whitespace())
            .collect::<Vec<&str>>()
            .into_iter()
            .peekable();

        let mut bioes_tags = vec![];
        let mut last_pos = tag.start;

        // Inside/End Tag
        while let Some(sub_tag) = sub_tags.next() {
            if !sub_tag.is_empty() {
                bioes_tags.push(iob_o![sub_tag, [last_pos => sub_tag.len()]]);

                last_pos += sub_tag.len();
            } else {
                last_pos += 1;
            }
        }

        bioes_tags
    }
}
