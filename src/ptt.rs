
use std::time::Duration;

use telnet::{Telnet, TelnetEvent};
use screen::Screen;

const BYTE_CR: u8 = '\r' as u8;
const BYTE_LF: u8 = '\n' as u8;
const BYTE_SPACE: u8 = ' ' as u8;

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

    pub fn login(&mut self, account: &str, password: &str) {
        self.skip_until("請輸入代號");
        self.tel_conn.write(account.as_bytes()).expect("寫入錯誤");
        self.write_enter();
        self.tel_conn.write(password.as_bytes()).expect("寫入錯誤");
        self.write_enter();

        // XXX: 因為它可能出現「載入中」的字樣，也許是要改成判斷「請重新輸入」或「歡迎您再度拜訪」
        // 其中之一是否有出現

        self.read_to_timeout();

        if self.screen.check_string("請重新輸入") {
            panic!("帳號或密碼錯誤");
        }

        if !self.screen.check_string("歡迎您再度拜訪") {
            self.screen.print_screen();
            panic!("Something wrong");
        }

        println!("登入成功！");

        let mut try_count = 0;
        while !self.screen.check_string("主功能表") {
            self.tel_conn.write(&[BYTE_SPACE]).expect("寫入錯誤");
            self.read_to_timeout();

            try_count += 1;
            if try_count > 5 {
                panic!("Something wrong");
            }
        }

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

    fn write_enter(&mut self) {
        self.tel_conn.write(&[BYTE_CR, BYTE_LF]).expect("寫入錯誤");
    }
}
