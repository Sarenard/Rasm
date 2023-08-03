

pub fn file_to_tok(file: &str) -> Vec<String> {
    let file = file.replace("\r", "")
                           .replace("\t", " ")
                           .replace("//", "#");

    let mut new_file: String = String::new();

    // Remove comments
    let mut in_comment = false;
    for c in file.chars() {
        if c == '#' {
            in_comment = true;
        }
        if c == '\n' {
            in_comment = false;
        }
        if !in_comment {
            new_file.push(c);
        }
    }

    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut inside_string = false;

    for c in new_file.chars() {
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