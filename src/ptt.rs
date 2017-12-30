
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
        loop {
            let event = self.tel_conn.read().expect("IO 錯誤");
            if let TelnetEvent::Data(data) = event {
                self.screen.process(&data);
            }
        }
    }

    pub fn login(&mut self) {
        self.skip_until("請輸入代號");
        self.tel_conn.write(&(format!("SLMT\r").into_bytes())).expect("寫入錯誤");
        println!("{:?}", self.read_string());
    }

    fn read_string(&mut self) -> String {
        loop {
            let event = self.tel_conn.read().expect("IO 錯誤");
            if let TelnetEvent::Data(data) = event {
                return BIG5_2003.decode(&data, DecoderTrap::Ignore).expect("Big5 解碼錯誤");
            }
        }
    }

    fn skip_until(&mut self, pattern: &str) -> usize {
        loop {
            let string = self.read_string();
            if let Some(index) = string.find(pattern) {
                return index;
            }
        }
    }
}
