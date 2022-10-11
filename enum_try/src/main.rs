use std::io;

enum Message {
    PersonalInfo {
        name: String,
        age: u8,
    },
    WorkInfo {
        company: String,
        position: String,
    },
    Quit,
}

impl Message {
    fn input() -> Message {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Failed to read line");

        let input = input.trim();
        if input == "quit" {
            return Message::Quit;
        } else if input == "personal" {
            let mut name = String::new();
            io::stdin().read_line(&mut name)
            .expect("Failed to read line");
            let mut age = String::new();
            io::stdin().read_line(&mut age)
            .expect("Failed to read line");
            return Message::PersonalInfo {
                name: name.trim().to_string(),
                age: age.trim().parse().expect("Failed to parse age"),
            };
        } else if input == "work" {
            let mut company = String::new();
            io::stdin().read_line(&mut company)
            .expect("Failed to read line");
            let mut position = String::new();
            io::stdin().read_line(&mut position)
            .expect("Failed to read line");
            return Message::WorkInfo {
                company: company.trim().to_string(),
                position: position.trim().to_string(),
            };
        } else {
            println!("Invalid input");
            return Message::input();
        }
    }

    fn print(&self) {
        match self {
            Message::PersonalInfo { name, age } => {
                print!("Personal Info :\n");
                println!("Name: {}", name);
                println!("Age: {}", age);
            },
            Message::WorkInfo { company, position } => {
                println!("Company Info\n");
                println!("Company: {}", company);
                println!("Position: {}", position);
            },
            Message::Quit => {
                println!("Quitting...");
            },
        }
    }
}

fn main() {
    println!("Enter personal, work or quit");
    loop {
        let message = Message::input();
        message.print();
        if let Message::Quit = message {
            break;
        }
    }
}
