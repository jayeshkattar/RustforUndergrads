fn main() {
    let b = sum(1,2);

    println!("The sum = {}",b );
}


fn sum(a:i32,b:i32) -> i32{
    let z = a + b;
    return z*10;
}