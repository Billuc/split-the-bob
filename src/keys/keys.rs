#[derive(Debug)]
struct Noun {
    word: String,
    gender: String,
}

#[derive(Debug)]
struct Adjective {
    masculine: String,
    feminine: String,
}
const NOUNS: &str = include_str!("nouns.csv");
const ADJECTIVES: &str = include_str!("adjectives.csv");
const NUMBER_OF_PAIRS: usize = 2;
const NUMBER_OF_WORDS: usize = 512;

lazy_static::lazy_static! {
    pub static ref KEY_GENERATOR: KeyGenerator = KeyGenerator::new();
}

pub struct KeyGenerator {
    nouns: [Noun; NUMBER_OF_WORDS],
    adjectives: [Adjective; NUMBER_OF_WORDS],
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
                .collect::<Vec<Noun>>()
                .try_into()
                .unwrap(),
            adjectives: ADJECTIVES
                .lines()
                .map(|s: &str| {
                    let parts: Vec<&str> = s.split(',').collect();
                    Adjective {
                        masculine: parts[0].to_string(),
                        feminine: parts[1].to_string(),
                    }
                })
                .collect::<Vec<Adjective>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn generate_key(&self) -> String {
        let mut indices_iter = rand::random_iter::<u16>();
        let mut key = String::new();

        for _ in 0..NUMBER_OF_PAIRS {
            let noun_index = indices_iter.next().unwrap() as usize % NUMBER_OF_WORDS;
            let adjective_index = indices_iter.next().unwrap() as usize % NUMBER_OF_WORDS;

            let noun = &self.nouns[noun_index];
            let adjective = &self.adjectives[adjective_index];

            key.push_str(&noun.word);
            key.push('-');
            if noun.gender == "f" {
                key.push_str(&adjective.feminine);
            } else {
                key.push_str(&adjective.masculine);
            }
            key.push('-');
        }

        key.pop(); // Remove the trailing '-'
        key
    }
}
