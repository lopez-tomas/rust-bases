#[derive(Debug)]
struct DivisionByZeroError;
/*
The #[derive(Debug)] part that precedes the DivisionByZeroError
struct is a macro that tells the Rust compiler to make the type
printable for debugging purposes. We'll cover this concept in depth
later, in the Traits module.
*/

fn main() {
    // # Fatal errors with `panic!` #
    /*
    Panicking is the simplest error handling mechanism in Rust.

    You can use the `panic!` macro to panic the current thread. It
    prints an error message, frees resources, and then exits the program.
    */
    //panic!("Farewell");
    
    /*
    In general terms, you should use panic! when a program reaches
    an unrecoverable state meaning anything where there is
    absolutely no way to recover from the error.

    Rust panics on some operations such as a division by zero or an
    attempt to access an index that isn't present in an array, a
    vector, or a hash map, as shown in the following code:
    */
    let _v = vec![0, 1, 2, 3];
    //println!("{}", v[6]); // this will cause a panic!
    

    // # Use the Option type to deal with absence
    println!("\nUsing Option type:\n");
    /*
    The Rust standard library provides an Option<T> enum to be used
    when the absence of a value is a possibility. Option<T> is widely
    used in Rust code. It's useful for working with values that might
    exist or that might be empty.

    In many other languages this would be modeled using null or nil,
    but Rust doesn't use null outside of code that interoperates with
    other languages. This means Rust is explicit about when a value
    is optional. While in many languages, a function that takes a
    String might actually take either a String or null, in Rust, that
    same function can only take actual Strings. If you want to model
    an optional string in Rust, you need to explicitly wrap it in an
    Option type: Option<String>.

    Option<T> manifests itself as one of two variants:
    */
    /*
    enum Option<T> {
        None,       // The value doens't exit
        Some(T),    // The value exists
    }
    */

    /*
    The <T> part of the Option<T> enum declaration states that the
    type T is generic and will be associated with the Some variant
    of the Option enum.
    */
    
    /*
    In the preceding unit, we mentioned that trying to access a
    vector's non-existent index would cause the program to panic,
    but you could avoid that by using the Vec::get method, which
    returns an Option type instead of panicking. If the value exists
    at a specified index, it's wrapped in the Option::Some(value)
    variant. If the index is out of bounds, it would return a
    Option::None value instead.
    */
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
    

    // # Pattern matching #
    println!("\n\nUsing `match` operator:\n");
    /*
    There's a powerful operator in Rust that's called match. You can
    use it to control the flow of your program by providing patterns.
    When match finds a matching pattern, it runs code that you supply
    with that pattern.
    */
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
    
    /*
    Note that when the string value is "coconut", the first arm is
    matched and then used to determine the flow of execution.

    Whenever you use the match expression, keep the following rules
    in mind:
        - match arms are evaluated from top to bottom. Specific
        cases must be defined earlier than generic cases or they'll
        never be matched and evaluated.
        - match arms must cover every possible value that the input
        type could have. You'll get a compiler error if you try to
        match against a non-exhaustive pattern list.
    */


    // # The `if let` expression #
    print!("\n\nUsing `if let`:\n");
    /*
    Rust offers a convenient way to test whether a value conforms with
    a single pattern.

    In the following example, the input to match is an Option<u8>
    value. The match expression should only run code if that input
    value is 7.
    */
    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {},
    }

    /*
    In this case, we'd like to ignore the None variant and all
    Some<u8> values that don't match Some(7). Wildcard patterns are
    useful for this type of situation. You can add the _ (underscore)
    wildcard pattern after all other patterns to match anything else,
    and it's used to satisfy the compiler demands for exhausting
    match arms.

    To condense this code, you can use an if let expression:
    */
    let b_number: Option<u8> = Some(7);
    if let Some(7) = b_number {
        println!("That's my lucky number!");
    }
    /*
    An if let operator compares a pattern with an expression. If the
    expression matches the pattern, the if block is executed. The
    nice thing about if let expressions is that you don't need all
    the boilerplate code of a match expression when you're interested
    in a single pattern to match against.
    */
    

    // # Use `unwrap` and `expect`
    println!("\n\nUsing `unwrap` & `expect` methods:\n");
    /*
    You can try to access the inner value of an Option type directly
    by using the unwrap method. Be careful, though, because this
    method will panic if the variant is a None.
    */
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");
    println!("{:?}", gift.unwrap());

    let empty_gift: Option<&str> = None;
    //assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!
    
    /*
    The expect method does the same as unwrap, but it provides a
    custom panic message that's provided by its second argument:
    */
    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    let b: Option<&str> = None;
    //b.expect("fruits are healthy"); // panics with `fruits are healthy`

    /*
    Because these functions might panic, we don't recommend using
    them. Instead, consider either of the following approaches:
        - Use pattern matching and handle the None case explicitly.
        - Call similar non-panicking methods, such as unwrap_or,
        which returns a default value if the variant is None or the
        inner value if the variant is Some(value).
    */
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    println!("{:?}", a.unwrap_or("Variant is `None`"));

    assert_eq!(None.unwrap_or("cat"), "cat");
    println!("{:?}", b.unwrap_or("Variant is `None`"));
    

    // # Result type to handle errors #
    println!("\n\nResult type to handle errors:\n");
    /*
    Rust provides the Result<T, E> enum for returning and
    propagating errors. By convention, the Ok(T) variant represents
    a success and contains a value, and the variant Err(E)
    represents an error and contains an error value.

    The Result<T, E> enum is defined as:
        enum Result<T, E> {
            Ok(T):  // A value T was obtained.
            Err(E): // An error of type E was encountered instead.
        }
    */
    
    /*
    In contrast to the Option type, which describes the possibility
    of the absence of a value, the Result type is best suited
    whenever failures might occur.

    The Result type also has the unwrap and expect methods, which do
    either of the following:
        - Return the value inside the Ok variant, if this is the case.
        - Cause the program to panic, if the variant is an Err.

    Let's see Result in action. In the following example code,
    there's an implementation for a safe_division function that
    returns either of following:
        - A Result value with an Ok variant that carries the result
        of a successful division.
        - An Err variant that carries a struct DivisionByZeroError
        which signals an unsuccessful division.
    */
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}


fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}