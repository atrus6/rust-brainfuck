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
    let mut left: ~[uint] = ~[0];

    while ip < str::len(p.program) {
        let str::CharRange {ch, next} = str::char_range_at(p.program, ip);
        //println(fmt!("%c at %i", ch, ip as int));
        match ch {
            '>' => {
                //if(data_pointer == p.data.len()-1) {
                //    p.data.push(0)
                //}
                 data_pointer += 1;
                // println(fmt!("Stack size: %i \nData Pointer: %i", p.data.len() as int, data_pointer as int));
             },
            '<' => {
                data_pointer -= 1;
             },
            '+' => {
                p.data[data_pointer] += 1;
             },
            '-' => {
                p.data[data_pointer] -= 1;
             },
            '.' => print(fmt!("%c", p.data[data_pointer] as char)),
            ',' => {
                let c = io::stdin().read_char() as int;
                if(c != '\n' as int) {
                    p.data[data_pointer] = c;
                }
             }
            '[' => {
                left.push(ip);
             },
            ']' => {
                if(p.data[data_pointer] != 0) {
                   ip = *left.last();
                } else {
                    left.pop();
                }
             },
            _   => println("Ignored")
        }

        ip += 1;
    }
}

fn main() {
    // Hello World
    //let mut u1 = Program::new(~"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    //let mut u2 = Program::new(~",+[-.,+]");
    //let mut u3 = Program::new(~"++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]+++++[>+++++++<<++>-]>.<<.");
    //Loop Test
    let mut u4 = Program::new(~"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]\"A*$\";?@![[#>>+<<]>[>>]<<<<[>++<[-]]>.>.");
    //Reverse INput
    let mut u5 = Program::new(~">,[>,]<[.<]");

    
    execute(u5);
}
