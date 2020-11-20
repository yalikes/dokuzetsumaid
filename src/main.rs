use std::io::{self,Write    };
use std::process;
fn main() {
    println!("Hello master! Maid detsu.");
    println!("what can I help you?");
    let mut command=String::new();
    loop{
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut command)
            .expect("read command failed, this must be master's reason!");
        response(&command);
        command.clear();
    }
}

fn response(command: &String){
    let trimed_command=command.trim();
    if trimed_command=="h" || trimed_command=="help"{
        show_help();
    }
}

fn show_help(){
    let clear_command=process::Command::new("clear")
                      .stdout(process::Stdio::inherit())
                      .output()
                      .expect("Failed to clear output");
    let help_vec=vec![
        CommandHelp::new(String::from("help"),
            String::from("show help message")),
        CommandHelp::new(String::from("list"),
            String::from("list all aviliable topic"))
        ];
    println!("{}{}",
        "It seems that master's memory is not so well.",
        "let's tell you what I can do.");
    for message in help_vec.iter() {
        println!("{}    {}",message.command_name,
            message.description);
    }
}

struct CommandHelp{
    command_name: String,
    description: String,
}
impl CommandHelp{
    fn new(command_name:String,description:String)->CommandHelp{
        CommandHelp{
            command_name,
            description,
        }
    }
}
