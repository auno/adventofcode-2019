mod int_code;

use int_code::Computer;

fn main() {
    let memory = int_code::read_memory().unwrap();

    let mut count = 0;

    for x in 0..50 {
        for y in 0..50 {
            let mut computer = Computer::new(&memory, None);
            computer.push_input(y);
            computer.push_input(x);
            computer.run();

            if computer.pop_output() == 1 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
