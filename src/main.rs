extern crate encoding;
extern crate telnet;
extern crate ansi_escapes;

mod screen;
mod ptt;

use ptt::PttConnection;
use std::env;

fn main() {
    if env::args().count() < 3 {
        println!("Usage: {} [account] [password]", env::args().nth(0).unwrap());
        return;
    }
    let account = env::args().nth(1).unwrap();
    let password = env::args().nth(2).unwrap();

    let mut connection = PttConnection::new();
    connection.login(&account, &password);
    connection.go_to_first_board();
}
