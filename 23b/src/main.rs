mod int_code;

use crate::int_code::{Computer, Unit};

fn main() {
    let memory = int_code::read_memory().unwrap();
    let mut computers: Vec<Computer> = (0..50)
        .map(|i| Computer::new(&memory, Some(&vec![i])))
        .collect();
    computers
        .iter_mut()
        .for_each(|c| { c.run(); } );
    let mut nat: Option<(Unit, Unit)> = None;
    let mut delivered_nat: Option<(Unit, Unit)> = None;

    'outer: loop {
        if nat.is_some() && computers.iter().all(|c| c.get_output().is_empty()) {
            computers[0].push_input(nat.unwrap().0);
            computers[0].push_input(nat.unwrap().1);

            if delivered_nat.is_some() && delivered_nat.unwrap().1 == nat.unwrap().1 {
                println!("{}", nat.unwrap().1);
                break 'outer;
            }

            delivered_nat = nat;
            nat = None;
        }

        for i in 0..50 {
            while !computers[i].get_output().is_empty() {
                let j = computers[i].pop_output() as usize;
                let x = computers[i].pop_output();
                let y = computers[i].pop_output();

                if j == 255 {
                    nat = Some((x, y));
                } else {
                    computers[j].push_input(x);
                    computers[j].push_input(y);
                }
            }

            if computers[i].get_input().is_empty() {
                computers[i].push_input(-1);
            }

            computers[i].run();
        }
    }
}
