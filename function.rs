fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // Return early
        return 0;
    }
    num / 5 //use the return or the term has to be of the same type of the return type and without semicolon
}

fn main() {
    let formal = "Formal: Goodbye.";
    goodbye(formal);
    let numero = 5;
    pri
