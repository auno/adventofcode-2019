use std::io;
use std::io::BufRead;
use mod_exp::mod_exp;

use crate::Operation::*;

#[derive(Debug)]
enum Operation {
    DealWithIncrements(i128),
    DealIntoNewStack,
    Cut(i128),
}

impl From<&String> for Operation {
    fn from(s: &String) -> Operation {
        let s = s.trim();

        if s == "deal into new stack" {
            return DealIntoNewStack;
        }

        if s.starts_with("deal with increment ") {
            let n = s.split(" ").last().unwrap().parse().unwrap();
            return DealWithIncrements(n);
        }

        if s.starts_with("cut ") {
            let n = s.split(" ").last().unwrap().to_string().parse().unwrap();
            return Cut(n);
        }

        unreachable!("Unknown technique: {}", s);
    }
}

fn reverse_deal_with_increment(increment: i128, len: i128, index: i128) -> i128 {
    (0..increment)
        .map(|x| x * len + index)
        .find_map(|x| match x % increment {
            0 => Some(x / increment),
            _ => None
        })
        .unwrap()
}

fn reverse_cut(n: i128, len: i128, index: i128) -> i128 {
    let n = match n < 0 {
        true => len - n.abs(),
        false => n
    };

    (n + index) % len
}

fn reverse_deal_into_new_stack(len: i128, index: i128) -> i128 {
    len - index - 1
}

fn reverse_operation(operation: &Operation, len: i128, index: i128) -> i128 {
    match operation {
        DealWithIncrements(n) => reverse_deal_with_increment(*n, len, index),
        DealIntoNewStack => reverse_deal_into_new_stack(len, index),
        Cut(n) => reverse_cut(*n, len, index),
    }
}

fn reverse_operations(operations: &Vec<Operation>, len: i128, index: i128) -> i128 {
    operations.iter()
        .rev()
        .fold(index, |acc, op| reverse_operation(op, len, acc))
}

fn parse_operations(input: &Vec<String>) -> Vec<Operation> {
    input
        .iter()
        .map(|o| o.into())
        .collect()
}

fn modinv(a: i128, n: i128) -> i128 {
    let (mut t, mut newt) = (0, 1);
    let (mut r, mut newr) = (n, a);

    while newr != 0 {
        let q = r / newr;

        let temp = t;
        t = newt;
        newt = temp - q * newt;

        let temp = r;
        r = newr;
        newr = temp - q * newr;
    }

    if r > 1 {
        panic!("{} is not invertible mod {}", a, n);
    }

    if t < 0 {
        t += n;
    }

    t
}

fn main() {
    let input = io::stdin().lock().lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    let operations = parse_operations(&input);

    let len = 119315717514047i128;
    let iterations: i128 = 101741582076661;

    let x0 = 2020;
    /* x1 = a * x0 + b */
    let x1 = reverse_operations(&operations, len, x0);
    /* x2 = a * x1 + b */
    let x2 = reverse_operations(&operations, len, x1);

    /* Solve for a and b */
    let a = (x1 - x2) * modinv(x0 - x1 + len, len) % len;
    let b = (x1 - a * x0) % len;

    /* Solution is a^iterations*x + (a^iterations-1) / (a-1) * b mod len */
    let solution = (mod_exp(a, iterations, len)*x0 + ((mod_exp(a, iterations, len)-1) * modinv(a-1, len)) % len * b) % len;
    println!("{}", solution + len);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    type Deck = Vec<i128>;

    #[test]
    fn test_reverse_deal_with_increment_01() {
        assert_eq!(reverse_deal_with_increment(3, 10, 0), 0);
        assert_eq!(reverse_deal_with_increment(3, 10, 3), 1);
        assert_eq!(reverse_deal_with_increment(3, 10, 6), 2);
        assert_eq!(reverse_deal_with_increment(3, 10, 9), 3);
        assert_eq!(reverse_deal_with_increment(3, 10, 2), 4);
        assert_eq!(reverse_deal_with_increment(3, 10, 5), 5);
        assert_eq!(reverse_deal_with_increment(3, 10, 8), 6);
        assert_eq!(reverse_deal_with_increment(3, 10, 1), 7);
        assert_eq!(reverse_deal_with_increment(3, 10, 4), 8);
        assert_eq!(reverse_deal_with_increment(3, 10, 7), 9);
    }

    #[test]
    fn test_reverse_cut_01() {
        assert_eq!(reverse_cut(3, 10, 0), 3);
        assert_eq!(reverse_cut(3, 10, 1), 4);
        assert_eq!(reverse_cut(3, 10, 2), 5);
        assert_eq!(reverse_cut(3, 10, 3), 6);
        assert_eq!(reverse_cut(3, 10, 4), 7);
        assert_eq!(reverse_cut(3, 10, 5), 8);
        assert_eq!(reverse_cut(3, 10, 6), 9);
        assert_eq!(reverse_cut(3, 10, 7), 0);
        assert_eq!(reverse_cut(3, 10, 8), 1);
        assert_eq!(reverse_cut(3, 10, 9), 2);
    }

    #[test]
    fn test_reverse_cut_02() {
        assert_eq!(reverse_cut(-4, 10, 0), 6);
        assert_eq!(reverse_cut(-4, 10, 1), 7);
        assert_eq!(reverse_cut(-4, 10, 2), 8);
        assert_eq!(reverse_cut(-4, 10, 3), 9);
        assert_eq!(reverse_cut(-4, 10, 4), 0);
        assert_eq!(reverse_cut(-4, 10, 5), 1);
        assert_eq!(reverse_cut(-4, 10, 6), 2);
        assert_eq!(reverse_cut(-4, 10, 7), 3);
        assert_eq!(reverse_cut(-4, 10, 8), 4);
        assert_eq!(reverse_cut(-4, 10, 9), 5);
    }

    #[test]
    fn test_reverse_deal_into_new_stack_01() {
        assert_eq!(reverse_deal_into_new_stack(10, 0), 9);
        assert_eq!(reverse_deal_into_new_stack(10, 1), 8);
        assert_eq!(reverse_deal_into_new_stack(10, 2), 7);
        assert_eq!(reverse_deal_into_new_stack(10, 3), 6);
        assert_eq!(reverse_deal_into_new_stack(10, 4), 5);
        assert_eq!(reverse_deal_into_new_stack(10, 5), 4);
        assert_eq!(reverse_deal_into_new_stack(10, 6), 3);
        assert_eq!(reverse_deal_into_new_stack(10, 7), 2);
        assert_eq!(reverse_deal_into_new_stack(10, 8), 1);
        assert_eq!(reverse_deal_into_new_stack(10, 9), 0);
    }

    #[test]
    fn test_reverse_example_01() {
        let operations = parse_operations(&vec![
            "deal with increment 7".into(),
            "deal into new stack".into(),
            "deal into new stack".into(),
        ]);

        let reversed: Deck = (0..10)
            .map(|i| reverse_operations(&operations, 10, i))
            .collect();

        assert_eq!(reversed.as_slice(), &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_reverse_example_02() {
        let operations = parse_operations(&vec![
            "cut 6".into(),
            "deal with increment 7".into(),
            "deal into new stack".into(),
        ]);

        let reversed: Deck = (0..10)
            .map(|i| reverse_operations(&operations, 10, i))
            .collect();

        assert_eq!(reversed.as_slice(), &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_reverse_example_03() {
        let operations = parse_operations(&vec![
            "deal with increment 7".into(),
            "deal with increment 9".into(),
            "cut -2".into(),
        ]);

        let reversed: Deck = (0..10)
            .map(|i| reverse_operations(&operations, 10, i))
            .collect();

        assert_eq!(reversed.as_slice(), &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_reverse_example_04() {
        let operations = parse_operations(&vec![
            "deal into new stack".into(),
            "cut -2".into(),
            "deal with increment 7".into(),
            "cut 8".into(),
            "cut -4".into(),
            "deal with increment 7".into(),
            "cut 3".into(),
            "deal with increment 9".into(),
            "deal with increment 3".into(),
            "cut -1".into(),
        ]);

        let reversed: Deck = (0..10)
            .map(|i| reverse_operations(&operations, 10, i))
            .collect();

        assert_eq!(reversed.as_slice(), &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_reverse_input() {
        let input: Vec<String> = fs::read_to_string("input.txt")
            .unwrap()
            .split("\n")
            .filter(|s| !s.trim().is_empty())
            .map(str::to_string)
            .collect();

        let operations = parse_operations(&input);

        assert_eq!(reverse_operations(&operations, 10007, 4485), 2019);
    }
}