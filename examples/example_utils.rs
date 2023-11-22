use std::{env, fmt::Write, fs, io};
#[allow(dead_code)]
pub fn print_with_lines(content: &str) {
    println!(
        "{}",
        content
            .lines()
            .enumerate()
            .fold(String::new(), |mut out, (num, content)| {
                let _ = writeln!(out, "{:4}. {}", num + 1, content);
                out
            })
    );
}

// read first arg as file name or read stdin if arg[1] == "-"
pub fn content_from_arg() -> Result<Option<String>, Box<dyn std::error::Error>> {
    if let Some(arg) = env::args().nth(1) {
        let (mut std_in, mut file);
        let readable: &mut dyn io::Read = if arg == "-" {
            std_in = io::stdin();
            &mut std_in
        } else {
            file = fs::File::open(arg)?;
            &mut file
        };
        let mut output = String::new();
        readable.read_to_string(&mut output)?;
        Ok(Some(output))
    } else {
        eprintln!("no input provided");
        Ok(None)
    }
}

#[allow(dead_code)]
fn main() {
    eprintln!("this is not an example, just a utility module");
}
