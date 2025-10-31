fn main() {
    let s = "Hello, ";
    let mut buffer = String::new();
    println!("What is your name?: ");
    
    let error = std::io::stdin().read_line(&mut buffer);
    if error.is_err() {
        println!("Something went wrong while reading the file");
    }
    
    let w = format!("{} {}!", s, format_proper_noun(buffer.as_str()));
    println!("{}", w);
}
// This function will take a string in and split each word out. Each word will then 
// have its first character capitalized while every other letter in the word will be set to
// lowercase. Then all the words are rejoined into a string with spaces between each word.
fn format_proper_noun(s: &str) -> String {
    let words = s.split_ascii_whitespace();
    let result = words.map(|w|{
        let mut c = w.chars();
        let first = c.nth(0);
        let rest = c.collect::<String>().to_lowercase();
        format!("{}{}", first.unwrap().to_uppercase().collect::<String>(), rest.trim_end())
    });
    
    result.collect::<Vec<String>>().join(" ")
}

/* TODO: Get text from file and print to console */