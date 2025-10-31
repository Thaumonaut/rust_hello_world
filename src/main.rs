mod hello;

fn main() {
    let s = "Hello, ";
    let mut buffer = String::new();
    println!("What is your name?: ");
    
    let error = std::io::stdin().read_line(&mut buffer);
    if error.is_err() {
        println!("Something went wrong while reading the file");
    }
    
    let w = format!("{} {}!", s, hello::format_proper_noun(buffer.as_str()));
    println!("{}", w);
}

/* TODO: Get text from file and print to console */