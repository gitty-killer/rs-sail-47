# rs-sail-47

A small Rust tool that computes text statistics for sail.

## Objective
- Provide quick text metrics for sail documents.
- Report top word frequencies for fast inspection.

## Use cases
- Validate sail drafts for repeated terms before review.
- Summarize sail notes when preparing reports.

## Usage
rustc main.rs && ./main data/sample.txt --top 5

## Output
- lines: total line count
- words: total word count
- chars: total character count
- top words: most frequent tokens (case-insensitive)

## Testing
- run `bash scripts/verify.sh`

## Notes
- Only ASCII letters and digits are treated as word characters.
