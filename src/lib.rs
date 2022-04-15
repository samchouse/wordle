#[cfg(target_family = "wasm")]
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

static WORDS: &str = include_str!("resources/words.txt");

enum SchemaValueState {
    Unknown,
    Contains,
    Exact,
}

impl SchemaValueState {
    fn from_u8(value: u8) -> SchemaValueState {
        match value {
            0 => SchemaValueState::Unknown,
            1 => SchemaValueState::Contains,
            2 => SchemaValueState::Exact,
            _ => SchemaValueState::Unknown,
        }
    }
}

struct SchemaValue {
    index: usize,
    state: SchemaValueState,
    letters: String,
}

trait Wordle {
    fn to_schema_value(&self, index: usize) -> SchemaValue;
    fn to_schema(&self) -> Vec<SchemaValue>;
    fn can_be_wordle(
        &self,
        confirmed: &str,
        extras: &str,
        multiples: &str,
        schema: &Vec<SchemaValue>,
    ) -> bool;
}

impl Wordle for &str {
    fn to_schema_value(&self, index: usize) -> SchemaValue {
        let mut state = SchemaValueState::Unknown;
        let mut letters = String::new();

        for c in self.chars() {
            if c.is_numeric() {
                state = SchemaValueState::from_u8(c.to_digit(10).unwrap() as u8);
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

    fn to_schema(&self) -> Vec<SchemaValue> {
        let mut schema: Vec<SchemaValue> = Vec::new();

        for (idx, letters) in self.split_inclusive(char::is_numeric).enumerate() {
            schema.push(letters.to_schema_value(idx + 1));
        }

        schema
    }

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
                SchemaValueState::Exact => {
                    if self.chars().skip(schema_value.index - 1).next()
                        == schema_value.letters.chars().next()
                    {
                        is_valid[schema_value.index - 1] = true;
                        continue;
                    }
                }
                SchemaValueState::Contains => {
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
                SchemaValueState::Unknown => is_valid[schema_value.index - 1] = true,
            }
        }

        if !is_valid.contains(&false) {
            return true;
        }

        false
    }
}

#[wasm_bindgen]
pub fn solver(confirmed: &str, extras: &str, multiples: &str, schema: &str) -> String {
    #[cfg(target_family = "wasm")]
    console_error_panic_hook::set_once();

    let words: Vec<_> = WORDS
        .lines()
        .filter(|line| line.can_be_wordle(&confirmed, &extras, &multiples, &schema.to_schema()))
        .collect();

    words.join(",")
}
