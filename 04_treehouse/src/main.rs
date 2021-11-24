#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn prompt_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Problem in reading the input.");
    name.trim().to_lowercase()
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome!"),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            _ => println!("You cannot enter.")
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
}

fn main() {
    println!("What is your name? [Leave empty and press enter to exit.]");
    let mut visitors = vec![
        Visitor::new("fred", VisitorAction::Accept, 40),
        Visitor::new("manu", VisitorAction::AcceptWithNote { note: String::from("Hello Manu, thanks for the aperitives") }, 30),
        Visitor::new("lionel", VisitorAction::Accept, 48),
        Visitor::new("gaetan", VisitorAction::Accept, 35),
        Visitor::new("eric zemmour", VisitorAction::Refuse, 50)
    ];

    loop {
        let name = prompt_name();
        let known_visitor = visitors.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("You were not invited, but come anyway.");
                    visitors.push(Visitor::new(
                        &name,
                        VisitorAction::AcceptWithNote { note: String::from("Ah, you were a new guest, weren't you?") },
                        0)
                    );
                }
            }
        };
    };
    println!("Final list of visitors: {:#?}", visitors);
}
