fn main() {
    let w1 = 30;
    let h1 = 50;

    println!(
        "area is {}",
        area(w1, h1)
    )
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

