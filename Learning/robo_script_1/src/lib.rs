

fn get_color(ch: u8) -> Option<String> {
    match ch {
        b'F' => Some(String::from("pink")),
        b'L' => Some(String::from("red")),
        b'R' => Some(String::from("green")),
        b'0' ... b'9' => Some(String::from("orange")),
//        _ if ch.is_ascii_digit() => Some(String::from("orange")),
        _ => None,
    }
}

pub fn wrap_it(code: &str) -> String {
//    if let Some(color) = get_color(code.as_bytes()[0]) {
//            format!("<span style=\"color: {}\">{}</span>", color, code)
//    }
//    else { String::from("") }

    get_color(code.as_bytes()[0])
        .map(|s| format!("<span style=\"color: {}\">{}</span>", s, code ))
        .or_else(|| Some(String::from(code)))
        .unwrap()

//    match get_color(code.as_bytes()[0]) {
//        Some(color) => format!("<span style=\"color: {}\">{}</span>", color, code ),
//        None => String::from(""),
//    }
}

pub fn highlight(code: &str) -> String {
    let mut start = (0usize, code.as_bytes()[0] as char);
    let mut _r = String::new();

    for ( i , ch) in code.chars().enumerate() {
        if start.1 != ch {
            if start.1.is_digit(10) && ch.is_digit(10) { /* do nothing */ }
            else{
                _r += &wrap_it(&code[start.0..i]);
                start = (i, ch);
            }
        }
    }

    _r += &wrap_it(&code[start.0..]);

    _r
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color() {
        assert_eq!(Option::Some(String::from("pink")), get_color(b'F'));
        assert_eq!(Option::Some(String::from("red")), get_color(b'L'));
        assert_eq!(Option::Some(String::from("green")), get_color(b'R'));
        assert_eq!(Option::Some(String::from("orange")), get_color(b'0'));
        assert_eq!(Option::Some(String::from("orange")), get_color(b'1'));
        assert_eq!(Option::Some(String::from("orange")), get_color(b'5'));
        assert_eq!(Option::Some(String::from("orange")), get_color(b'9'));
    }
}