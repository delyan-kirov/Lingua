#[cfg(test)]
mod test {
    use crate::{Category, Etymology, Word};

    use rusqlite::{Connection, Result};
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

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

    #[test]
    fn serialize_word() -> Result<(), Box<dyn std::error::Error>> {
        let word_file = Path::new("./tests/jo.yaml");
        let mut file = File::open(word_file)?;

        // Read the contents of the file into a String
        let mut word_yaml = String::new();
        file.read_to_string(&mut word_yaml)?;

        // Deserialize the YAML string into a Vec<Word>
        let words: Vec<Word> = serde_yaml::from_str(&word_yaml)?;

        // Print the deserialized words
        // for word in &words {
        //     println!("{:}", word);
        // }

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

    #[test]
    fn test3() -> Result<(), Box<dyn std::error::Error>> {
        let word_file = Path::new("./tests/multiple_words.yaml");
        let mut file = File::open(word_file)?;

        // Read the contents of the file into a String
        let mut word_yaml = String::new();
        file.read_to_string(&mut word_yaml)?;

        // Deserialize the YAML string into a Vec<Word>
        let words: Vec<Word> = serde_yaml::from_str(&word_yaml)?;

        println!("{}", words[1]);

        assert_eq!(2, words.len());

        Ok(())
    }

    // TODO: How to make something happen for the first time when running an app

    #[test]
    /// Initializing the sqlite3 database
    fn init_db() -> Result<()> {
        let db = Path::new("./tests/example.db");
        let conn = Connection::open(&db)?;

        conn.execute(
            "
             CREATE TABLE IF NOT EXISTS category (
                categories TEXT PRIMARY KEY
             )
             ",
            (),
        )?;

        conn.execute("PRAGMA foreign_keys = ON;", ())?;

        conn.execute(
            "
             INSERT OR IGNORE INTO category (categories) VALUES
             ('verb'),
             ('noun'),
             ('article'),
             ('pronoun'),
             ('preposition'),
             ('adverb'),
             ('conjunction'),
             ('participle')
             ",
            (),
        )?;

        conn.execute(
            "
             CREATE TABLE IF NOT EXISTS word (
                id INTEGER PRIMARY KEY,
                spelling TEXT NOT NULL,
                categories TEXT NOT NULL,
                definitions TEXT NOT NULL,
                etymology TEXT NOT NULL,
                phonology TEXT,
                synonyms TEXT,
                usage TEXT,
                derivatives TEXT,
                forms TEXT,
                FOREIGN KEY (categories) REFERENCES category(categories)
             )
                 ",
            (),
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

    #[test]
    fn sql_schema_to_rust_vec_struct() {
        todo!()
    }
}

// TODO: Fix some of the tests
// TODO: Add more tests
