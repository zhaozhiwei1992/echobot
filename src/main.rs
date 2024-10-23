use std::io::{self, Write};
fn main() {
    println!("Echobot is running. Type 'exit' to quit.");
    loop {
        print!("> ");
        // 确保提示符被立即打印
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "exit" {
            break;
        } else {
            println!("Echo: {}", input.trim());
        }
    }
}
