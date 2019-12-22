use std::io;
use std::io::BufRead;

type Deck = Vec<u32>;

fn deal_with_increment(deck: &Deck, increment: usize) -> Deck {
    let mut new_deck: Deck = Vec::new();
    new_deck.resize(deck.len(), 0);

    for i in 0..deck.len() {
        new_deck[i * increment % deck.len()] = deck[i];
    }

    new_deck
}

fn deal_into_new_stack(deck: &Deck) -> Deck {
    let mut deck = deck.clone();
    deck.reverse();
    deck
}

fn cut(deck: &Deck, n: i32) -> Deck {
    let n: usize = ((n + deck.len() as i32) % deck.len() as i32) as usize;
    let parts = deck.split_at(n);
    let deck = [parts.1, parts.0].concat();

    deck.clone()
}

fn do_line(deck: &Deck, line: &String) -> Deck {
    let line = line.trim();

    if line == "deal into new stack" {
        return deal_into_new_stack(deck);
    }

    if line.starts_with("deal with increment ") {
        let n = line.split(" ").last().unwrap().parse().unwrap();
        return deal_with_increment(deck, n);
    }

    if line.starts_with("cut ") {
        let n = line.split(" ").last().unwrap().parse().unwrap();
        return cut(deck, n);
    }

    unreachable!("Unknown technique: {}", line);
}

fn do_lines(deck: &Deck, lines: &Vec<String>) -> Deck {
    lines
        .iter()
        .fold(deck.clone(), |acc, line| do_line(&acc, line))
}

fn main() {
    let input = io::stdin().lock().lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    let deck: Deck = (0..10007).collect();
    let shuffled_deck = do_lines(&deck, &input);

    let pos = shuffled_deck
        .iter()
        .enumerate()
        .find_map(|(i, v)| match v {
            2019 => Some(i),
            _ => None
        })
        .unwrap();

    println!("{}", pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_01() {
        let deck: Deck = (0..10).collect();
        let deck = do_lines(&deck, &vec![
            "deal with increment 7".into(),
            "deal into new stack".into(),
            "deal into new stack".into(),
        ]);

        assert_eq!(deck.as_slice(), &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_example_02() {
        let deck: Deck = (0..10).collect();
        let deck = do_lines(&deck, &vec![
            "cut 6".into(),
            "deal with increment 7".into(),
            "deal into new stack".into(),
        ]);

        assert_eq!(deck.as_slice(), &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_example_03() {
        let deck: Deck = (0..10).collect();
        let deck = do_lines(&deck, &vec![
            "deal with increment 7".into(),
            "deal with increment 9".into(),
            "cut -2".into(),
        ]);

        assert_eq!(deck.as_slice(), &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_example_04() {
        let deck: Deck = (0..10).collect();
        let deck = do_lines(&deck, &vec![
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

        assert_eq!(deck.as_slice(), &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
