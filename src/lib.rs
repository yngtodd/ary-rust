use std::path::Path;
use std::collections::HashMap;

struct Vocab {
    pub map: HashMap<String, i32>,
}

impl Vocab {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Vocab, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string(path)?;
        let tokens = Vocab::tokenize(contents);

        let mut tok = 0;
        for term in &tokens {
            if !map.contains_key(term) {
                map.insert(term.to_lowercase(), tok);
                tok += 1;
            }
        }

        Ok(Vocab {map})
    }

    // Parse the string, stripping punctuation and whitespace
    pub fn tokenize(text: String) -> Vec<String> {
        let tokens: Vec<String> = text.split(|c: char| !(c.is_alphanumeric() || c == '\''))
                                      .filter(|s| !s.is_empty())
                                      .map(|s| s.to_string())
                                      .collect();

        tokens
    }

    // Load the vocabulary from disk
    pub fn _load<P: AsRef<Path>>(path: P) -> Result<Vocab, std::io::Error> {
        let mut map = HashMap::new(); 
        let contents = std::fs::read_to_string(path).expect("File not found!");

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let voc = chunks.next().expect("No vocab term!");
            let tok = chunks.next().expect("No token!").parse().unwrap();
            map.insert(voc.to_owned(), tok);
        }

        Ok(Vocab {map})
    } 

    // Write the vocabulary to disk
    pub fn _write<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, val) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(&val.to_string());
            contents.push('\n');
        }

        std::fs::write(path, contents)
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }

    // Print the contents of the database
    fn show(&self) {
        for (voc, tok) in &self.map {
            println!("  KEY: {}, VALUE: {}", voc, tok);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
