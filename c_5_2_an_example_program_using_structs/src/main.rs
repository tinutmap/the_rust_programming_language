fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle using tuple is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle using struct is {} square pixels.",
        area_struct(&rect2) // NOTE: important to use reference here so it won't be dropped in passing to function.
    );

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("rect3is {:#?}", rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
