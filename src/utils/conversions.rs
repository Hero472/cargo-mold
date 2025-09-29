pub fn to_pascal_case(s: &str) -> String {
    s.split_whitespace()
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

pub fn to_camel_case(s: &str) -> String {
    let mut words = s.split_whitespace()
        .filter(|w| !w.is_empty());

    match words.next() {
        Some(first) => {
            let mut result = first.to_lowercase();
            for w in words {
                let mut c = w.chars();
                if let Some(first_char) = c.next() {
                    result.push_str(
                        &(first_char.to_uppercase().collect::<String>() + c.as_str())
                    );
                }
            }
            result
        }
        None => String::new(),
    }
}