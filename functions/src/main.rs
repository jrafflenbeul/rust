fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    // Expressions
    let y = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons -> return
    };
    println!("The value of y is: {}", y);

    // Functions with return values
    let x = five();
    println!("The value of x is: {}", x);

    let plus_one = plus_one(5);
    println!("The value of plus_one is: {}", plus_one)
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is: {}{}", x, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}