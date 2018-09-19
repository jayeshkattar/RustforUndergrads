struct Rectangle {
    lenght: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.lenght*self.width
    }
}

fn main() {

    let rectangle1 = Rectangle{
        lenght: 20,
        width: 10
    };

    println!("Area of Rectangle= {}", rectangle1.area() );

}
