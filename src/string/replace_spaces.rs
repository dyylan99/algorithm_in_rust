pub fn replace_spaces(s: String, length: i32) -> String {
    let mut res=String::from("");
    for (index,value) in s.chars().enumerate() {
        if index==length as usize {
            break;
        }
        match value {
            ' ' => res.push_str("%20"),
            _ => res.push(value),
            
        }
    }
    res
}

#[test]
fn test() {
    let replace_spaces = replace_spaces("Mr John Smith    ".to_string(), 13);
    println!("{}", replace_spaces)
}
