#![macro_use]

use colored::Colorize;

use crate::tagger::Tags;
use crate::tagger::{Tag, TaggedContent, UntaggedContent};
use crate::traits::PrettyDisplay;
use std::fmt;

#[derive(Debug, Clone)]
/// Beginning Inside Outside End Single tagging format
pub struct BIOES {
    tags: Vec<BIOESTag>,
}

/// Tags of the BIOES format
#[derive(Debug, Clone)]
pub enum BIOESTag {
    Beginning(TaggedContent),
    Inside(TaggedContent),
    Outside(UntaggedContent),
    End(TaggedContent),
    Single(TaggedContent),
}

/// Creates a BIOES beginning tag
#[macro_export]
macro_rules! bioes_b {
    ($text:expr, [$start:expr => $end:expr], $class:expr) => {
        BIOESTag::Beginning(TaggedContent::new($text, $start, $end, $class))
    };

    ($tagged_content:expr) => {
        BIOESTag::Beginning($tagged_content)
    };
}

/// Creates a BIOES inside tag
#[macro_export]
macro_rules! bioes_i {
    ($text:expr, [$start:expr => $end:expr], $class:expr) => {
        BIOESTag::Inside(TaggedContent::new($text, $start, $end, $class))
    };

    ($tagged_content:expr) => {
        BIOESTag::Inside($tagged_content)
    };
}

/// Creates a BIOES outside tag
#[macro_export]
macro_rules! bioes_o {
    ($text:expr, [$start:expr => $end:expr]) => {
        BIOESTag::Outside(UntaggedContent::new($text, $start, $end))
    };

    ($untagged_content:expr) => {
        BIOESTag::Outside($untagged_content)
    };
}

/// Creates a BIOES end tag
#[macro_export]
macro_rules! bioes_e {
    ($text:expr, [$start:expr => $end:expr], $class:expr) => {
        BIOESTag::End(TaggedContent::new($text, $start, $end, $class))
    };

    ($tagged_content:expr) => {
        BIOESTag::End($tagged_content)
    };
}

/// Creates a BIOES single tag
#[macro_export]
macro_rules! bioes_s {
    ($text:expr, [$start:expr => $end:expr], $class:expr) => {
        BIOESTag::Single(TaggedContent::new($text, $start, $end, $class))
    };

    ($tagged_content:expr) => {
        BIOESTag::Single($tagged_content)
    };
}

impl<'a> PrettyDisplay for BIOESTag {
    fn pretty_display(&self) -> String {
        match self {
            Self::Beginning(tag) => format!(
                "{:<55} {:<} {}{}",
                tag.original_text.trim().bold(),
                " ▍".green(),
                "B-".green().bold(),
                tag.class.green()
            ),
            Self::Inside(tag) => format!(
                "{:<55} {:<} {}{}",
                tag.original_text.trim().bold(),
                " ▍".cyan(),
                "I-".cyan().bold(),
                tag.class.cyan().bold()
            ),
            Self::Outside(tag) => format!(
                "{:<55} {:<} {}",
                tag.original_text.trim(),
                " ▍".dimmed(),
                "O".dimmed()
            ),
            Self::End(tag) => format!(
                "{:<55} {:<} {}{}",
                tag.original_text.trim().bold(),
                " ▍".blue(),
                "E-".blue().bold(),
                tag.class.blue().bold()
            ),
            Self::Single(tag) => format!(
                "{:<55} {:<} {}{}",
                tag.original_text.trim().bold(),
                " ▍".purple(),
                "S-".purple().bold(),
                tag.class.purple().bold()
            ),
        }
    }
}

impl<'a> fmt::Display for BIOESTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Beginning(tag) => write!(f, "{} {}{}", tag.original_text.trim(), "B-", tag.class),
            Self::Inside(tag) => write!(f, "{} {}{}", tag.original_text.trim(), "I-", tag.class),
            Self::Outside(tag) => write!(f, "{} {}", tag.original_text.trim(), "O".dimmed()),
            Self::End(tag) => write!(f, "{} {}{}", tag.original_text.trim(), "E-", tag.class),
            Self::Single(tag) => write!(f, "{} {}{}", tag.original_text.trim(), "S-", tag.class),
        }
    }
}

impl<'a> From<Tags> for BIOES {
    fn from(tags: Tags) -> Self {
        let mut tgs: Vec<BIOESTag> = vec![];

        for tag in tags.0 {
            match tag {
                Tag::UnTagged(t) => tgs.append(&mut t.into()),
                Tag::Tagged(t) => tgs.append(&mut t.into()),
            }
        }

        BIOES { tags: tgs }
    }
}

impl<'a> fmt::Display for BIOES {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let results = self
            .tags
            .iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>();
        write!(f, "{}", results.join("\n"))
    }
}

impl<'a> PrettyDisplay for BIOES {
    fn pretty_display(&self) -> String {
        let results = self
            .tags
            .iter()
            .map(|t| format!("{}", t.pretty_display()))
            .collect::<Vec<_>>();
        format!("{}", results.join("\n"))
    }
}

impl From<TaggedContent> for Vec<BIOESTag> {
    fn from(tag: TaggedContent) -> Self {
        let mut sub_tags = tag
            .original_text
            .split_terminator(|c: char| c.is_ascii_whitespace())
            .collect::<Vec<&str>>()
            .into_iter()
            .peekable();

        let mut bioes_tags = vec![];
        let mut last_pos = tag.start;

        // Beginning tag
        while let Some(sub_tag) = sub_tags.next() {
            if !sub_tag.is_empty() {
                if sub_tags.peek().is_some() {
                    bioes_tags.push(bioes_b![sub_tag, [last_pos => sub_tag.len()], &tag.class])
                } else {
                    bioes_tags.push(bioes_s![sub_tag, [last_pos => sub_tags.len()], &tag.class])
                }

                last_pos += sub_tag.len();
                break;
            }

            last_pos += 1;
        }

        // Inside/End Tag
        while let Some(sub_tag) = sub_tags.next() {
            if !sub_tag.is_empty() {
                if sub_tags.peek().is_some() {
                    bioes_tags.push(bioes_i![sub_tag, [last_pos => sub_tag.len()], &tag.class])
                } else {
                    bioes_tags.push(bioes_e![sub_tag, [last_pos => sub_tag.len()], &tag.class])
                }

                last_pos += sub_tag.len();
            } else {
                last_pos += 1;
            }
        }

        bioes_tags
    }
}

impl From<UntaggedContent> for Vec<BIOESTag> {
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
                bioes_tags.push(bioes_o![sub_tag, [last_pos => sub_tag.len()]]);

                last_pos += sub_tag.len();
            } else {
                last_pos += 1;
            }
        }

        bioes_tags
    }
}
