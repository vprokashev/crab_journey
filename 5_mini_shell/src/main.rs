use std::io;
use std::io::Write;
use std::process::Command;

fn run(line: &str) {
    let mut args = line.split_whitespace();
    let Some(cmd) = args.next() else {
        return; // Пустая строка.
    };
    let mut child = Command::new(cmd);
    child.args(args);
    let mut running_child = child.spawn().expect("Can not run");
    let status = running_child.wait().expect("Interrupted");
    println!("Process ended with {status}");
}

fn main() -> io::Result<()> {
    loop {
        print!("blazing_shell> ");
        io::stdout().flush()?;
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        println!("Run: {trimmed}");
        run(trimmed);
    }
    Ok(())
}
