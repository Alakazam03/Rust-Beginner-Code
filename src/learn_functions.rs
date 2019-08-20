#![allow(dead_code)]
pub struct Point{
    x : f64,
    y : f64,
}

impl Point{
    pub fn origin() -> Point {
        Point {x : 0.0, y: 0.0}
    }

    pub fn new (x : f64, y : f64) -> Point {
        Point { x : x, y : y}
    }
}

pub fn line() -> f64 {
    let p_start : Point = Point::origin();
    let p_end : Point = Point::new(3.0, 4.0);
    p_start.x - p_end.x + p_start.y - p_end.y
}

pub struct Rectangle{
    pub p1 : Point,
    pub p2 : Point,
}

impl Rectangle{
    fn area(&self) -> f64 {
        let Point {x : x1, y: y1} = self.p1;
        let Point {x : x2, y: y2} = self.p2;

        ((x1-x2) * (y1-y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x : x1, y: y1} = self.p1;
        let Point {x : x2, y: y2} = self.p2;

        2.0 * ((x2-x1).abs() + (y2-y1).abs())
    }

    pub fn translate(&mut self, x : f64, y : f64){
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}










