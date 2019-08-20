#[derive(Debug)]
struct Person<'a>{
    name : &'a str,
    age : u8,
}

pub fn students(){
    let name = "Vaibhav";
    let age = 23;
    let s1 : Person = Person{ name, age};

    println!("{:?}", s1);

}

//#[derive(Debug)]
struct Point{
    x : f32,
    y : f32,
}

//#[derive(Debug)]
//struct Pair(f32, f32);


pub fn rectangle(x : f32, y : f32){
    let point = Point { x : x, y : y};
    println!("{:?}, {}", point.x, point.y);

    let new_point = Point {x : 0.1, ..point};
    println!("{}, {}", new_point.x, new_point.y);
}

pub fn make_line(){
    let point = Point { x : 3.2, y : 1.6};
    let new_point = Point {x : 0.1, ..point};

    println!("{}", line(point, new_point));
}

fn line(p1 : Point , p2 : Point) -> f32 {
    let xd = f32::powf(p1.x - p2.x, 2.0);
    let yd = f32::powf(p1.y - p2.y, 2.0);
    f32::powf(xd + yd, 2.0)
}
