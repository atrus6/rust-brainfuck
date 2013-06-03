struct Program {
    program: ~str,
    data: ~[int],
}

impl Program {
    fn new(program: ~str) -> Program {
        Program{program:program, data:~[0, ..30000]}
    }
}

fn execute(mut p: Program) {
    let mut data_pointer = 0u;
    let mut ip = 0u;
    let mut left = 0u;

    while ip < str::len(p.program) {
        let str::CharRange {ch, next} = str::char_range_at(p.program, ip);

        match ch {
            '>' => {
                if(data_pointer == p.data.len()) {
                    p.data.push(0)
                }
                 data_pointer += 1;
             },
            '<' => {
                if(data_pointer > 0) {
                    data_pointer -= 1;
                }
             },
            '+' => {
                p.data[data_pointer] += 1;
             },
            '-' => {
                p.data[data_pointer] -= 1;
             },
            '.' => print(fmt!("%c", p.data[data_pointer] as char)),
            ',' => println("Reading Not Supported"),
            '[' => {
                left = ip-1;
             },
            ']' => {
                if(p.data[data_pointer] != 0) {
                   ip = left;
                }
             },
            _   => println("Ignored")
        }

        ip += 1;
    }
}

fn main() {
    let mut u1 = Program::new(~"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    execute(u1);
}
