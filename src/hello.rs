// This function will take a string in and split each word out. Each word will then 
// have its first character capitalized while every other letter in the word will be set to
// lowercase. Then all the words are rejoined into a string with spaces between each word.
pub fn format_proper_noun(noun: &str) -> String {
    let trimmed_noun = noun.trim();
    if trimmed_noun.is_empty() {
        return String::from("world");
    }
    let words = trimmed_noun.split_ascii_whitespace();
    let result = words.map(|w|{
        let mut c = w.chars();
        match c.next() {
            Some(first_char) => {
                first_char.to_ascii_uppercase().to_string()
                    + &c.map(|rest| rest.to_ascii_lowercase()).collect::<String>()
            }
            None => String::new(),
        }
    });

    result.collect::<Vec<String>>().join(" ")
}