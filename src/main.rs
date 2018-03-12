extern crate encoding;
extern crate telnet;
extern crate ansi_escapes;

mod error;
mod screen;
mod ptt;

use std::env;

use error::CrawlerError;
use ptt::PttConnection;

fn main() {
    if env::args().count() < 3 {
        println!("Usage: {} [account] [password]", env::args().nth(0).unwrap());
        return;
    }
    let account = env::args().nth(1).unwrap();
    let password = env::args().nth(2).unwrap();

    let mut connection = PttConnection::new();
    if let Err(error_type) = execute(&mut connection, &account, &password) {
        println!("Error: {}", error_type);
        println!("Print the current screen:");
        connection.print_screen();
    }
}

fn execute(connection: &mut PttConnection, account: &str, password: &str) -> Result<(), CrawlerError> {
    connection.login(account, password)?;
    connection.go_to_first_board()?;

    // Debug
    connection.print_screen();

    Ok(())
}
