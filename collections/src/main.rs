fn main() {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    {
        let v4 = vec![1, 2, 3, 4];

        // do stuff with v4
    } // <- v4 goes out of scope and is freed here

    let v5 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v5[2]; // return reference when accessing with index
    println!("The third element is {}", third);

    match v5.get(2) { // .get() returns Option<T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    let does_not_exist_ref = &v5[100]; // causes panic
    let does_not_exist_opt = v5.get(100); // returns None

    // Can't have mutable and immutable references in the same scope
    let mut v6 = vec![1, 2, 3, 4, 5]; // mutable

    let first = &v6[0]; // immutable

    // v6.push(6); // -> cannot borrow `v6` as mutable because it is also borrowed as immutable

    /* Adding a new element onto the end of the vector
    might require allocating new memory and copying
    the old elements to the new space, if there isn’t
    enough room to put all the elements next to each
    other where the vector currently is. In that case,
    the reference to the first element would be pointing
    to deallocated memory. */

    println!("The first element is: {}", first);

    // iterating with immutable references
    let v7 = vec![100, 32, 57];
    for i in &v7 {
        println!("{}", i);
    }

    // iterating with mutable references and the dereference operator *
    let mut v8 = vec![100, 32, 57];
    for i in &mut v8 {
        *i += 50;
    }

    // using enum

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    // Strings
    let mut s1 = String::new();

    let data = "initial contents";
    let s2 = data.to_string();

    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();

    let s4 = String::from("initial contents");

    // push
    let mut s5 = String::from("foo");
    s5.push_str("bar");

    let mut s6 = String::from("foo");
    let s7 = "bar";
    s6.push_str(s7);
    println!("s7 is {}", s7); // if push_str took ownership of s7, printing wouldn't be possible

    // .push() only takes a single character
    let mut s8 = String::from("lo");
    s8.push('l');

    // concatenation
    let s9 = String::from("Hello, ");
    let s10 = String::from("world!");
    let s11 = s9 + &s10; // s9 has been moved here and can no longer be used

    /* The reason s1 is no longer valid after the addition
    and the reason we used a reference to s2 has to do
    with the signature of the method that gets called
    when we use the + operator. The + operator uses the
    add method, whose signature looks something like this:

    fn add(self, s: &str) -> String { 
        
    s9 has been moved, because of self => no ref! */

    /* The reason we’re able to use &s2 in the call to add
    is that the compiler can coerce the &String argument
    into a &str. 
    
    deref coercion: &s2 -> &s2[..] */

    let s12 = String::from("tic");
    let s13 = String::from("tac");
    let s14 = String::from("toe");

    // let s15 = s12 + "-" + &s13 + "-" + &s14;

    // format! macro returns String and uses references, therefore no ownership is taken
    let s16 = format!("{}-{}-{}", s12, s13, s14);

    // slicing
    let hello = "Здравствуйте";
    let s17 = &hello[0..4];

    // s17 will be Зд, since cyrillic characters have a size of 2 bytes each
    // &hello[0..1] -> panic!

    // iterating Strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

    // hash maps -> basically Objects!
    use std::collections::HashMap;

    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // hash maps and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, because of move

    let team_name = String::from("Blue");
    let score = scores2.get(&team_name); // returns Option<&V>

    // iterating
    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    // overwriting a value
    scores2.insert(String::from("Blue"), 25);

    // insert if key has no value
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);

    // updating value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
