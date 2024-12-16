/*
Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

Memory management
- Stack(Last In First Out) vs Heap(First In First Out)

Onwership rules
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.


Variables and Data Interacting with Move
- example 1 and 2

Scope and Assignment
- example 3

Variables and Data Interacting with Clone
- example 4

Stack-Only Data: Copy
- example 5
that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.

Here are some of the types that implement Copy:
- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

Ownership and Functions
- example 6
The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. Listing 4-3 has an example with some annotations showing where variables go into and out of scope.


Return Values and Scope


*/

fn main() {
    println!("Hello, world!");

    
// Example 1
// When s comes into scope, it is valid.
// It remains valid until it goes out of scope.
    {                           // s is not valid here, it’s not yet declared
        let s = "hello";  // s is valid from this point forward

                                // do stuff with s
    }                           // this scope is now over, and s is no longer valid


// Example 2 
// Rust do shallow copy(NO - It's move instead of shallow copy)
// To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:

//If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2

//In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!"); // this won't work


// Example 3 - prints "ahoy, world!"
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");


// Example 4 - using clone to do deep copy
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");

// Example 5 - we don’t have a call to clone, but x is still valid and wasn’t moved into y.

let x = 5;
let y = x;

println!("x = {x}, y = {y}");


// Example 6 
let s = String::from("hello");  // s comes into scope

takes_ownership(s);        // s's value moves into the function...
                                        //... and so is no longer valid here

let x = 5;                        // x comes into scope

makes_copy(x);           // x would move into the function,
                                       // but i32 is Copy, so it's okay to still
                                       // use x afterward


// Example 7
// 
let s1 = gives_ownership();         // gives_ownership moves its return
// value into s1

let s2 = String::from("hello");     // s2 comes into scope

let s3 = takes_and_gives_back(s2);  // s2 is moved into
// takes_and_gives_back, which also
// moves its return value into s3
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

// Example 8 - Rust does let us return multiple values using a tuple, as shown in Listing 4-5.

let s1 = String::from("hello");

let (s2, len) = calculate_length(s1);

println!("The length of '{s2}' is {len}.");


}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
