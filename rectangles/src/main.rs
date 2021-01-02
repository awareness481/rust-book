struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // #1 Refactor with Tuples

    let rect1 = (30, 50);

    println!(
        "#1 The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // #2 Refactor with structs

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "#2 The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area2(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area3(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
