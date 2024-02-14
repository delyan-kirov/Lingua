use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(warnings)]
use std::fs::File;
use std::io::{self};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Word {
    spelling: String,
    category: Category,
    phonology: Option<String>,
    definition: Vec<String>,
    etymology: Etymology,
    synonyms: Option<Vec<String>>,
    usage: Option<String>,
    derivatives: Option<Vec<String>>,
    forms: Option<Vec<String>>,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.spelling)?;

        write!(f, "{}\n\n", self.category)?;

        for (i, def) in self.definition.iter().enumerate() {
            write!(f, "{}. {}\n", i, def)?;
        }

        if let Some(phonology) = &self.phonology {
            write!(f, "\n{}\n", phonology)?;
        }

        write!(f, "\n{}\n", self.etymology)?;

        if let Some(usage) = &self.usage {
            write!(f, "Usage\n{}\n", usage)?;
        }

        if let Some(synonyms) = &self.synonyms {
            for (i, synonym) in synonyms.iter().enumerate() {
                write!(f, "{}. {}\n", i, synonym)?;
            }
        }

        if let Some(derivatives) = &self.derivatives {
            for (i, derivative) in derivatives.iter().enumerate() {
                write!(f, "{}. {}\n", i, derivative)?;
            }
        }

        if let Some(forms) = &self.forms {
            write!(f, "\nForms: ")?;
            for form in forms.iter() {
                write!(f, "| {}", form)?;
            }
            writeln!(f, " |")?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[allow(warnings)]
enum Category {
    Verb,
    Noun,
    Article,
    Pronoun,
    Preposition,
    Adverb,
    Conjunction,
    Participle,
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Verb => write!(f, "verb"),
            Category::Pronoun => write!(f, "pronoun"),
            Category::Participle => write!(f, "participle"),
            Category::Noun => write!(f, "noun"),
            Category::Conjunction => write!(f, "Conjunction"),
            Category::Adverb => write!(f, "adverb"),
            Category::Article => write!(f, "article"),
            Category::Preposition => write!(f, "preposition"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Etymology {
    source: String,
    description: String,
}
impl fmt::Display for Etymology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Etymology\nSource: {}\n{}",
            self.source, self.description,
        )
    }
}

// Check if word follows the phonological rules
#[allow(warnings)]
fn is_word(word: Word) -> bool {
    todo!()
}

#[allow(warnings)]
fn read_word(dir_path: &Path) -> io::Result<String> {
    // Open the file located at the specified directory path
    let file_path = dir_path.join("file.txt");
    let file = File::open(file_path)?;
    dbg!(file);
    return io::Result::Ok("".to_string());
}

// TODO: Implement CLI

#[cfg(test)]
mod tests {

    use crate::{Category, Etymology, Word};

    #[allow(warnings)]
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    // TODO: Create a Connection to the sqlite database
    // TODO: Write to the sqlite database

    #[test]
    fn test1() {
        let new_word = Word {
            spelling: "jo".to_string(),
            category: Category::Pronoun,
            phonology: None,
            definition: vec!["First person singular pronoun in the nominative case".to_string()],
            usage: None,
            etymology: Etymology {
                source: "Latin".to_string(),
                description: "From Latin ego".to_string(),
            },
            synonyms: None,
            derivatives: None,
            forms: Some(vec![
                "jo".to_string(),
                "me".to_string(),
                "mi".to_string(),
                "mio".to_string(),
            ]),
        };
        println! {"{:}", new_word};
    }

    // TODO: Create a funtion that reads words from files

    #[test]
    fn test2() -> Result<(), Box<dyn std::error::Error>> {
        let word_file = Path::new("./tests/jo.yaml");
        let mut file = File::open(word_file)?;

        // Read the contents of the file into a String
        let mut word_yaml = String::new();
        file.read_to_string(&mut word_yaml)?;

        // Deserialize the YAML string into a Vec<Word>
        let words: Vec<Word> = serde_yaml::from_str(&word_yaml)?;

        // Print the deserialized words
        for word in &words {
            println!("{:}", word);
        }

        let new_word = Word {
            spelling: "jo".to_string(),
            category: Category::Pronoun,
            phonology: None,
            definition: vec!["First person singular pronoun in the nominative case".to_string()],
            usage: None,
            etymology: Etymology {
                source: "Latin".to_string(),
                description: "From Latin ego".to_string(),
            },
            synonyms: None,
            derivatives: None,
            forms: Some(vec![
                "jo".to_string(),
                "me".to_string(),
                "mi".to_string(),
                "mio".to_string(),
            ]),
        };

        assert_eq!(new_word, words[0]);

        Ok(())
    }
}

fn main() -> Result<()> {
    let db = Path::new("./tests/example.db");
    let conn = Connection::open(&db)?;

    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS word (
       id INTEGER PRIMARY KEY
    )
        ",
        (),
    )?;

    Result::Ok(())
}
