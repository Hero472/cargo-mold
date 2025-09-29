pub fn find_matching_parenthesis(content: &str, start_pos: usize) -> Option<usize> {
    let mut count = 1;
    let chars: Vec<char> = content[start_pos..].chars().collect();
    
    for (i, c) in chars.iter().enumerate().skip(1) {
        match c {
            '(' => count += 1,
            ')' => {
                count -= 1;
                if count == 0 {
                    return Some(start_pos + i);
                }
            }
            _ => {}
        }
    }
    None
}