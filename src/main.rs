extern crate encoding;
extern crate telnet;
extern crate ansi_escapes;

mod screen;
mod ptt;

use ptt::PttConnection;

fn main() {
    let mut connection = PttConnection::new();
    loop {
        connection.doathing();
    }
}
