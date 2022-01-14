fn main() {
    // ARRAYS
    // An array can be defined in two ways:
    // - A comma-separated list of values, where the length isn't specified.
    // - The initial value followed by a semicolon, and then the array length.
    // In both cases, the content is enclosed in square brackets [].
    
    // Declare array, initialize all values, compiler infers length = 7
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    
    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];
    
    // Read the array signature
    // At compile time, the signature of an array is defined as [T; size]:
    // - T is the data type for all elements in the array.
    // - size is a nonnegative integer that represents the array length.

    // Only one thing about an array can change over time: the values for the elements in the array.
    // The data type and number of elements (length) remain constant.
    
    // Index into an array
    // We use indexing to access the elements in an array with the expression <array>[<index>].
    
    // Set first day of week
    let first = days[0];

    // Set second day of week
    let second = days[1];
    println!("{} - {}", first, second);
    
    // Watch for out-of-bounds index values
    // Because the array length is known at compile time, Rust makes it impossible to build any
    // program that attempts to access the array with an out-ouf-bounds index.
    
    // VECTORS
    // As with arrays, vectors store multiple values that have the same data type.
    // Unlike arrays, the size or length of a vector can grow or shrink at any time.
    // The ability for the size to change over time is implied at compile time. As a
    // result, Rust can't prevent you from accessing an invalid position in your vector
    // like it does for out-of-bounds access in arrays.
    
    // Define a vector
    // When you read code in the Rust language, you'll notice the syntax <T>. This syntax
    // representes the use of a generic type T. We use a generic type declaration
    // when we don't yet know the actual data type.

    // To actually create a vector, we use a concrete type like <vector>u32, a vector of
    // type u32, or <vector>String, a vector of type String.
    
    // A common way to declare and initialize a vector is with the vec! macro. This macro
    // also accepts the same syntax as the array constructor.
    
    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec!["0"; 5];
    println!("Zeroes: {:?}", zeroes);
    
    // In this example, we use the colon question mark {:?} syntax with the println! macro.
    // Rust doesn't know how to display a vector of integers. If we try to display the vector
    // information without using special formatting, the compiler issues an error. We use the
    // empty curly parenthesis {} to help display the vector values.
    
    // Vectors can also be craeted by using the Vec::new() method. This method of vector creation
    // lets us add and remove values at the end of the vector. To support this behavior, we declare
    // the vector variable as mutable with the mut keyword.
    
    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();
    
    // Push and pop values
    // When we create a vector with the Vec::new() method, we can add and remove values at the end of the vector.
    
    // To ADD a value to the end of the vector, we use the push(<value>) method.
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);
    
    // After the type of a vector is set to a concrete type, only values of that specific type can
    // be added to the vector. If we try to add a value of a different type, the compiler returns an error.
    
    // To REMOVE the value at the end of the vector, we use the pop() method.
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);
    
    // Index into a vector
    // Vectors support indexing in the same manner as arrays.
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
    
    // Because vector values are mutable, we can change a value in place by accessing the element value with the index:
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
    

    // Watch for out-of-bounds index values
    // As with arrays, we can't access an element in a vector with an index that's not in the allowed range.
    // This type of expression for an array causes the compiler to return an error.
    // For vector, compilation passes, but the program enters an unrecoverable panic state at the expression and stops program execution.
}