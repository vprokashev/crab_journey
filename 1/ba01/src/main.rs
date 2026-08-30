use std::io::{self, Read};

fn main() {
    match count_bytes(&mut io::stdin()) {
        Ok(count) => println!("{count}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}

fn count_bytes(input: &mut impl Read) -> io::Result<usize> {
    let mut buf = [0u8; 8192];
    let mut count = 0;

    loop {
        let n = input.read(&mut buf)?;

        if n == 0 {
            break;
        }

        count += n;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes() {
        let mut input: &[u8] = b"";
        assert_eq!(count_bytes(&mut input).unwrap(), 0);

        let mut input1 = "foo\n".as_bytes();
        assert_eq!(count_bytes(&mut input1).unwrap(), 4);

        let input2_vec = b"y\n".repeat(500);
        let mut input2 = input2_vec.as_slice();
        assert_eq!(count_bytes(&mut input2).unwrap(), 1000);

        let mut input3 = "Hello world!\n".as_bytes();
        assert_eq!(count_bytes(&mut input3).unwrap(), 13);

        let mut input4 = "\u{1F980}\n".as_bytes(); // 🦀
        assert_eq!(count_bytes(&mut input4).unwrap(), 5);
    }
}