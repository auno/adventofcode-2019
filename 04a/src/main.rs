fn to_digits(num: i32) -> Vec<i32> {
    let mut num = num;
    let mut digits = Vec::new();

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits.reverse();
    digits
}

fn main() {
    let lower = 264793;
    let upper = 803935;

    let num_satisfying = (lower..upper)
        .map(|num| to_digits(num))
        .filter(|digits| digits.windows(2).filter(|digit_pair| digit_pair[0] == digit_pair[1]).count() > 0)
        .filter(|digits| digits.windows(2).filter(|digit_pair| digit_pair[0] > digit_pair[1]).count() == 0)
        .count();

    println!("{}", num_satisfying);
}
