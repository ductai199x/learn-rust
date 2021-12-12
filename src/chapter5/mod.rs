use std::{fmt, io};

#[derive(Debug)]
struct Point(f32, f32);

#[derive(Debug)]
struct Rect {
    upper_left: Point,
    lower_right: Point,
    width: f32,
    height: f32,
}

trait IntoRect {
    fn into(self) -> Rect;
}

impl IntoRect for (Point, Point) {
    fn into(self) -> Rect {
        let (x1, x2) = self;
        let ul = Point(x1.0.min(x2.0), x1.1.min(x2.1));
        let lr = Point(x1.0.max(x2.0), x1.1.max(x2.1));
        let w = lr.0 - ul.0;
        let h = lr.1 - ul.1;
        Rect {
            upper_left: ul,
            lower_right: lr,
            width: w,
            height: h,
        }
    }
}

impl Rect {
    fn new<A>(args: A) -> Rect
    where
        A: IntoRect,
    {
        args.into()
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn can_hold(&self, r2: &Rect) -> bool {
        self.upper_left.0 <= r2.upper_left.0
            && self.upper_left.1 <= r2.upper_left.1
            && self.lower_right.0 >= r2.lower_right.0
            && self.lower_right.1 >= r2.lower_right.1
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_f!(f, "({self.0} {self.1})")
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_f!(f, "{{ ul: {self.upper_left}, lr: {self.lower_right}, w: {self.width}, h: {self.height} }}")
    }
}

pub fn chapter5() {
    let r1 = Rect::new((Point(0.0, 0.0), Point(10.0, 10.0)));
    let r2 = Rect::new((Point(3.0, 3.0), Point(17.0, 17.0)));

    println_f!("{r1} with area: {area}", area = r1.area());
    println!("r2 in r1? {}", r1.can_hold(&r2));
}
