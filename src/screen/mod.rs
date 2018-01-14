
mod ansi;

use encoding::all::BIG5_2003;
use encoding::types::{Encoding, DecoderTrap};

use self::ansi::{AnsiTokenizer, AnsiToken, EraseOption};

const SCREEN_HEIGHT: usize = 24;
const SCREEN_WIDTH: usize = 80;

const SPACE_BYTE: u8 = 32;

pub struct Screen {
    tokenizer: AnsiTokenizer,
    lines: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    cursor: (usize, usize) // (r, c)
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            tokenizer: AnsiTokenizer::new(),
            lines: [[SPACE_BYTE; SCREEN_WIDTH]; SCREEN_HEIGHT],
            cursor: (0, 0)
        }
    }

    pub fn process(&mut self, bytes: &[u8]) {
        // println!("Data: {:?}", bytes);

        for token in self.tokenizer.tokenize(bytes).unwrap() {
            // println!("Tokens: {:?}", token);

            match token {
                AnsiToken::Text(text_bytes) => {
                    for b in text_bytes {
                        if is_control(*b) {
                            self.handle_control_byte(*b);
                        } else {
                            self.type_in(*b);
                        }
                    }

                    // println!("===== Print Screen =====");
                    // self.print_screen();
                },
                AnsiToken::SelectGraphic(_) | AnsiToken::ResetGraphic => {}, // Ignored
                AnsiToken::CursorPosition(r, c) => {
                    self.cursor.0 = (r - 1) as usize;
                    self.cursor.1 = (c - 1) as usize;
                },
                AnsiToken::EraseDisplay(opt) => {
                    match opt {
                        EraseOption::EraseEntire => {
                            self.erase_screen();
                        },
                        _ => {
                            println!("Unhandled EraseDisplay Operation: {:?}", opt);
                        }
                    }
                },
                AnsiToken::EraseLine(opt) => {
                    match opt {
                        EraseOption::EraseToEnd => {
                            self.erase_to_end(true);
                        },
                        _ => {
                            println!("Unhandled EraseLine Operation: {:?}", opt);
                        }
                    }
                },
                _ => {
                    println!("Unhandled Escaped Code: {:?}", token);
                }
            }
        }
    }

    fn backspace(&mut self) {
        self.lines[self.cursor.0][self.cursor.1] = SPACE_BYTE;
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        }
    }

    fn type_in(&mut self, byte: u8) {
        self.lines[self.cursor.0][self.cursor.1] = byte;
        self.cursor.1 += 1;
    }

    fn handle_control_byte(&mut self, byte: u8) {
        match byte {
            0 => {  // NUL: Null (\0)
                self.print_screen();
            },
            8 => {  // BS: Backspace (\b)
                self.backspace();
            },
            10 => { // LF: Line Feed (\n)
                self.cursor.0 += 1;
            },
            13 => { // CR: Carriage Return (\r)
                self.cursor.1 = 0;
            },
            _ => {
                println!("Unhandled Control Byte: {:?}", byte);
            }
        }
    }

    pub fn print_screen(&self) {
        println!("==============================================================================");
        for i in 0 .. SCREEN_HEIGHT {
            println!("{:02}| {}", i + 1, BIG5_2003.decode(&self.lines[i][..], DecoderTrap::Ignore).expect("Big5 解碼錯誤"));
        }
        println!("==============================================================================");
    }

    fn erase_to_end(&mut self, only_line: bool) {
        for i in self.cursor.1 .. SCREEN_WIDTH {
            self.lines[self.cursor.0][i] = SPACE_BYTE;
        }

        // Erase the area below the current line
        if !only_line {
            for i in self.cursor.0 + 1 .. SCREEN_HEIGHT {
                for j in 0 .. SCREEN_WIDTH {
                    self.lines[i][j] = SPACE_BYTE;
                }
            }
        }
    }

    fn erase_screen(&mut self) {
        for i in 0 .. SCREEN_HEIGHT {
            for j in 0 .. SCREEN_WIDTH {
                self.lines[i][j] = SPACE_BYTE;
            }
        }
    }
}

fn is_control(byte: u8) -> bool {
    return byte < 32;
}
