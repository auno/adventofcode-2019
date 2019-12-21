use crate::int_code::Computer;

mod int_code;

fn main() {
    let memory = int_code::read_memory().unwrap();
    let mut computer = Computer::new(&memory, None);

    computer.println("NOT A J".into());
    computer.println("NOT B T".into());
    computer.println( "OR T J".into());
    computer.println("NOT C T".into());
    computer.println( "OR T J".into());
    computer.println("AND D J".into());
    computer.println("NOT J T".into());
    computer.println("OR H T".into());
    computer.println("OR E T".into());
    computer.println("AND T J".into());
    computer.println("RUN".into());

    computer.run();

    for value in computer.get_output() {
        if *value < 128 {
            print!("{}", (*value as u8) as char);
        } else {
            eprintln!("{}", value);
        }
    }

}
