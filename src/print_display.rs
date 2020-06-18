use std::fmt;

#[derive(Debug)]
struct MinMax(i64,i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Nameable values
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);

    println!("{big} {small}",
             small = small_range,
             big = big_range);

    let p = Point2D{x:3.3, y: 2.2};

    println!("Compare structures:");
    println!("Display: {}", p);
    println!("Debug: {:?}", p);

    let c = Complex{real: 3.3, imag:2.2};
    println!("Compare structures:");
    println!("Display: {}", c);
    println!("Debug: {:?}", c);


}
