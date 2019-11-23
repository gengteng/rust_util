use std::{
    io::{self, Write},
};

lazy_static! {
    pub static ref IS_ATTY: bool = is_atty();
}

pub enum MessageType { INFO, OK, WARN, ERROR, DEBUG, }

pub fn is_atty() -> bool{
    let isatty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;
    isatty
}

pub fn print_color(color: Option<term::color::Color>, is_bold: bool, m: &str) {
    let mut t = term::stdout().unwrap();
    match *IS_ATTY {
        true => {
            match color {
                Some(c) => t.fg(c).unwrap(),
                None => (),
            }
            if is_bold {
                t.attr(term::Attr::Bold).unwrap();
            }
            write!(t, "{}", m).unwrap();
            t.reset().unwrap();
        },
        false => write!(t, "{}", m).unwrap(),
    };
}

pub fn print_color_and_flush(color: Option<term::color::Color>, is_bold: bool, m: &str) {
    print_color(color, is_bold, m);
    flush_stdout();
}

pub fn print_message_ex(color: Option<term::color::Color>, h: &str, message: &str) {
    print_color(color, true, h);
    println!(" {}", message);
}

pub fn print_message(mt: MessageType, message: &str) {
    match mt {
        MessageType::OK => print_message_ex(Some(term::color::GREEN),      "[OK   ]", message),
        MessageType::WARN => print_message_ex(Some(term::color::YELLOW),   "[WARN ]", message),
        MessageType::ERROR => print_message_ex(Some(term::color::RED),     "[ERROR]", message),
        MessageType::INFO => print_message_ex(None,                        "[INFO ]", message),
        MessageType::DEBUG => print_message_ex(Some(term::color::MAGENTA), "[DEBUG]", message),
    }
}

pub fn flush_stdout() {
    io::stdout().flush().ok();
}

pub fn clear_lastline() {
    print_lastline("");
}

pub fn print_lastline(line: &str) {
    print!("\x1b[1000D{}\x1b[K", line);
    flush_stdout();
}

// thanks https://blog.csdn.net/star_xiong/article/details/89401149
pub fn find_char_boundary(s: &str, index: usize) -> usize {
    if s.len() <= index {
        return index;
    }
    let mut new_index = index;
    while !s.is_char_boundary(new_index) {
        new_index += 1;
    }
    new_index
}

pub fn get_term_width_message(message: &str, left: usize) -> String {
    match term_size::dimensions() {
        None => message.to_string(),
        Some((w, _h)) => {
            let len = message.len();
            if w > len {
               return message.to_string();
            }
            let mut s = String::new();
            s.push_str(&message[0..find_char_boundary(&message, w-10-5-left)]);
            s.push_str("[...]");
            s.push_str(&message[find_char_boundary(&message, len-10)..]);
            s
        },
    }
}
