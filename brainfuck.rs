struct Program {
    data: ~str
}

impl Program {
    fn new(data: ~str) -> Program {
        Program{data:data}
    }

    fn execute(&self) -> ~str {
        let mut i = 0u;

        while i < str::len(self.data) {
            let str::CharRange {ch, next} = str::char_range_at(self.data, i);
            io::println(fmt!("%c", ch));
            i = i+1;
        }

        return self.data.clone();
    }
}

fn main() {
    let u1 = Program::new(~"Test");
    u1.execute();
}
