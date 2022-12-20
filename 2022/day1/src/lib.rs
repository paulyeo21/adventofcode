use std::collections::BinaryHeap;

pub fn get_fattest(contents: String) -> i32 {
    let mut fattest_elf = 0;
    let mut current_elf = 0;

    for line in contents.split("\n") {
        let line = line.trim();

        if line == "" {
            fattest_elf = std::cmp::max(fattest_elf, current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line
            .parse::<i32>()
            .expect("Input should be a number convertible to i32");
    }

    fattest_elf
}

pub fn top_three_fattest(contents: String) -> i32 {
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut current_elf = 0;

    for line in contents.split("\n") {
        let line = line.trim();

        if line == "" {
            max_heap.push(current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line
            .parse::<i32>()
            .expect("Input should be a number convertible to i32");
    }

    let mut res = 0;

    // return three or less
    for _ in 0..3 {
        match max_heap.pop() {
            Some(item) => res += item,
            None => break,
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_fattest_elf() {
        let res = get_fattest("10\n10\n5\n\n".to_string());

        assert_eq!(res, 25);
    }

    #[test]
    fn return_top_three_fattest_elves() {
        let res = top_three_fattest("10\n\n1\n\n2\n\n3\n\n".to_string());

        assert_eq!(res, 15);
    }

    #[test]
    fn return_less_than_three_elves() {
        let res = top_three_fattest("1\n\n2\n".to_string());

        assert_eq!(res, 3);

        let res = top_three_fattest("1\n".to_string());

        assert_eq!(res, 1);

        let res = top_three_fattest("".to_string());

        assert_eq!(res, 0);
    }
}
