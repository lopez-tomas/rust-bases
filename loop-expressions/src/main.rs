use std::collections::HashMap;
/* 
The `use` command brings the `HashMap` definition from the `collection`
portion of the Rust standard library into scope for our program. This
syntax is similar to what other programming languages call an `import`.
*/

fn main() {
    // ## Work with hash maps ##
    /* 
    Another common collection type in Rust is the hash map.
    The HashMap<K, V> type stores data by mapping each key K with its value V.
    */


    // # Define a hash map #
    /*
    We create an empty hash map with the `HashMap::new method`.
    We declare the reviews variable as mutable so we can add or
    remove keys and values, as needed.
    */
    let mut reviews: HashMap<String, String> = HashMap::new();


    // # Add a key-value pair #
    /*
    We add elements to the hash map by using the insert(<key>, <value>)
    method. In the code, the syntax is <hash_map_name>.insert():
    */
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));


    // # Get a key value #
    /*
    After we add data to our hash map, we can get a specific value for a key with the get(<key>) method.
    */
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    /* Note
    Notice the output displays the book review as "Some("Great examples.")"
    rather than just "Great examples." Because the get method returns
    an Option<&Value> type, Rust wraps the result of the method call
    with the "Some()" notation.
    */


    // # Remove a key-value pair #
    /*
    We can remove entries from a hash map by using the .remove() method.
    If we use the get method for an invalid hash map key, the get
    method returns "None."
    */
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));


    // ## Use `for`, `whil`, and `loop` expressions ##
    /*
    Rust offers three loop expressions to make a program repeat a block of code:
        - loop: Repeat, unless a manual stop occurs.
        - while: Repeat while a condition remains true.
        - for: Repeat for all values in a collection.
    */


    // # Loop expression
    println!("\n\nLoop loop:\n");
    /*
    The loop expression creates an infinite loop. This keyword lets
    us repeat the actions in the expression body continuously. 
    The actions repeat until we take some direct action to make
    the loop stop.
    */

    /* Infinite loop printing "We loop forever!"
    loop {
        println!("We loop forever!");
    }
    */

    // # Stoping a loop expression
    /*
    The most common way to stop a loop expression is by using the
    `break` keyword to set a break point:
    */
    loop {
        // Keep printing, printing, printing...
        println!("We loop forever!");
        // On the other hand, maybe we should stop!
        break;
    }

    /*
    The following example shows how we can use the break keyword
    in a loop expression to also return a value:
    */
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);

    /*
    The loop expression body can have more than one break point.
    When the expression has multiple break points, every break point
    must return a value of the same type. All values must be of type
    integer, or String, or bool, and so on. When a break point doesn't
    explicitly return a value, the program interprets the expression
    result as an empty tuple, ().
    */
    

    // # Loop a while
    println!("\n\nWhile loop:\n");
    /*
    The while loop uses a conditional expression. The loop repeats as
    long as the conditional expression remains true.
    */
    counter = 1;
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }
    

    // # Loop for these values
    println!("\n\nFor loop:\n");
    /*
    The `for` loop uses an iterator to process a collection of items.
    The loop repeats the actions in the expression body for each
    item in the collection. This type of loop repetition is called
    iterating. When all iterations are complete, the loop stops.

    In Rust, we can iterate over any collection type, such as an
    array, vector, or hash map. Rust uses an iterator to move through
    each item in the collection from first to last.
    
    We access the items in the collection by using the iter() method.
    The for expression binds the current value of the iterator to
    the result of the iter() method. In the expression body, we can
    work with the iterator value.
    */
    println!("Using iter()");
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    
    /*
    Another easy way to create an iterator is to use the range
    notation `a..b`. The iterator starts at the `a` value and
    continues through to `b` in steps of one, but it doesn't use the
    value `b`.
    */
    println!("\nUsing a..b notation");
    for number in 0..5 {
        println!("{}", number * 2);
    }
}