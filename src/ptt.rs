
use std::time::Duration;

use encoding::all::BIG5_2003;
use encoding::types::{Encoding, DecoderTrap};

use telnet::{Telnet, TelnetEvent};
use screen::Screen;

pub struct PttConnection {
    tel_conn: Telnet,
    screen: Screen
}

impl PttConnection {
    pub fn new() -> PttConnection {
        PttConnection {
            tel_conn: Telnet::connect(("ptt.cc", 23), 256).expect("無法連到 PTT"),
            screen: Screen::new()
        }
    }

    pub fn doathing(&mut self) {
        self.login();
    }

    pub fn login(&mut self) {
        self.skip_until("請輸入代號");
        self.tel_conn.write(&(format!("SLMT\r").into_bytes())).expect("寫入錯誤");
        self.read_to_timeout();
        self.screen.print_screen();
    }

    fn skip_until(&mut self, pattern: &str) {
        loop {
            let event = self.tel_conn.read_timeout(Duration::new(3, 0)).expect("IO 錯誤");
            if let TelnetEvent::Data(data) = event {
                self.screen.process(&data);
                if self.screen.check_string(pattern) {
                    return;
                }
            } else if let TelnetEvent::TimedOut = event {
                self.screen.print_screen();
                if self.screen.check_string(pattern) {
                    return;
                }
            } else {
                // print!("{:?}", event);
            }
        }
    }

    fn read_to_timeout(&mut self) {
        loop {
            let event = self.tel_conn.read_timeout(Duration::new(3, 0)).expect("IO 錯誤");
            if let TelnetEvent::Data(data) = event {
                self.screen.process(&data);
            } else if let TelnetEvent::TimedOut = event {
                return;
            }
        }
    }
}
