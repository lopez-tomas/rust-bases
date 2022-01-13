fn main() {
    println!("Hello, world!\n");
    
    //todo!("Use curly brackets to print values.");
    println!("The first letter of the English alphabet is \'{}\' and the last letter is \'{}\'", 'A', 'Z');

    println!("\n# Data types #\n");

    println!("Integers:");
    println!("Length\t\t\t\tSigned\t\tUnsigned\n");

    // 8-bits
    let _number_i8: i8;
    let _number_u8: u8;

    println!("8-bits\t\t\t\ti8\t\tu8");

    // 16-bits
    let _number_i16: i16;
    let _number_u16: u16;

    println!("16-bits\t\t\t\ti16\t\tu16");

    // 32-bits
    let _number_i32: i32;
    let _number_u32: u32;

    println!("32-bits\t\t\t\ti32\t\tu32");

    // 64-bits
    let _number_i64: i64;
    let _number_u64: u64;

    println!("64-bits\t\t\t\ti64\t\tu64");

    // 128-bits
    let _number_i128: i128;
    let _number_u128: u128;

    println!("128-bits\t\t\ti128\t\tu128");

    // architecture-dependent
    let _number_isize: isize;
    let _number_usize: usize;

    println!("architecture-dependent\t\tisize\t\tusize");

    println!("\nDecimal:");
    let _number_f32: f32;
    let _number_f64: f64;

    println!("32-bits\t\t f32");
    println!("64-bits\t\t f64");

    println!("\nBooleans:");
    let is_true: bool = true;
    let is_false: bool = false;
    let is_greater: bool = 1 > 4;

    println!("true\t\t {}", is_true);
    println!("false\t\t {}", is_false);
    println!("1 > 4 ?\t\t {}", is_greater);

    println!("\nCharacters and strings:");
    let uppercase_s: char = 'S';
    let lowercase_f: char = 'f';
    let smiley_face: char = '😊';
    let string: &str = "Text...";

    println!("char\t\t\'{}\'\t\'{}\'\t\'{}\'", uppercase_s, lowercase_f, smiley_face);
    println!("&str\t\t\"{}\"", string);
}