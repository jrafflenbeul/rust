fn main() {
    let x = 5;
    
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);

    // Arrays

    // arrays have a fixed size and will be allocated on the stack!
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // [<type>; <size>]

    let b = [3; 5]; // [<value>; <size>]

    let first = a[0];
    let second = a[1];

    println!("The first value is {} and the second value is {}", first, second);
    println!("The length of b is: {}", b.len())
}
