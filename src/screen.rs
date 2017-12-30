
const SCREEN_HEIGHT: usize = 24;
const SCREEN_WIDTH: usize = 80;

const BYTE_ESC: u8 = 27;
const BYTE_CSI: u8 = '[' as u8;
const CSI_SAVE_CUR: u8 = 's' as u8;
const CSI_RES_CUR: u8 = 'u' as u8;

enum ProcessState {
    Normal,
    Escaping,
    CSI
}

pub struct Screen {
    lines: Vec<Vec<char>>,
    cursor: (usize, usize),
    state: ProcessState,
    csi_param: Vec<u8>
}

impl Screen {
    pub fn new() -> Screen {
        let mut screen = Screen {
            lines: vec![],
            cursor: (0, 0),
            state: ProcessState::Normal,
            csi_param: vec![]
        };

        for _ in 0..SCREEN_HEIGHT {
            screen.lines.push(vec![]);
        }

        screen
    }

    pub fn process(&mut self, bytes: &[u8]) {
        println!("Data: {:?}", bytes);

        for byte in bytes {
            match self.state {
                ProcessState::Normal => {
                    match *byte {
                        BYTE_ESC => {
                            self.state = ProcessState::Escaping;
                        },
                        b => {
                            // print!("{:?}", b);
                        }
                    }
                },
                ProcessState::Escaping => {
                    match *byte {
                        BYTE_CSI => {
                            self.state = ProcessState::CSI;
                        },
                        b => {
                            panic!("Unhandled Escaping Code: {:?}", b);
                            self.state = ProcessState::Normal;
                        }
                    }
                },
                ProcessState::CSI => {
                    if is_csi_parameter(*byte) {
                        self.csi_param.push(*byte);
                    } else if is_csi_itermidate(*byte) {
                        unimplemented!("CSI: itermidate byte {}", *byte);
                    } else if is_csi_final(*byte) {
                        panic!("CSI: unhandled CSI '{}', param: {:?}", *byte as char, parse_csi_param(&self.csi_param));
                        self.state = ProcessState::Normal;
                    } else {
                        panic!("CSI: undefined byte {}", *byte);
                    }
                }
            }
        }
    }
}

fn is_csi_parameter(byte: u8) -> bool {
    return 0x30 <= byte && byte <= 0x3F;
}

fn is_csi_itermidate(byte: u8) -> bool {
    return 0x20 <= byte && byte <= 0x2F;
}

fn is_csi_final(byte: u8) -> bool {
    return 0x40 <= byte && byte <= 0x7E;
}

// XXX: only handle numbers with semicolon
fn parse_csi_param(byte: &[u8]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let string = String::from_utf8_lossy(byte);

    for token in string.split(";") {
        result.push(i32::from_str_radix(token, 10).expect("parse error"));
    }

    result
}
