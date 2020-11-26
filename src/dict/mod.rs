use crate::types::*;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

/// Represents a dictionary entry. A dictionary entry has a term and it's class.
#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    term: Term,
    class: Class,
}

impl DictionaryEntry {
    /// Returns the term of a dictionary entry
    pub fn term(&self) -> &String {
        &self.term
    }

    /// Returns the class of a dictionary entry
    pub fn class(&self) -> &String {
        &self.class
    }
}

/// A struct used to build Dictionary
#[derive(Debug, Clone)]
pub struct DictionaryBuilder {
    entries: Entries,
}

impl DictionaryBuilder {
    pub fn from_files<P: Into<PathBuf> + AsRef<OsStr>>(
        paths: &[P],
    ) -> Result<DictionaryBuilder, Box<dyn Error>> {
        let mut dictionaries = Vec::new();
        let paths: Vec<PathBuf> = paths.into_iter().map(|p| p.into()).collect();
        for file in paths {
            dictionaries.push(DictionaryBuilder::from_file(&file)?.build());
        }

        let entries = dictionaries
            .into_iter()
            .flat_map(|dictionary| dictionary.entries().clone())
            .collect::<Entries>();
        Ok(DictionaryBuilder { entries: entries })
    }
    /// Create a dictionnary from a TSV file (term, class)
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<DictionaryBuilder, Box<dyn Error>> {
        let path = path.into();
        info!("Loading dictionary from file {:?}", path.display());

        let dict_file = File::open(path)?;
        let reader = BufReader::new(dict_file);

        let terms = reader
            .lines()
            .into_iter()
            .flat_map(|line| line)
            .map(|line| {
                let entry_elements = line.splitn(2, "\t").collect::<Vec<&str>>();

                match (entry_elements.get(0), entry_elements.get(1)) {
                    (Some(term), Some(category)) if term.len() > 7 => Some(DictionaryEntry {
                        term: term.to_string(),
                        class: category.to_string(),
                    }),
                    _ => None,
                }
            })
            .flat_map(|entry| entry)
            .collect::<Vec<DictionaryEntry>>();

        info!("Dictionary loaded");

        Ok(DictionaryBuilder { entries: terms })
    }

    pub fn build(self) -> Dictionary {
        Dictionary {
            entries: self.entries,
        }
    }
}

/// A struct representing a dictionary. A dictionnary is composed of DictionaryEntries
#[derive(Debug, Clone)]
pub struct Dictionary {
    entries: Entries,
}

impl Dictionary {
    /// Returns all entries of a dictionary
    fn entries(&self) -> &Vec<DictionaryEntry> {
        &self.entries
    }

    /// Returns all terms of the dictionary
    pub fn terms(&self) -> Terms {
        let terms = self.entries().iter().map(|e| e.term()).collect::<Terms>();

        terms
    }

    /// Returns uniq classes of the dictionary
    #[allow(dead_code)]
    fn classes(&self) -> Classes {
        let mut classes = self
            .entries()
            .iter()
            .map(|e| e.class())
            .collect::<Classes>();

        classes.sort();
        classes.dedup();

        classes
    }

    /// Get the class of a term by it's index in the dictionary
    pub fn get_class(&self, index: usize) -> String {
        self.entries[index].class().to_string()
    }
}
