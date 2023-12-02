fn main(){

    //define a 32 bit integer without sign("u")
    let number_1: u32 = 14;

    //define a 32 bit integer with sign("i")
    let number_2: i32 = -12; 

    let number_3: u32 = 16;

    let number_64 = 4.0;      // compiler infers the value to use the default type f64
    let number_32: f32 = 5.0; // type f32 specified via annotation

    println!("The number is {}.", number_1);
    println!("The number is {}.", number_2);
    println!("The number is {}.", number_64);
    println!("The number is {}.", number_32);

        // Addition, Subtraction, and Multiplication
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    let is_bigger = number_1 > number_3;
    println!("Is {} > {}? {}",number_1,number_3, is_bigger); 

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str. With &str the value can't change"
    let string_2: &str = "ace";
    //string_2 = "ciao"; we can't do it

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);

}
