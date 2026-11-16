use std::io;
use std::io::Write;
use std::process::{Child, Command, Stdio};

fn create_command(raw_cmd: &str) -> io::Result<Command> {
    let mut args = raw_cmd.split_whitespace();

    let Some(cmd) = args.next() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Пустая команда",
        ));
    };

    let mut command = Command::new(cmd);
    command.args(args);

    Ok(command)
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
        let commands: io::Result<Vec<Command>> = trimmed
            .split('|')
            .map(str::trim)
            .map(create_command)
            .collect(); // Вычисляем ошибки Command

        let commands = match commands {
            Ok(commands) => commands,
            Err(error) => {
                eprintln!("Ошибка команды: {error}");
                continue;
            }
        };
        let mut commands = commands.into_iter().peekable();

        let mut previous_stdout = None;
        let mut children: Vec<Child> = Vec::new();

        while let Some(mut command) = commands.next() {
            let input = match previous_stdout.take() {
                Some(stdout) => Stdio::from(stdout),
                None => Stdio::inherit(), // Первая команда.
            };

            let output = if commands.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit() // Последняя команда.
            };

            let mut child = match command.stdin(input).stdout(output).spawn() {
                Ok(child) => child,
                Err(error) => {
                    eprintln!("Ошибка запуска {:?}: {error}", command.get_program());
                    drop(command);

                    // Останавливаем предыдущие команды конвейера.
                    // После выхода из while дождёмся их завершения.
                    for child in &mut children {
                        let _ = child.kill();
                    }

                    break;
                }
            };

            previous_stdout = child.stdout.take();
            children.push(child);
        }

        for mut child in children {
            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("Process ended with {status}");
                    }
                }
                Err(error) => eprintln!("Ошибка ожидания процесса: {error}"),
            }
        }
    }
    Ok(())
}
