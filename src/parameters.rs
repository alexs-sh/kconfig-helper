use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// A very simple representation of a parameter's history:
// 0 - old value
// 1 - new value
const MAX_VALUES_SEQUENCE_LEN: usize = 2;
pub(crate) type ValuesSequence = [String; MAX_VALUES_SEQUENCE_LEN];
pub(crate) type Parameters = HashMap<String, ValuesSequence>;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ChangeType {
    Modified,
    Unmodified,
    Added,
    Removed,
}

pub(crate) struct Change<'a> {
    pub change_type: ChangeType,
    pub parameter_name: &'a str,
    pub old_value: &'a str,
    pub new_value: &'a str,
}

fn new_values_sequence(idx: usize, value: &str) -> ValuesSequence {
    assert!(idx < MAX_VALUES_SEQUENCE_LEN);
    let mut sequence = [String::new(), String::new()];
    sequence[idx] = value.to_owned();
    sequence
}

fn update_values_sequence(state: &mut ValuesSequence, idx: usize, value: &str) -> bool {
    assert!(idx < MAX_VALUES_SEQUENCE_LEN);
    let target = &mut state[idx];
    if target.is_empty() {
        *target = value.to_owned();
        true
    } else {
        false
    }
}

fn parse_name_and_value(line: &str) -> Option<(&str, &str)> {
    // There are two possible cases:
    //
    // - A directly enabled parameter, which has the form: CONFIG_XXX=value
    //
    // - A directly disabled parameter, which has the form: # CONFIG_XXX is not set Let's detect
    // these two cases; all other lines will be ignored.
    let mut line = line.trim();
    if line.is_empty() {
        return None;
    }
    if line.starts_with("CONFIG") {
        if let Some(idx) = line.find('=') {
            let key = &line[..idx];
            let value = &line[idx + 1..];
            return Some((key, value));
        } else {
            eprintln!("failed to parse line:'{}'", line);
        }
    } else if line.starts_with('#') {
        line = line[1..].trim();
        if line.starts_with("CONFIG") && line.ends_with("is not set") {
            if let Some(idx) = line.find(' ') {
                let key = &line[..idx];
                let value = &line[idx + 1..];
                return Some((key, value));
            } else {
                eprintln!("failed to parse line:'{}'", line);
            }
        }
    }
    None
}

pub(crate) fn analyze_changes(sequence: &ValuesSequence) -> ChangeType {
    match (!sequence[0].is_empty(), !sequence[1].is_empty()) {
        (true, true) => {
            if sequence[0] == sequence[1] {
                ChangeType::Unmodified
            } else {
                ChangeType::Modified
            }
        }
        (false, true) => ChangeType::Added,
        (true, false) => ChangeType::Removed,
        (false, false) => unreachable!(),
    }
}

pub(crate) fn read(files_to_read: &[&Path; MAX_VALUES_SEQUENCE_LEN]) -> io::Result<Parameters> {
    let mut parameters: HashMap<String, ValuesSequence> = HashMap::new();
    let mut update_history = |line: &str, idx: usize| {
        if let Some((name, value)) = parse_name_and_value(line) {
            parameters
                .entry(name.to_owned())
                .and_modify(|entry| {
                    if !update_values_sequence(entry, idx, value) {
                        eprintln!("WARN: key {} already exists", name);
                    };
                })
                .or_insert(new_values_sequence(idx, value));
        }
    };

    for (idx, file) in files_to_read.iter().enumerate() {
        let lines_read = for_each_line_in_file(file, |line| {
            update_history(line, idx);
        })?;

        eprintln!("Read {} line(s) from {} file", lines_read, file.display());
    }

    Ok(parameters)
}

pub(crate) fn for_each_line_in_file<F>(path: &Path, mut func: F) -> io::Result<u64>
where
    F: FnMut(&str),
{
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut counter = 0_u64;

    for line in reader.lines().flatten() {
        func(&line);
        counter += 1;
    }
    Ok(counter)
}
