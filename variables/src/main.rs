fn shadowing_vs_mutable() {
    // We can declare a variable and then re-use its name with a different type
    let characters = "abcde";           // String
    let characters = characters.len();  // Number
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        // Withing this inner scope, the value of x is x * 2
        // but outside the value continues to be x + 1
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    let x = 5;
    let mut y = 1;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // This throws an error because x is not declared mutable
    // x = 6;
    // We can change the value of y because we declared it mutable
    y = 2
    println!("The value of y is: {y}");
}
