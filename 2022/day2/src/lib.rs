use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Combination {
    you: String, // A B C
    me: String,  // X Y Z
}

impl Combination {
    fn new(you: &str, me: &str) -> Self {
        Self {
            you: you.to_string(),
            me: me.to_string(),
        }
    }
}

pub fn score(contents: String) -> i32 {
    let scoring: HashMap<Combination, i32> = HashMap::from([
        (Combination::new("A", "X"), 4),
        (Combination::new("A", "Y"), 8),
        (Combination::new("A", "Z"), 3),
        (Combination::new("B", "X"), 1),
        (Combination::new("B", "Y"), 5),
        (Combination::new("B", "Z"), 9),
        (Combination::new("C", "X"), 7),
        (Combination::new("C", "Y"), 2),
        (Combination::new("C", "Z"), 6),
    ]);

    let mut score = 0;

    for line in contents.split("\n") {
        if line == "" {
            break;
        }

        let combination: Vec<&str> = line.split(" ").collect();

        match scoring.get(&Combination::new(combination[0], combination[1])) {
            Some(point) => score += point,
            None => panic!("All combinations should map to points"),
        }
    }

    score
}

pub fn predict(contents: String) -> i32 {
    let scoring: HashMap<Combination, i32> = HashMap::from([
        (Combination::new("A", "X"), 3),
        (Combination::new("A", "Y"), 4),
        (Combination::new("A", "Z"), 8),
        (Combination::new("B", "X"), 1),
        (Combination::new("B", "Y"), 5),
        (Combination::new("B", "Z"), 9),
        (Combination::new("C", "X"), 2),
        (Combination::new("C", "Y"), 6),
        (Combination::new("C", "Z"), 7),
    ]);

    let mut score = 0;

    for line in contents.split("\n") {
        if line == "" {
            break;
        }

        let combination: Vec<&str> = line.split(" ").collect();

        match scoring.get(&Combination::new(combination[0], combination[1])) {
            Some(point) => score += point,
            None => panic!("All combinations should map to points"),
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_score() {
        let res = score("A Y\nB X\nC Z\n".to_string());

        assert_eq!(res, 15);
    }

    #[test]
    fn return_predict() {
        let res = predict("A Y\nB X\nC Z\n".to_string());

        assert_eq!(res, 12);
    }
}
