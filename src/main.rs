use multimap::MultiMap;
use std::fs;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::str;
use std::time::Instant;
use termion::raw::IntoRawMode;
fn main() {
    println!("Use ESC to exit");
    let path = Path::new("resources/weighted_strings.txt");
    let mut suggestions = MultiMap::new();
    let contens: String = fs::read_to_string(&path).unwrap();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    for line in contens.lines() {
        let line_splitted: Vec<&str> = line.split('\t').collect();
        let string = line_splitted[0];
        let chars: Vec<char> = string.chars().collect();
        for idx in 1..chars.len() {
            let slice: String = chars[0..idx].iter().collect();
            suggestions.insert(slice.to_string(), string.to_string());
        }
    }
    println!(
        "Number of available suggestions: {}",
        suggestions.keys().count()
    );

    let mut bytes = stdin.bytes();
    let mut query: String = "".to_string();
    loop {
        let b = bytes.next().unwrap().unwrap();
        if b == 127 {
            query.pop();
        } else {
            query.push_str(str::from_utf8(&[b]).unwrap());
        };
        if b == 27 {
            return;
        }
        let now = Instant::now();
        let suggestion = get_suggestion(&query, &suggestions);
        let top_n_suggestions = take_slice(suggestion, 0, 10);
        let elapsed = now.elapsed();
        let output_suggestions = format!(
            "{}: {}      | response time: {:.2?}",
            query,
            top_n_suggestions.join(", "),
            elapsed
        );
        #[allow(unused_must_use)]
        write!(
            stdout,
            "{} {} {} {}",
            termion::clear::All,
            termion::cursor::Goto(5, 5),
            output_suggestions,
            termion::cursor::Goto(5, 5),
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}

fn get_suggestion(query: &str, suggestions: &MultiMap<String, String>) -> Vec<String> {
    let suggestion = suggestions.get_vec(query);
    match suggestion {
        Some(values) => values.to_vec(),
        None => Vec::<String>::new(),
    }
}

fn take_slice(vec: Vec<String>, from_index: usize, to_index: usize) -> Vec<String> {
    if vec.get(from_index).is_none() || vec.get(to_index).is_none() {
        Vec::<String>::new()
    } else {
        vec[from_index..to_index].to_vec()
    }
}
