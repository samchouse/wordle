static WORDS: &str = include_str!("resources/words.txt");

#[derive(Debug)]
struct SchemaValue {
    index: usize,
    state: SchemaState,
    letters: String,
}

#[derive(Debug)]
enum SchemaState {
    Unknown,
    Contains,
    Exact,
}

trait Wordle {
    fn to_schema_value(&self, index: usize) -> SchemaValue;
    fn to_schema(&self) -> Vec<SchemaValue>;
    fn can_be_wordle(
        &self,
        confirmed: &str,
        left_overs: &str,
        multiples: &str,
        schema: &Vec<SchemaValue>,
    ) -> bool;
}

impl TryFrom<char> for SchemaState {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(SchemaState::Unknown),
            '/' => Ok(SchemaState::Contains),
            '!' => Ok(SchemaState::Exact),
            _ => Err(()),
        }
    }
}

impl Wordle for &str {
    fn to_schema_value(&self, index: usize) -> SchemaValue {
        let mut state = SchemaState::Unknown;
        let mut letters = String::new();

        for c in self.chars() {
            if let Ok(lstate) = SchemaState::try_from(c) {
                state = lstate;
                continue;
            }

            letters.push(c);
        }

        SchemaValue {
            index,
            state,
            letters,
        }
    }

    fn to_schema(&self) -> Vec<SchemaValue> {
        let mut schema: Vec<SchemaValue> = Vec::new();

        for (idx, letters) in self
            .split_inclusive(|c| c == '!' || c == '/' || c == '?')
            .enumerate()
        {
            schema.push(letters.to_schema_value(idx + 1));
        }

        schema
    }

    fn can_be_wordle(
        &self,
        confirmed: &str,
        left_overs: &str,
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
        } else if left_overs.len() != 0
            && !self.chars().all(|c| {
                let all_letters = format!("{}{}", confirmed, left_overs);
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
                SchemaState::Exact => {
                    if self.chars().skip(schema_value.index - 1).next()
                        == schema_value.letters.chars().next()
                    {
                        is_valid[schema_value.index - 1] = true;
                        continue;
                    }
                }
                SchemaState::Contains => {
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
                SchemaState::Unknown => is_valid[schema_value.index - 1] = true,
            }
        }

        if !is_valid.contains(&false) {
            return true;
        }

        false
    }
}

pub fn solver(confirmed: &str, extras: &str, multiples: &str, schema: &str) -> String {
    let schema = schema.to_schema();
    let words: Vec<_> = WORDS
        .lines()
        .filter(|word| word.can_be_wordle(&confirmed, &extras, &multiples, &schema))
        .collect();

    words.join(",")
}
