// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    //transmission: Transmission,
    roof: bool,
    //convertible: bool,
    age: (Age, u32),
    //mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic
}

// Previous main()
//fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    //let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    //println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    //car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    //println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    //car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    //println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
//}

fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car;
    let mut engine = Transmission::Manual;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[2]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[0]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality;
    if miles != 0 {
        quality = (Age::Used, miles);
    } else {
        quality = (Age::New, 0);
    }

    // Return the completed tuple to the caller
    quality
}