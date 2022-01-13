fn main() {
    println!("Tuples:");
    println!("A tuple is a grouping of values of different types collected into one compound value.");
    println!("The individual values in a tuple are called \'elements\'.");
    println!("The values are specified as a comma-separated list enclosed in parentheses (<value>, <value>, ...)");
    println!("\nA typle has a fixed length, which is equal to its number of elements. After a tuple is declared, it can't grow or srink in size.");
    
    // Tuple of length 3
    let tuple_e = ('E', 5i32, true);
    println!("\n\nTo acces the elements in the tuple we use the syntax <tuple>.<index> where index starts at 0");

    println!("\nIs {} the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);
    println!("\n\n");

    println!("Structs:");
    println!("A struct is a type that's composed of other types. The elements in a struct are called \'fields\'.");
    println!("Like tuples, the fields in a struct can have different data types.");
    println!("A significant benefit of the struct type is that you can name each field so it's clear what the value means.");
    
    println!("\nRust supports three struct types: classic structs, tuple structs, and unit structs. These struct types support different ways to group and work with the data.");

    println!("\n\t- Classic C structs are the most commonly used. Each field in the struct has a name a a data type.");
    
    println!("\n\t- Tuple structs are similar to classic structs, but the fields don't have names.");
    
    println!("\n\t- Unit structs are most commonly used as markers.");
    
    // Classic struct with named fields
    struct Student { 
        name: String,
        level: u8,
        remote: bool
    }
    
    let student_1 = Student { 
        name: String::from("Tomas Lopez"),
        level: 23,
        remote: true
    };
    println!("\n\nWe use \'String::from(&str)\' to convert a string literal to a String type");
    println!("\n\nstudent_1 info:");
    println!("Name: {}\tLevel: {}\tRemote: {}", student_1.name, student_1.level, student_1.remote);
    
    // Tuple struct with data types only
    struct Grades(
        char,
        char,
        char,
        char,
        f32
    );
    
    let grades_student_1 = Grades (
        'A',
        'A',
        'C',
        'F',
        5.33
    );
    println!("\n\nGrades of student_1:");
    println!("{}\t{}\t{}\t{}\t\t{:?}", grades_student_1.0, grades_student_1.1, grades_student_1.2, grades_student_1.3, grades_student_1.4);
    
    // Unit struct
    struct _Unit;
}