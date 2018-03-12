use std::fmt;

#[derive(Debug)]
pub enum CrawlerError {
    UsernameOrPasswordWrong,
    SomethingWrong(u32)
}

impl CrawlerError {
    pub fn to_string(&self) -> String {
        match self {
            &CrawlerError::UsernameOrPasswordWrong => format!("帳號或密碼錯誤"),
            &CrawlerError::SomethingWrong(code) => format!("好像哪裡怪怪的？ Code: {}", code)
        }
    }
}

impl fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
