// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    age: (Age,u32)
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
// Corrected code: Enum definition uses commas to separate values
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age{
    New,
    Used
}
// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the age ("New" or "Used") and miles
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    // Corrected code: Define "quality"
    // - Set the value to a "New" car
    // - Set the mileage using the "miles" input argument
    let quality = (Age::New, miles);

    // Corrected code: Return tuple, no need for "return" keyword or semicolon
    quality
}

// Build a new "Car" using the values of three input arguments
// - Color of the car (String)
// - Transmission type (enum)
// - Convertible (boolean, true if the car is a convertible)
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool,miles: u32) -> Car {

    // Create a new "Car" instance with requested characteristics
    // - Corrected code: return a "Car" struct
    // - Bind first three fields to value of corresponding input argument
    // - Set mileage to 0
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: (Age,u32),
    }
}

fn main() {
    // Create car color array
    let colors = todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

    // Declare the car type and initial values
    // Corrected code: Declare "car" as mutable "Car" struct
    // Corrected code: Declare "engine" as mutable "Transmission" enum, initialize to "Manual"
    let mut car: Car;
    let mut engine = Transmission::Manual;

     // Order 3 cars, one car for each type of transmission
    // Corrected code: Index into `colors` array and vary color for the orders
    
    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
    
    // Car order #2: New, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: New, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}

    
  
