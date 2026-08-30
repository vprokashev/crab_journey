fn main() {
    let mut arguments: Vec<String> = std::env::args().skip(1).collect();

    insertion_sort(&mut arguments);

    for argument in arguments {
        println!("{argument}");
    }
}

fn insertion_sort(arguments: &mut [String]) {
    for i in 1..arguments.len() {
        let mut j = i;

        while j > 0 && arguments[j] < arguments[j - 1] {
            arguments.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn test_insertion_sort() {
        let mut empty = strings(&[]);
        insertion_sort(&mut empty);
        assert_eq!(empty, strings(&[]));

        let mut sorted = strings(&["a", "b", "c"]);
        insertion_sort(&mut sorted);
        assert_eq!(sorted, strings(&["a", "b", "c"]));

        let mut reversed = strings(&["e", "d", "c", "b", "a"]);
        insertion_sort(&mut reversed);
        assert_eq!(reversed, strings(&["a", "b", "c", "d", "e"]));

        let mut repeated = strings(&["A", "a", "A", "a", "A", "a"]);
        insertion_sort(&mut repeated);
        assert_eq!(repeated, strings(&["A", "A", "A", "a", "a", "a"]));

        let mut punctuation = strings(&["hello,", "world,", "this", "is", "a", "program"]);
        insertion_sort(&mut punctuation);
        assert_eq!(
            punctuation,
            strings(&["a", "hello,", "is", "program", "this", "world,"])
        );
    }
}
