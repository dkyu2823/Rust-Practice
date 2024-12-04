// immutable vs mutable through mut 
// constant(with naming convention) with const(Constants can be declared in any scope, including the global scope) vs immutable
// shadowing with let - this associated within scope and could change data type

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}