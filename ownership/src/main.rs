fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // move

    // fixed size (stack) -> copies value
    let x = 5;
    let y = x;

    // no fixed size (heap) -> copies pointer, length & capacity
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); => doesn't work in order to prevent double-freeing memory! Use s2 instead.

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);
}
