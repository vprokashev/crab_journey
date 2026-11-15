use std::io::{self, Read};

fn main() {
    match read_input_meta(&mut io::stdin()) {
        Ok((lines, words, bytes)) => println!("{lines} {words} {bytes}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}

fn read_input_meta(input: &mut impl Read) -> io::Result<(usize, usize, usize)> {
    let mut lines:usize = 0;
    let mut words:usize = 0;
    let mut bytes:usize = 0;
    let mut buf = [0u8; 8192];
    let mut cursor_was_on_whitespace:bool = true;

    loop {
        let n = input.read(&mut buf)?;
        if n == 0 {
            break;
        }
        buf[..n].iter().for_each(|&b| {
            bytes += 1;
            if b == b'\n' {
                lines += 1;
            }
            if !cursor_was_on_whitespace && b.is_ascii_whitespace() {
                words += 1;
            }
            cursor_was_on_whitespace = b.is_ascii_whitespace();
        });
    }

    if !cursor_was_on_whitespace {
        words += 1;
    }

    Ok((lines, words, bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_meta() {
        let mut input1: &[u8] = b"";
        assert_eq!(read_input_meta(&mut input1).unwrap(), (0, 0, 0));

        let mut input2 = "foo\n".as_bytes();
        assert_eq!(read_input_meta(&mut input2).unwrap(), (1, 1, 4));

        let input3_vec: Vec<u8> = b"y\n"
            .iter()
            .copied()
            .cycle()
            .take(1000)
            .collect();
        let mut input3 = input3_vec.as_slice();
        assert_eq!(read_input_meta(&mut input3).unwrap(), (500, 500, 1000));

        let input4_vec: Vec<u8> = b"Hello\n"
            .iter()
            .copied()
            .cycle()
            .take(1000)
            .collect();
        let mut input4 = input4_vec.as_slice();
        assert_eq!(read_input_meta(&mut input4).unwrap(), (166, 167, 1000));

        let mut input5 = "Hello world!\n".as_bytes();
        assert_eq!(read_input_meta(&mut input5).unwrap(), (1, 2, 13));

        let mut input6 = "  hi  \n".as_bytes();
        assert_eq!(read_input_meta(&mut input6).unwrap(), (1, 1, 7));

        let mut input7 = "hello".as_bytes();
        assert_eq!(read_input_meta(&mut input7).unwrap(), (0, 1, 5));

        let mut input8 = "hello\n".as_bytes();
        assert_eq!(read_input_meta(&mut input8).unwrap(), (1, 1, 6));

        let mut input9 = "hello rust\n".as_bytes();
        assert_eq!(read_input_meta(&mut input9).unwrap(), (1, 2, 11));

        let mut input10 = "  hello   rust  \n".as_bytes();
        assert_eq!(read_input_meta(&mut input10).unwrap(), (1, 2, 17));

        let mut input11 = "a\tb\nc".as_bytes();
        assert_eq!(read_input_meta(&mut input11).unwrap(), (1, 3, 5));
    }
}
