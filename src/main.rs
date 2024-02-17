mod test;

use rusqlite::{Connection, Rows};
use serde::{Deserialize, Serialize};
use std::fmt;

use std::fs::File;
use std::io::{self};
use std::path::Path;
use std::str::FromStr;

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
        // SPELLING
        write!(f, "{} :: ", self.spelling)?;

        // CATEGORY
        write!(f, "{}\n", self.category)?;
        for (i, def) in self.definition.iter().enumerate() {
            write!(f, "{}. {}\n", i, def)?;
        }

        // PHONOLOGY
        if let Some(phonology) = &self.phonology {
            write!(f, "\nPronunciation :: {}\n", phonology)?;
        }
        // ETYMOLOGY
        write!(f, "\n{}\n", self.etymology)?;

        // USAGE
        if let Some(usage) = &self.usage {
            write!(f, "Usage\n{}\n", usage)?;
        }

        // SYNONYMS
        if let Some(synonyms) = &self.synonyms {
            write!(f, "Synonyms\n")?;
            for (i, synonym) in synonyms.iter().enumerate() {
                write!(f, "{}. {}\n", i, synonym)?;
            }
        }

        // DERIVATIVES
        if let Some(derivatives) = &self.derivatives {
            write!(f, "Derivatives\n")?;
            for (i, derivative) in derivatives.iter().enumerate() {
                write!(f, "{}. {}\n", i, derivative)?;
            }
        }

        // FORMS
        if let Some(forms) = &self.forms {
            write!(f, "\nForms: ")?;
            for form in forms.iter() {
                write!(f, "| {} ", form)?;
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}

enum InputErr {
    Empty,
    Invalid,
}

impl fmt::Display for InputErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputErr::Invalid => write!(f, "Invalid"),
            InputErr::Empty => write!(f, "Invalid"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
impl FromStr for Category {
    type Err = InputErr;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "verb" => Ok(Category::Verb),
            "preposition" => Ok(Category::Preposition),
            "pronoun" => Ok(Category::Pronoun),
            "conjunction" => Ok(Category::Conjunction),
            "adverb" => Ok(Category::Adverb),
            "noun" => Ok(Category::Noun),
            "article" => Ok(Category::Article),
            "participle" => Ok(Category::Participle),
            "" => Err(InputErr::Empty),
            _ => Err(InputErr::Invalid),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Etymology {
    source: String,
    description: String,
}
impl Default for Etymology {
    fn default() -> Self {
        Etymology {
            source: "".to_string(),
            description: "".to_string(),
        }
    }
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

//////////////////////////////////////////////////////////////////

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
    todo!()
}

fn cli_word_gen() -> Word {
    let word_spelling: String = {
        let mut spelling = String::new();
        println!("Provide the spelling of the word");
        io::stdin()
            .read_line(&mut spelling)
            .expect("Failed to read input");
        spelling.trim().to_string().to_lowercase()
    };

    let word_cat = {
        println!("Provide the lexical category of the word");
        let mut _word_cat = Category::Verb;
        loop {
            let mut word_cat_str = String::new();
            io::stdin()
                .read_line(&mut word_cat_str)
                .expect("Failed to read input");

            let word_cat_str = word_cat_str.trim().to_string().to_lowercase();

            match Category::from_str(&word_cat_str) {
                Ok(cat) => {
                    _word_cat = cat;
                    break;
                }
                Err(InputErr::Empty) => {
                    println!("The provided input is empty. Try again");
                    continue;
                }
                Err(InputErr::Invalid) => {
                    println!("The provided input is not valid");
                    println!("The category of a word must be one of: ...");
                    continue;
                }
            }
        }
        _word_cat
    };

    let word_etym: Etymology = {
        let mut etymology = Etymology::default();
        println!("Provide etymology");

        loop {
            println!("Provide the source of the word");
            let mut word_source_str = String::new();
            io::stdin()
                .read_line(&mut word_source_str)
                .expect("Failed to read input");
            let word_source_str = word_source_str.trim().to_lowercase();
            if word_source_str.is_empty() {
                println!("The etymological source is empty which is not allowed.");
                println!("Provide a language source");
                continue;
            }

            println!("Provide description");
            let mut word_description_str = String::new();
            io::stdin()
                .read_line(&mut word_description_str)
                .expect("Failed to read input");
            let word_description_str = word_description_str.trim().to_lowercase();
            if word_description_str.is_empty() {
                println!("The etymological description is empty which is not allowed");
                println!("Please provide a description");
                continue;
            }

            // Assign values to the etymology struct
            etymology.source = word_source_str.to_string();
            etymology.description = word_description_str.to_string();
            break;
        }
        etymology
    };

    let word_def: Vec<String> = {
        let mut word_definitions = vec![];
        let mut word_def_str = String::new();
        println!("Please provide the definition of the word");
        println!("More than one definition can be given");

        loop {
            io::stdin()
                .read_line(&mut word_def_str)
                .expect("Failed to read input");
            let word_def_str = word_def_str.trim().to_lowercase();
            if word_def_str.is_empty() {
                println!(
                    "The definition is empty which is not allowed. Please provide a definition"
                );
                continue;
            } else {
                word_definitions.push(word_def_str.to_string());
                println!("Would you like to provide more definitions? Yes/No");
                let mut user_opt = String::new();
                io::stdin()
                    .read_line(&mut user_opt)
                    .expect("Failed to read input");
                let user_opt = user_opt.trim();
                match user_opt {
                    "No" | "no" => break,
                    "Yes" | "yes" => continue,
                    _ => println!("Your input is invalid. Type enter to try continue"),
                }
            }
        }
        word_definitions
    };

    let word_phon: Option<String> = {
        println!("Provide the pronuntiation if needed or press enter to continue");
        let mut phonology = String::new();
        io::stdin()
            .read_line(&mut phonology)
            .expect("Failed to read input");
        let phonology = phonology.trim();
        match phonology {
            "" => None,
            a => Some(a.to_string()),
        }
    };

    let word_usage: Option<String> = {
        println!("Provide the usage if needed or press enter to continue");
        let mut usage = String::new();
        io::stdin()
            .read_line(&mut usage)
            .expect("Failed to read input");
        let usage = usage.trim().to_lowercase();
        match usage.as_str() {
            "" => None,
            a => Some(a.to_string()),
        }
    };

    let word_derivs: Option<Vec<String>> = {
        let mut derivatives: Vec<String> = vec![];
        loop {
            println!("Provide the derivative forms or press enter to continue");
            let mut deriv_word = String::new();
            io::stdin()
                .read_line(&mut deriv_word)
                .expect("Failed to read input");
            let deriv_word = deriv_word.trim().to_lowercase();
            if deriv_word == "" {
                break;
            }
            derivatives.push(deriv_word)
        }
        if derivatives.is_empty() {
            None
        } else {
            Some(derivatives)
        }
    };

    let word_syn: Option<Vec<String>> = {
        let mut synonyms: Vec<String> = vec![];
        loop {
            println!("Provide synonyms or press enter to continue");
            let mut syn_word = String::new();
            io::stdin()
                .read_line(&mut syn_word)
                .expect("Failed to read input");
            let syn_word = syn_word.trim().to_lowercase();
            if syn_word == "" {
                break;
            }
            synonyms.push(syn_word)
        }
        if synonyms.is_empty() {
            None
        } else {
            Some(synonyms)
        }
    };

    let word_forms: Option<Vec<String>> = {
        let mut forms: Vec<String> = vec![];
        loop {
            println!("Provide other forms of the word or press enter to continue");
            let mut form_word = String::new();
            io::stdin()
                .read_line(&mut form_word)
                .expect("Failed to read input");
            let form_word = form_word.trim().to_lowercase();
            if form_word == "" {
                break;
            }
            forms.push(form_word)
        }
        if forms.is_empty() {
            None
        } else {
            Some(forms)
        }
    };

    Word {
        spelling: word_spelling,
        category: word_cat,
        phonology: word_phon,
        definition: word_def,
        etymology: word_etym,
        synonyms: word_syn,
        usage: word_usage,
        derivatives: word_derivs,
        forms: word_forms,
    }
}

fn word_to_db(word: &Word) -> Result<(), Box<dyn std::error::Error>> {
    let db = Path::new("./tests/example.db");
    let conn = Connection::open(&db)?;

    conn.execute(
        "
    INSERT INTO word (
        spelling,
        categories,
        definitions,
        etymology,
        phonology,
        synonyms,
        usage,
        derivatives,
        forms
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    ",
        &[
            &word.spelling as &dyn rusqlite::types::ToSql,
            &word.category.to_string() as &dyn rusqlite::types::ToSql,
            &serde_yaml::to_string(&word.definition)? as &dyn rusqlite::types::ToSql,
            &serde_yaml::to_string(&word.etymology)? as &dyn rusqlite::types::ToSql,
            &word.phonology as &dyn rusqlite::types::ToSql,
            &word
                .synonyms
                .as_ref()
                .map(|syns| serde_yaml::to_string(syns))
                .transpose()? as &dyn rusqlite::types::ToSql,
            &word.usage as &dyn rusqlite::types::ToSql,
            &word
                .derivatives
                .as_ref()
                .map(|derivs| serde_yaml::to_string(derivs))
                .transpose()? as &dyn rusqlite::types::ToSql,
            &word
                .forms
                .as_ref()
                .map(|forms| serde_yaml::to_string(forms))
                .transpose()? as &dyn rusqlite::types::ToSql,
        ],
    )?;

    match conn.close() {
        Ok(()) => {
            println!("Connection to sqlite closed successfully")
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    Result::Ok(())
}

#[allow(warnings)]
/// Take a sql row as iterator and create a word struc
fn sql_to_word(mut rows: Rows) -> Result<Word, Box<dyn std::error::Error>> {
    // CREATE TABLE IF NOT EXISTS word (
    //    id INTEGER PRIMARY KEY,
    //    spelling TEXT NOT NULL,
    //    categories TEXT NOT NULL,
    //    definitions TEXT NOT NULL,
    //    etymology TEXT NOT NULL,
    //    phonology TEXT,
    //    synonyms TEXT,
    //    usage TEXT,
    //    derivatives TEXT,
    //    forms TEXT,
    //    FOREIGN KEY (categories) REFERENCES category(categories)

    ////

    // if let Some(row) = rows.next() {
    //     let row = row?;
    //     let spelling = row.get(1);
    //     let category = row.get(2);
    //
    //     todo!()
    // }
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Path::new("./tests/example.db");
    let conn = Connection::open(&db)?;

    let new_word = cli_word_gen();
    println!("{}", new_word);

    word_to_db(&new_word)?;

    match conn.close() {
        Ok(()) => {
            println!("Connection to sqlite closed successfully")
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    Result::Ok(())
}

// TODO: Write better tests
// TODO: Separate the test module
