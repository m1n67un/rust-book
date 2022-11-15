#[derive(Debug)] //: 구조체 위에 어노테이션 추가하게 되면, 디버그 동안 구조체의 모든 값을 볼 수 있다.
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("Rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}