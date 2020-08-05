use std::io;
fn main() {
    println!("Hello master! Maid detsu.");
    println!("what can I help you?");
    let mut command=String::new();
    io::stdin()
            .read_line(&mut command)
            .expect("read command failed, this must be master's reason!");
    
}