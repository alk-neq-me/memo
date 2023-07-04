pub enum Color<'a> {
    Red(&'a str),
    Purple(&'a str),
    Green(&'a str),
    Delete(&'a str),
}

impl<'a> std::fmt::Display for Color<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Color::Red(txt) => write!(f, "\x1b[1;31m{}\x1b[0m", txt),
            Color::Purple(txt) => write!(f, "\x1b[1;34m{}\x1b[0m", txt),
            Color::Green(txt) => write!(f, "\x1b[1;32m{}\x1b[0m", txt),
            Color::Delete(txt) => write!(f, "\x1b[0;9m{}\x1b[0m", txt),
        };
    } 
}
