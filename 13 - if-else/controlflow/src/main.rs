fn main() {
    
    let a = 10;

    if a % 3 == 0 {
        println!("a can divided by 3 to get zero");
    } else if a % 2 == 0 {
        println!("a can divided by 2 to get zero");
    }else if a % 5 == 0 {
        println!("a can divided by 5 to get zero");
    }else{
        println!("a cannot divided by 2,3,5 to get zero");
    }

}
