pub fn fizz_buzz(i: u32, result: &mut String) {
    result.clear();
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => result.push_str("FizzBuzz"),
        (true, false) => result.push_str("Fizz"),
        (false, true) => result.push_str("Buzz"),
        (false, false) => {
            use std::fmt::Write;
            write!(result, "{}", i).unwrap();
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let expected_output = include_str!("../fizzbuzz.out");
        let expected_lines: Vec<&str> = expected_output.lines().collect();

        let mut result = String::new();
        for (i, &line) in expected_lines.iter().enumerate() {
            let i = i as u32 + 1; // da die Zeilen bei 1 beginnen
            fizz_buzz(i, &mut result);
            println!("i: {i}, current: {result} line: {line}");
            assert_eq!(result, line);
        }
    }
}