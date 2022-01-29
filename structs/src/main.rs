struct Boy{
    name: String,
    age: i32
}

impl Boy {
    fn shout() -> String{
        return "woaaah!!!!!".to_string();
    }
}

struct  GenricCar<T>{
    speed: T,
    inetia: T
}

impl <T> GenricCar<T>{
    fn drive(&self, time: T) -> T{
        time * self.speed + self.inetia
    }
}
fn main() {
    println!("Hello, world!");
}
