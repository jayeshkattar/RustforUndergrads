enum IPVariants {
    IPV4(String),
    IPV6(String),
}

enum Message {
    Write(String),
    Color(i32,i32,i32),
    Move{x:i32,y:i32},
    Quit,
}

impl Message {
    fn call(&self){
        println!("I'm inside the implementation");
    }
}

fn main() {
    
    let IP1 = IPVariants::IPV4(String::from("192.168.0.1"));
    let IP2 = IPVariants::IPV6(String::from("::1"));

    let variable = Message::Write(String::from("Hi!"));
    variable.call();


}
