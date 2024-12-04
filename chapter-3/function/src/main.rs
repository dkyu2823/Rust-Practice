/*
the main function, which is the entry point of many programs.
Rust code uses snake case as the conventional style for function and variable names
Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

Parameters
We can define functions to have parameters, which are special variables that are part of a function’s signature. 
In function signatures, you must declare the type of each parameter.

Statements and Expressions
Rust is an expression-based language
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.
Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

Functions with Return Values
We don’t name return values, but we must declare their type after an arrow (->)
You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

*/

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}