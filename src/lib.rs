#[cfg(target_family = "wasm")]
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

static WORDS: &str = include_str!("resources/words.txt");

trait Game {
    fn can_be_wordle(
        &self,
        confirmed: &str,
        extras: &str,
        multiples: &str,
        schema: &Vec<SchemaValue>,
    ) -> bool;
}

impl Game for &str {
    fn can_be_wordle(
        &self,
        confirmed: &str,
        extras: &str,
        multiples: &str,
        schema: &Vec<SchemaValue>,
    ) -> bool {
        for char in multiples.chars() {
            if self.matches(char).count() <= 1 {
                return false;
            }
        }

        let mut checked = 0;
        for char in confirmed.chars() {
            if !self.contains(char) {
                return false;
            }

            checked += 1;
        }

        if checked != confirmed.len() {
            return false;
        } else if extras.len() != 0
            && !self.chars().all(|c| {
                let all_letters = format!("{}{}", confirmed, extras);
                all_letters.contains(c)
            })
        {
            return false;
        } else if schema.len() == 0 {
            return true;
        }

        let mut is_valid = [false; 5];
        for schema_value in schema.iter() {
            match schema_value.state {
                2 => {
                    if self.chars().skip(schema_value.index - 1).next()
                        == schema_value.letters.chars().next()
                    {
                        is_valid[schema_value.index - 1] = true;
                        continue;
                    }
                }
                1 => {
                    for char in schema_value.letters.chars() {
                        if self.chars().skip(schema_value.index - 1).next() == Some(char) {
                            is_valid[schema_value.index - 1] = false;
                            continue;
                        }
                        if self.contains(char) {
                            is_valid[schema_value.index - 1] = true;
                            continue;
                        }
                    }
                }
                _ => is_valid[schema_value.index - 1] = true,
            }
        }

        if !is_valid.contains(&false) {
            return true;
        }

        false
    }
}

trait Schema {
    fn schema_from_str(s: &str) -> Self;
}

impl Schema for Vec<SchemaValue> {
    fn schema_from_str(s: &str) -> Self {
        let mut schema: Vec<SchemaValue> = Vec::new();

        for (idx, letters) in s.split_inclusive(char::is_numeric).enumerate() {
            schema.push(SchemaValue::from_str(letters, idx + 1));
        }

        schema
    }
}

struct SchemaValue {
    index: usize,
    state: u8,
    letters: String,
}

impl SchemaValue {
    fn from_str(s: &str, index: usize) -> Self {
        let mut state = 0;
        let mut letters = String::new();

        for c in s.chars() {
            if c.is_numeric() {
                state = c.to_digit(10).unwrap() as u8;
            } else {
                letters.push(c);
            }
        }

        SchemaValue {
            index,
            state,
            letters,
        }
    }
}

#[wasm_bindgen]
pub fn solver(confirmed: &str, extras: &str, multiples: &str, schema: &str) -> String {
    #[cfg(target_family = "wasm")]
    console_error_panic_hook::set_once();

    let schema = Vec::schema_from_str(schema);

    let words: Vec<_> = WORDS
        .lines()
        .filter(|line| line.can_be_wordle(&confirmed, &extras, &multiples, &schema))
        .collect();

    words.join(",")
}
