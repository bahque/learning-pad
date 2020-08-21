fn main() {
    let w1 = 30;
    let h1 = 50;
    let rect = (w1, h1);

    println!(
        "area is {}",
        area(rect)
    )
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

