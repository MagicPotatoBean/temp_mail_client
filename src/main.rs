use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
    sync::mpsc::{self, Receiver},
    thread::{self, sleep},
    time::{self, Duration},
};
use tempmail::{Domain, Tempmail};
#[tokio::main]
async fn main() {
    let (username, domain) = menu();
    let tempmail = Tempmail::new(username, Some(domain));
    println!("Your email: {}@{}", tempmail.username, tempmail.domain);
    // Get a list of messages from the temporary email inbox.
    let mut message_vec = Vec::new();
    let stdin = spawn_stdin();
    println!("Press enter to exit.");
    loop {
        let messages = tempmail.get_messages().await;
        if let Ok(messages) = messages {
            println!("Successfully fetched mail.");
            for (index, message) in messages.into_iter().enumerate() {
                println!("Message {index}");
                if !message_vec.contains(&message) {
                    println!("Message");
                    println!("Id: {}", message.id);
                    println!("From: {}", message.from);
                    println!("Subject: {}", message.subject);
                    println!("Timestamp: {}", message.timestamp);
                    println!("Attachments:");
                    for attachment in &message.attachments {
                        println!("  Filename: {}", attachment.filename);
                        println!("  ContentType: {}", attachment.content_type);
                        println!("  Size: {}", attachment.size);
                    }
                    println!("Body: {}", message.body);
                    message_vec.push(message);
                }
            }
        } else {
            println!("Encountered an error when reading inbox, this is likely due to the mailbox being taken already.");
        }
        if stdin.try_iter().peekable().peek().is_some() {
            return;
        }
        sleep(Duration::from_secs(3));
    }
}

fn spawn_stdin() -> Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let mut str = String::default();
        let _ = std::io::stdin().read_line(&mut str);
        let _ = tx.send(str);
    });
    rx
}
fn menu() -> (String, Domain) {
    println!("Random email, or specific email?");
    'menu: loop {
        println!("1) Random");
        println!("2) Specific");
        print!("Selection: ");
        let _ = std::io::stdout().flush();
        let mut str = String::default();
        let _ = std::io::stdin().read_line(&mut str);
        if let Ok(is_random) = str.trim().parse() {
            let is_random: i32 = is_random;
            let username;
            let domain;
            if is_random == 1 {
                let mut hasher = DefaultHasher::default();
                time::SystemTime::now().hash(&mut hasher);
                let random = hasher.finish();
                username = format!("{random:0x}");
                domain = Domain::default();
                break 'menu (username, domain);
            } else if is_random == 2 {
                print!("Please enter a username: ");
                let _ = std::io::stdout().flush();
                let mut str = String::default();
                let _ = std::io::stdin().read_line(&mut str);
                username = str.trim().to_lowercase();
                println!("Please select a domain: ");
                println!("1) \"1secmail.com\" - Recommended");
                println!("2) \"1secmail.org\"");
                println!("3) \"1secmail.net\"");
                println!("4) \"wwjmp.com\"");
                println!("5) \"esiix.com\"");
                println!("6) \"xojxe.com\"");
                println!("7) \"yoggm.com\"");
                println!("8) \"Random one above\"");
                domain = {
                    'domain: loop {
                        let response = {
                            'domain_in: loop {
                                print!("Domain: ");
                                let _ = std::io::stdout().flush();
                                let mut str = String::default();
                                let _ = std::io::stdin().read_line(&mut str);
                                match str.trim().parse() {
                                    Ok(val) => break 'domain_in val,
                                    _ => println!("Invalid argument, please try again."),
                                }
                            }
                        };
                        match response {
                            1 => break 'domain Domain::SecMailCom,
                            2 => break 'domain Domain::SecMailOrg,
                            3 => break 'domain Domain::SecMailNet,
                            4 => break 'domain Domain::WwjmpCom,
                            5 => break 'domain Domain::EsiixCom,
                            6 => break 'domain Domain::XojxeCom,
                            7 => break 'domain Domain::YoggmCom,
                            8 => break 'domain Domain::random(),
                            _ => {
                                println!("Invalid argument, please try again.");
                            }
                        }
                    }
                };
                break 'menu (username, domain);
            }
            println!("Invalid argument, please try again.");
        } else {
            println!("Invalid argument, please try again.");
        }
    }
}
