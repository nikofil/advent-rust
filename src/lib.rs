use std::io;

pub fn read_line() -> Option<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    buf = buf.trim().to_string();
    if buf.len() > 0 {
        Some(buf)
    } else {
        None
    }
}
