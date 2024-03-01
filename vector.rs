// Declare vector, initialize with three values
let three_nums = vec![15, 3, 46];
println!("Initial vector: {:?}", three_nums);  
  
// Declare vector, value = "0", length = 5
let zeroes = vec![0; 5];
println!("Zeroes: {:?}", zeroes); // use {:?} only with the macro println!

// Create empty vector, declare vector mutable so it can grow and shrink
let mut fruit = Vec::new();

// Push values onto end of vector, type changes from generic `T` to String
fruit.push("Apple");
fruit.push("Banana");
fruit.push("Cherry");
println!("Fruits: {:?}", fruit); 

// Pop off value at end of vector
// Call pop() method from inside println! macro
println!("Pop off: {:?}", fruit.pop());
println!("Fruits: {:?}", fruit); 

/ Declare vector, initialize with three values
let mut index_vec = vec![15, 3, 46];
let three = index_vec[1];
index_vec[1] = index_vec[1] + 5;
println!("Vector: {:?}", index_vec);  
