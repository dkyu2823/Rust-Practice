/*
Data types(two subsets):
- scalar
- compound

Rust is a statically typed language, which means that it must know the types of all variables at compile time.

scalar - primitive type
- integers, floating-point numbers, Booleans, and characters
- integers
    - two’s complement representation
    - signed and unsigned
    - Number literals
    - overflow(assigning large value to a variable with low space) will cause panic
    - When you’re compiling in release mode with the --release flag. if overflow occurs, Rust performs two’s complement wrapping.
- floating-point numbers
    - f32 and f64
- Booleans
    - bool - one byte
- characters
    - with a single quote, as opposed to string literals.
    - char type - four bytes in size and represents a Unicode Scalar Value, which is more than ASCII
    - Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
    
Compound - can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
- tuple
    - A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    - Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    - We create a tuple by writing a comma-separated list of values inside parentheses.
    - let tup: (i32, f64, u8) = (500, 6.4, 1);
    - destructuring is available like we have in python
    - We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    - let x: (i32, f64, u8) = (500, 6.4, 1); x.0, x.1, x.2
    - The tuple without any values has a special name, unit
- The Array Type
    - Unlike a tuple, every element of an array must have the same type. 
    - Unlike arrays in some other languages, arrays in Rust have a fixed length.
    - let a = [1, 2, 3, 4, 5];
    - let a: [i32; 5] = [1, 2, 3, 4, 5];
    - let a = [3; 5]; -> let a = [3, 3, 3, 3, 3];
    - An array is a single chunk of memory of a known, fixed size that can be allocated on the stack
*/

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}