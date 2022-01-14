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
}