pub fn process_part1(input: &str) -> String {
    const MARKER_LENGTH: usize = 4;
    for i in 0..input.len()-MARKER_LENGTH {
        let window = &input[i..(i+MARKER_LENGTH)];
        assert!(window.len() == MARKER_LENGTH);
        let mut seen = vec![];
        let has_duplicate = window.chars().any(|c| {
            let res = seen.contains(&c);
            seen.push(c);
            return res;
            });
        if !has_duplicate {
            return (i+MARKER_LENGTH).to_string();
        }
    }
    return input.len().to_string();
}


pub fn process_part2(input: &str) -> String {
    const MARKER_LENGTH: usize = 14;
    for i in 0..input.len()-MARKER_LENGTH {
        let window = &input[i..(i+MARKER_LENGTH)];
        assert!(window.len() == MARKER_LENGTH);
        let mut seen = vec![];
        let has_duplicate = window.chars().any(|c| {
            let res = seen.contains(&c);
            seen.push(c);
            return res;
            });
        if !has_duplicate {
            return (i+MARKER_LENGTH).to_string();
        }
    }
    return input.len().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "10");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "29");
    }
}
