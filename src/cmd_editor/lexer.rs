use super::CommandLineEditor;

impl CommandLineEditor {
    pub fn tokenize_command(&self, command: String) -> Vec<Token> {
        command
            .split_whitespace()
            .rev()
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s.starts_with("@"){
                    Token::Adress(s[1..].to_string())
                } else {
                    Token::Alphanumeric(s.to_string())
                }
            })
            .collect()
    }
}
#[derive(Clone, Debug,)]
pub enum Token {
    Alphanumeric(String),
    Adress(String),
}
impl Default for Token {
    fn default() -> Self {
        Token::Alphanumeric(String::new())
    }
}
impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Alphanumeric(s) => s.clone(),
            Token::Adress(s) => s.clone(),
        }
    }
    pub fn trim(&self) -> String {
        self.to_string().trim().to_string()
    }
}