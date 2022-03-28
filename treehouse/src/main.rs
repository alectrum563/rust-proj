use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Hello, {}!", name);
    let visitor_list = [
        Visitor::new("aditya", "Hey aditya, we brought some lean."),
        Visitor::new("yuvraj", "Hi yuvraj, we did your geometry homework and failed it..."),
        Visitor::new("gaurav", "Yo gaurav, how'd the football game go?"),
    ];
    match visitor_list
            .iter()
            .find(|visitor| visitor.name == name) {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("get out of my treehouse")
    }
}
