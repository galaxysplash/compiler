pub fn cut(uncut_string: &str) -> Vec<String> {
    const EXCLUDED_CUTOUTS: [char; 1] = [' '];
    const INCLUDED_CUTOUTS: [char; 5] = ['(', ')', '{', '}', '\t'];
    const FORBIDDEN: [char; 1] = ['\t'];

    let mut cut_string: Vec<String> = vec![];
    let mut string_buffer: String = String::from("");

    for current_char in uncut_string.chars() {
        if search(current_char, EXCLUDED_CUTOUTS) {
            cutout(&mut string_buffer, &mut cut_string);
            continue;
        }
        if search(current_char, INCLUDED_CUTOUTS) {
            cutout(&mut string_buffer, &mut cut_string);
            cut_string.push(String::from(current_char));
            continue;
        }
        if search(current_char, FORBIDDEN) {
            panic!("forbidden token '{current_char}' in here!");
        }

        string_buffer.push(current_char);
    }

    cut_string.push(string_buffer.clone());

    cut_string
}

fn cutout(string_buffer: &mut String, cut_string: &mut Vec<String>) {
    cut_string.push(string_buffer.clone());
    string_buffer.clear();
}

fn search<const N: usize>(current_char: char, check_chars: [char; N]) -> bool {
    for check_char in check_chars {
        if check_char == current_char {
            return true;
        }
    }

    false
}
