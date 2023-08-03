

pub fn file_to_tok(file: &str) -> Vec<String> {
    let file = file.replace("\r", "");
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut inside_string = false;

    for c in file.chars() {
        if c == '\"' {
            inside_string = !inside_string; // Toggle the inside_string flag
            token.push(c);
        } else if inside_string || (c != ' ' && c != '\n') {
            token.push(c);
        } else if !token.is_empty() {
            tokens.push(token.clone());
            token.clear();
        }
    }

    if !token.is_empty() {
        tokens.push(token.clone());
    }

    tokens.retain(|x| x != "");
    tokens
}