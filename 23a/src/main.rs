mod int_code;

use crate::int_code::Computer;

fn main() {
    let memory = int_code::read_memory().unwrap();
    let mut computers: Vec<Computer> = (0..50)
        .map(|i| Computer::new(&memory, Some(&vec![i])))
        .collect();
    computers
        .iter_mut()
        .for_each(|c| { c.run(); } );

    'outer: loop {
        for i in 0..50 {
            while !computers[i].get_output().is_empty() {
                let j = computers[i].pop_output() as usize;
                let x = computers[i].pop_output();
                let y = computers[i].pop_output();

                if j == 255 {
                    println!("{}", y);
                    break 'outer;
                }

                computers[j].push_input(x);
                computers[j].push_input(y);
            }

            if computers[i].get_input().is_empty() {
                computers[i].push_input(-1);
            }

            computers[i].run();
        }
    }
}
