struct Noun {
    word: String,
    gender: String,
}

struct Adjective {
    masculine: String,
    feminine: String,
}
const NOUNS: &str = include_str!("nouns.csv");
const ADJECTIVES: &str = include_str!("adjectives.csv");

lazy_static::lazy_static! {
    pub static ref KEY_GENERATOR: KeyGenerator = KeyGenerator::new();
}

pub struct KeyGenerator {
    nouns: Vec<Noun>,
    adjectives: Vec<Adjective>,
}

impl KeyGenerator {
    fn new() -> Self {
        KeyGenerator {
            nouns: NOUNS
                .lines()
                .map(|s: &str| {
                    let parts: Vec<&str> = s.split(',').collect();
                    Noun {
                        word: parts[0].to_string(),
                        gender: parts[1].to_string(),
                    }
                })
                .collect::<Vec<Noun>>(),
            adjectives: ADJECTIVES
                .lines()
                .map(|s: &str| {
                    let parts: Vec<&str> = s.split(',').collect();
                    Adjective {
                        masculine: parts[0].to_string(),
                        feminine: parts[1].to_string(),
                    }
                })
                .collect::<Vec<Adjective>>(),
        }
    }

    pub fn generate_key(&self) -> String {
        let indices = rand::random::<(u8, u8, u8, u8)>();
        let mut key = String::new();

        let noun1 = &self.nouns[indices.0 as usize % self.nouns.len()];
        let adjective1 = &self.adjectives[indices.1 as usize % self.adjectives.len()];
        let noun2 = &self.nouns[indices.2 as usize % self.nouns.len()];
        let adjective2 = &self.adjectives[indices.3 as usize % self.adjectives.len()];

        key.push_str(&noun1.word);
        key.push('-');
        if noun1.gender == "f" {
            key.push_str(&adjective1.feminine);
        } else {
            key.push_str(&adjective1.masculine);
        }
        key.push('-');
        key.push_str(&noun2.word);
        key.push('-');
        if noun2.gender == "f" {
            key.push_str(&adjective2.feminine);
        } else {
            key.push_str(&adjective2.masculine);
        }

        key
    }
}
