
mod ansi;

const SCREEN_HEIGHT: usize = 24;
const SCREEN_WIDTH: usize = 80;

pub struct Screen {
    lines: Vec<Vec<char>>,
    cursor: (usize, usize)
}

impl Screen {
    pub fn new() -> Screen {
        let mut screen = Screen {
            lines: vec![],
            cursor: (0, 0)
        };

        for _ in 0..SCREEN_HEIGHT {
            screen.lines.push(vec![]);
        }

        screen
    }

    pub fn process(&mut self, bytes: &[u8]) {
        println!("Data: {:?}", bytes);

        let tokens = ansi::tokenize(bytes);
        println!("Tokens: {:?}", tokens);
    }
}
