#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }
}


fn main() {
    let w1 = 30;
    let h1 = 50;
    let rect = Rectangle {
        w: w1,
        h: h1,
    };

    println!(
        "area of {:?} is {}",
        &rect,
        rect.area()
    )
}


