
const ESC: u8 = 27;
const ESC_CSI: u8 = '[' as u8;
const CSI_CUU: u8 = 'A' as u8;
const CSI_CUD: u8 = 'B' as u8;
const CSI_CUF: u8 = 'C' as u8;
const CSI_CUB: u8 = 'D' as u8;
const CSI_CNL: u8 = 'E' as u8;
const CSI_CPL: u8 = 'F' as u8;
const CSI_CHA: u8 = 'G' as u8;
const CSI_CUP: u8 = 'H' as u8;
const CSI_ED: u8 = 'J' as u8;
const CSI_EL: u8 = 'K' as u8;
const CSI_SU: u8 = 'S' as u8;
const CSI_SD: u8 = 'T' as u8;
const CSI_HVP: u8 = 'f' as u8;
const CSI_SGR: u8 = 'm' as u8;
const CSI_AUX: u8 = 'i' as u8;
const CSI_DSR: u8 = 'n' as u8;
const CSI_SCP: u8 = 's' as u8;
const CSI_RCP: u8 = 'u' as u8;

#[derive(Debug)]
pub enum AnsiToken<'a> {
    Text(&'a [u8]),

    // Cursor Movement
    CursorUp(isize),
    CursorDown(isize),
    CursorForward(isize),
    CursorBack(isize),

    UnknownCsi(Vec<i32>, u8), // parameter list, final byte
    UnknownEscape(u8)
}

enum ProcessState {
    Text,
    Escaping,
    CSI
}

pub fn tokenize(bytes: &[u8]) -> Result<Vec<AnsiToken>, String> {
    let mut tokens = vec![];
    let mut state = ProcessState::Text;
    let mut text_start: usize = 0;
    let mut param_start: usize = 0;

    for i in 0 .. bytes.len() {
        let byte = bytes[i];
        match state {
            ProcessState::Text => {
                match byte {
                    ESC => {
                        if i > text_start {
                            tokens.push(AnsiToken::Text(&bytes[text_start .. i]));
                        }
                        state = ProcessState::Escaping;
                    },
                    b => {
                        // print!("{:?}", b);
                    }
                }
            },
            ProcessState::Escaping => {
                match byte {
                    ESC_CSI => {
                        state = ProcessState::CSI;
                        param_start = i + 1;
                    },
                    b => {
                        tokens.push(AnsiToken::UnknownEscape(b));
                        state = ProcessState::Text;
                        text_start = i + 1;
                    }
                }
            },
            ProcessState::CSI => {
                if is_csi_final_byte(byte) {
                    // Process the sequence
                    let params = parse_csi_param(&bytes[param_start .. i]);
                    let token = match match_csi(byte, params) {
                        Ok(t) => t,
                        Err(msg) => return Err(msg)
                    };
                    tokens.push(token);

                    state = ProcessState::Text;
                    text_start = i + 1;
                } else if !is_csi_param_or_interm_byte(byte) {
                    return Err(format!("Undefined byte occurred in control sequences: {}", byte));
                }
            }
        }
    }

    Ok(tokens)
}

fn match_csi(final_byte: u8, params: Vec<i32>) -> Result<AnsiToken<'static>, String> {
    match final_byte {
        CSI_CUU | CSI_CUD | CSI_CUF | CSI_CUB => {
            let mut moves: isize = 1;
            if params.len() == 1 {
                moves = params[0] as isize;
            } else if params.len() > 1 {
                return Err(format!("The parameter list is too long: {:?}", params));
            }

            match final_byte {
                CSI_CUU => Ok(AnsiToken::CursorUp(moves)),
                CSI_CUD => Ok(AnsiToken::CursorDown(moves)),
                CSI_CUF => Ok(AnsiToken::CursorForward(moves)),
                CSI_CUB => Ok(AnsiToken::CursorBack(moves)),
                _ => Err(format!("It should not be here"))
            }
        },
        b => {
            Ok(AnsiToken::UnknownCsi(params, b))
        }
    }
}

// CSI Parameter Byte: 0x30–0x3F
// CSI Intermidiate Byte: 0x20–0x2F
fn is_csi_param_or_interm_byte(byte: u8) -> bool {
    return 0x20 <= byte && byte <= 0x3F;
}

fn is_csi_final_byte(byte: u8) -> bool {
    return 0x40 <= byte && byte <= 0x7E;
}

// XXX: only handle "numbers" with "semicolon"
fn parse_csi_param(byte: &[u8]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let string = String::from_utf8_lossy(byte);

    for token in string.split(";") {
        if token.len() == 0 {
            result.push(0);
        } else {
            result.push(i32::from_str_radix(token, 10).expect("parse error"));
        }
    }

    result
}
