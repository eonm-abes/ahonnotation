use crate::dict::DictionaryEntry;

/// Represents the starting byte in a String of a Find result
pub type StartByte = usize;
/// Represents the ending byte in a String of a Find result
pub type EndByte = usize;
/// Represents the dictionary index of a find result. This index contains the class of the result
pub type DictionaryIndex = usize;
/// Represents a set of Entry of a Dictionary
pub type Entries = Vec<DictionaryEntry>;
/// Represents a term in a Dictionary
pub type Term = String;
/// Represents a set of terms in a Dictionary
pub type Terms<'a> = Vec<&'a Term>;
/// Represents a class in a Dictionary
pub type Class = String;
/// Represents a set of classes in a Dictionary
pub type Classes<'a> = Vec<&'a Class>;
/// Represents a FindResult
pub type FindResult = (StartByte, EndByte, DictionaryIndex);
