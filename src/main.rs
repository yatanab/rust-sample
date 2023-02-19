fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    println!("{}" ,sum)
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
