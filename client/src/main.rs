use std::{
    io::{stdin, stdout, Read, Write},
    net::{Shutdown, TcpStream},
    thread,
};

fn prompt(prompt: &str) -> String {
    let mut buff = String::new();

    print!("\x1b[0m{}", prompt);
    stdout().flush().unwrap();

    stdin().read_line(&mut buff).unwrap();
    buff = buff.trim().to_string();

    buff
}

fn main() {
    loop {
        let input = prompt("> \x1b[32m");
        if input == "exit" {
            break;
        }
        if input == "" {
            continue;
        }
        let mut connection = TcpStream::connect("127.0.0.1:5050").expect("Couldnt Connect");
        connection.write(input.as_bytes()).unwrap();

        if input.split(" ").collect::<Vec<&str>>().first() == Some(&"get") {
            println!("waiting...");
            let mut value = [0; 16];
            connection.read_exact(&mut value).unwrap();

            println!("{:?}", value);
        }

        connection.shutdown(Shutdown::Both).unwrap();
    }
}
