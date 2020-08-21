struct Rectangle {
    w: u32,
    h: u32,
}


fn main() {
    let w1 = 30;
    let h1 = 50;
    let rect = Rectangle {
        w: w1,
        h: h1,
    };

    println!(
        "area is {}",
        area_s(&rect)
    )
}


fn area_s(rect: &Rectangle) -> u32 {
    //rect.w = 100;
    rect.w * rect.h
}

