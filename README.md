# rs-sail-47

A small Rust tool that computes text statistics for sail.

## Goal
- Provide quick text metrics for sail documents.
- Report top word frequencies for fast inspection.

## Usage
rustc main.rs && ./main data/sample.txt --top 5

## Output
- lines: total line count
- words: total word count
- chars: total character count
- top words: most frequent tokens (case-insensitive)

## Notes
- Only ASCII letters and digits are treated as word characters.
