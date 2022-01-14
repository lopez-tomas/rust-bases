// fn: The function declaration keyword in Rust.
// goodybye: The function name.
// (message: &str): The function's argument or parameter list. One pointer to string data is expected as the input value.
// -> bool: The arrow points to the type of value this funciton will always return.
fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // Return early
        return 0; // When you explicitly use the return keyword, you end the statement with a semicolon.
    }
    num / 5 // If you send back a return value without using the return keyword, you don't end the statement with a semicolon.
}

fn main() {
    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);
    
    let num: u32 = 25;
    println!("\n\nReturning a value:");
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}