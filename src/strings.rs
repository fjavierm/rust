// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // str
    let hello = "Hello";
    let mut hello1 = String::from("Hello ");

    // Get length
    println!("Lenght: {} or {}", hello.len(), hello1.len());

    // Push a char
    hello1.push('W');

    // Push a string
    hello1.push_str("orld!");

    // Capacity in byte
    println!("Capacity: {}", hello1.capacity());

    // Check if empty
    println!("Is empty: {}", hello1.is_empty());

    // Contains
    println!("Contains 'World' {}", hello1.contains("World"));

    // Replace
    println!("Replace  {}", hello1.replace("World", "there"));

    // Loop through string by whitespace
    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    println!("{} or {}", hello, hello1);

    // Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assetion testing
    assert_eq!(2, s.len());
    // assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}