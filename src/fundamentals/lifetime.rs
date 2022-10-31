pub fn run() {
    let fn1 = demo();
    println!("{}", fn1);

    let fn2 = demo2();
    println!("{}", fn2);

    let fn3 = demo3("something");
    println!("{}", fn3);

    fn demo() -> String {
        String::from("heyo!")
    }

    fn demo2() -> &'static str {
        "heyo"
    }

    fn demo3(s:&str) -> usize {
        s.len()
    }
}