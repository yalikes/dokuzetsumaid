use std::io;
fn main() {
    println!("Hello master! Maid detsu.");
    println!("what can I help you?");
    let mut command=String::new();
    loop{
        io::stdin()
            .read_line(&mut command)
            .expect("read command failed, this must be master's reason!");
        response(&command);
    }
}

fn response(command: &String){
    let trimed_command=command.trim();
    if(trimed_command=="h" || trimed_command=="help"){
        show_help();
    }
}

fn show_help(){
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