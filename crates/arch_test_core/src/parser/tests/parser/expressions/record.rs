enum Test1 {
    A {
        a: u32,
        b: u32,
        c: u32
    }
}

fn main() {
    let test = Test1::A {
        a: 1,
        b: 2,
        c: 3
    };
    let Test1::A { a, b, c } = test;
}