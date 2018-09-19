fn main() {
    
    let s = String::from("Hello, World!");

    let hello = &s[0..6];

    println!("Sliced String = {}",hello);

}
