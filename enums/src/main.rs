fn main() {
    println!("Enums:");
    println!("Enums are types that can be any one of several variants. What Rust calls enums are more commonly known as \'algebraic data types\'.");
    
    // Define an enum
    enum _WebEvent {
        // An enum variant can be like a unit struct without fields or data types
        WELoad,
        // ... a tuple struct with data types but no named fields
        WEKeys(String, char),
        // ... a classic struct with named fields and their data types
        WEClick { x: i64, y: i64 }
    }
    
    println!("\nAny function that uses a variant of the WebEvent enum must accept all the variants in the enum. We can't have a function that accepts only the WEClick variant, but not the other variants.");
    
    // Define an enum with structs
    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick { x: i64, y: i64 }
    
    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    #[derive(Debug)]
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
    
    // Instantiate an enum
    // To access the specific variant in the enum definition, we use the syntax <enum>::<variant> with double colons ::

    // #1: Simple variant: WELoad(bool)
    let we_load = WebEvent::WELoad(true);
    
    // #2: Struct variant: WEClick(MouseClick)

    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("\nMouse click location: {}, {}", click.x, click.y);
    
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    
    // #3: Tuple variant: WEKeys(KeyPress)
    
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
    
    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\n\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}
