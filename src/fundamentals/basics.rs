pub fn run() {
    // variables();
}

fn variables() {
    let name = "Sunny"; //Cannot change as they are immutable by default
    let mut age = 29; //Can potentially change in the future
    println!("{name} is {age} years old..");

    println!(".. one year goes by");
    age = 30;
    println!("{name} is {age} years old now..");

    const PROFESSION: &str = "Everything!";
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{PROFESSION}, {THREE_HOURS_IN_SECONDS}");

    //Shadowing
    let x = 10;
    let x = x + 2;
    {
        let x = x * 2;
        println!("Inside scope: {x}");
    }

    println!("Outside scope: {x}");
}