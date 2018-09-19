fn main() {
    println!("Hello, world!");
    main1();
    main2(2);
}

fn main1(){
    println!("I'm main1");
}

fn main2(a:i32){
    println!("What value am I getting = {}",a );
}