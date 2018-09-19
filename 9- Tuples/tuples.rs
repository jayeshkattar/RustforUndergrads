fn main() {
    let tup:(i32,i32,i32) = (1,2,3);

    let (x,y,z) = tup;

    let a = tup.0;

    println!("A = {}",a);

    println!("X = {} , Y = {} and Z = {}",x,y,z);
}