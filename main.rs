use std::env;
use std::fs;

fn tokenize(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            current.push(ch.to_ascii_lowercase());
        } else if !current.is_empty() {
            words.push(current.clone());
            current.clear();
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: textstats <path> [--top N]");
        std::process::exit(1);
    }

    let mut top = 10usize;
    let mut path = String::new();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--top" && i + 1 < args.len() {
            top = args[i + 1].parse().unwrap_or(10);
            i += 2;
        } else if path.is_empty() {
            path = args[i].clone();
            i += 1;
        } else {
            i += 1;
        }
    }

    let text = fs::read_to_string(&path).expect("read file");
    let lines = if text.is_empty() { 0 } else { text.matches('\n').count() + 1 };
    let words = tokenize(&text);
    let mut counts = std::collections::HashMap::new();
    for w in &words {
        *counts.entry(w.clone()).or_insert(0usize) += 1;
    }

    let mut list: Vec<(String, usize)> = counts.into_iter().collect();
    list.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    println!("lines: {}", lines);
    println!("words: {}", words.len());
    println!("chars: {}", text.len());
    println!("top words:");
    for (word, count) in list.into_iter().take(top) {
        println!("  {}: {}", word, count);
    }
}
