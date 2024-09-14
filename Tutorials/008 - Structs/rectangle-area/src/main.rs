struct Rectangle {
    width: u32,
    height: u32
}

fn calculate_area(rectangle: Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

fn main() {
    let rect = Rectangle {
        width: 100,
        height: 200
    };

    println!("The rectangle area is {0}. \nThis was made by {1}, and the area is {0}.", calculate_area(rect), "Tulio");
}
