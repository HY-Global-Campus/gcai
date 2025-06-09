static WORD_LIST: &[&str] = &["open access"];

pub fn filter_and_truncate(input: String) -> String {
    let truncated: String = input.chars().take(1_000).collect();

    let tokens: Vec<&str> = truncated.split_whitespace().collect();

    let mut out_tokens = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        let mut skipped = false;

        for &phrase in WORD_LIST {
            let parts: Vec<&str> = phrase.split_whitespace().collect();
            let len = parts.len();

            if len > 0 && i + len <= tokens.len() && tokens[i..i + len] == parts[..] {
                i += len;
                skipped = true;
                break;
            }
        }

        if !skipped {
            out_tokens.push(tokens[i]);
            i += 1;
        }
    }

    out_tokens.join(" ")
}
