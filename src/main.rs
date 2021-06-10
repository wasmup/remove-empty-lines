use std::io;

fn main() -> io::Result<()> {
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                if !line.chars().all(|c| c.is_whitespace()) {
                    print!("{}", line);
                }
                line.clear();
            }
            Err(_) => break,
        }
    }
    Ok(())
}
