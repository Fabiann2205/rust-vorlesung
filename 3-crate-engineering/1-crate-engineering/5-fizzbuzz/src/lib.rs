/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> String {
    if i % 3 == 0 {
        if i % 5 == 0 {
            "FizzBuzz".to_owned()
        } else {
            "Fizz".to_owned()
        }
    } else if i % 5 == 0 {
        "Buzz".to_owned()
    } else {
        format!("{i}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let expected_output = include_str!("../fizzbuzz.out");
        let expected_lines: Vec<&str> = expected_output.lines().collect();

        for (i, &line) in expected_lines.iter().enumerate() {
            let i = i as u32 + 1; // da die Zeilen bei 1 beginnen
            let current = fizz_buzz(i);
            println!("i: {i}, current: {current} line: {line}");
            assert_eq!(current, line);
        }
    }
}
