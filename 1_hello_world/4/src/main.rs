fn main() {}

pub fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
    if a > (u8::MAX - b) {
        None
    } else {
        Some(a + b)
    }
}

pub fn add_u8_wrapping(a: u8, b: u8) -> u8 {
    if a > u8::MAX - b {
        a - (u8::MAX - b) - 1
    } else {
        a + b
    }
}

pub fn add_u8_saturating(a: u8, b: u8) -> u8 {
    if a > (u8::MAX - b) {
        u8::MAX
    } else {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_overflow_modes() {
        assert_eq!(add_u8_checked(255, 1), None);
        assert_eq!(add_u8_wrapping(255, 1), 0);
        assert_eq!(add_u8_saturating(255, 1), 255);

        assert_eq!(add_u8_checked(10, 20), Some(30));
        assert_eq!(add_u8_wrapping(10, 20), 30);
        assert_eq!(add_u8_saturating(10, 20), 30);
    }
}
