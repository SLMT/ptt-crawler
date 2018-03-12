
use std::time::Duration;

use telnet::{Telnet, TelnetEvent};

use error::CrawlerError;
use screen::Screen;

const BYTE_CR: u8 = '\r' as u8;
const BYTE_LF: u8 = '\n' as u8;
const BYTE_SPACE: u8 = ' ' as u8;

const ARROW_UP: [u8; 3] = [27, '[' as u8, 'A' as u8];
const ARROW_DOWN: [u8; 3] = [27, '[' as u8, 'B' as u8];
const ARROW_RIGHT: [u8; 3] = [27, '[' as u8, 'C' as u8];
const ARROW_LEFT: [u8; 3] = [27, '[' as u8, 'D' as u8];

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

    pub fn print_screen(&self) {
        self.screen.print_screen();
    }

    pub fn login(&mut self, account: &str, password: &str) -> Result<(), CrawlerError> {
        self.skip_until("請輸入代號");
        self.tel_conn.write(account.as_bytes()).expect("寫入錯誤");
        self.write_enter();
        self.tel_conn.write(password.as_bytes()).expect("寫入錯誤");
        self.write_enter();

        // XXX: 因為它可能出現「載入中」的字樣，也許是要改成判斷「請重新輸入」或「歡迎您再度拜訪」
        // 其中之一是否有出現

        if !self.skip_until("歡迎您再度拜訪") {
            if self.screen.check_string("請重新輸入") {
                return Err(CrawlerError::UsernameOrPasswordWrong);
            }
            return Err(CrawlerError::SomethingWrong(1));
        }

        println!("登入成功！");

        let mut try_count = 0;
        while !self.screen.check_string("主功能表") {
            self.tel_conn.write(&[BYTE_SPACE]).expect("寫入錯誤");
            self.read_to_timeout();

            try_count += 1;
            if try_count > 5 {
                return Err(CrawlerError::SomethingWrong(2));
            }
        }

        Ok(())
    }

    pub fn go_to_first_board(&mut self) -> Result<(), CrawlerError> {
        self.tel_conn.write(&ARROW_RIGHT).expect("寫入錯誤");
        self.skip_until("即時熱門看板");
        self.tel_conn.write(&ARROW_UP).expect("寫入錯誤");
        self.skip_until_cursor_at(19, 10);
        self.tel_conn.write(&ARROW_RIGHT).expect("寫入錯誤");
        self.skip_until("Gossiping");
        self.tel_conn.write(&ARROW_RIGHT).expect("寫入錯誤");
        self.skip_until("請按任意鍵繼續");
        self.tel_conn.write(&ARROW_RIGHT).expect("寫入錯誤");

        if !self.skip_until("看板《Gossiping》") {
            self.screen.print_screen();
            return Err(CrawlerError::SomethingWrong(3));
        }

        println!("進入八卦版！");

        Ok(())
    }

    fn skip_until(&mut self, pattern: &str) -> bool {
        loop {
            let event = self.tel_conn.read_timeout(Duration::new(3, 0)).expect("IO 錯誤");
            if let TelnetEvent::Data(data) = event {
                self.screen.process(&data);
                if self.screen.check_string(pattern) {
                    return true;
                }
            } else if let TelnetEvent::TimedOut = event {
                self.screen.print_screen();
                return false;
            } else {
                // print!("{:?}", event);
            }
        }
    }

    fn skip_until_cursor_at(&mut self, rid: usize, cid: usize) -> bool {
        loop {
            let event = self.tel_conn.read_timeout(Duration::new(3, 0)).expect("IO 錯誤");
            if let TelnetEvent::Data(data) = event {
                self.screen.process(&data);
                if self.screen.get_cursor_position() == (rid, cid) {
                    return true;
                }
            } else if let TelnetEvent::TimedOut = event {
                self.screen.print_screen();
                return false;
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
