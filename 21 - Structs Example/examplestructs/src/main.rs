struct Square {
    side: u32,
}

fn main() {
    
    let square1 = Square{
        side: 10,
    };

    let area = area(&square1);

    println!("Area of Square is {}", area);

}

fn area(square: &Square) -> u32 {
    square.side*square.side
}