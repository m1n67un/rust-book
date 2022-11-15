#[derive(Debug)] //: 구조체 위에 어노테이션 추가하게 되면, 디버그 동안 구조체의 모든 값을 볼 수 있다.
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length = self.width
    }

    fn can_hlod(&self) -> bool{
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle { //: let sq = Rectangle::square(3);
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixles.",
        rect1.area()
    );

    let rect2 = Rectangle { length: 50, width: 30 };
    let rect3 = Rectangle { length: 40, width: 10 };
    let rect4 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect2.can_hold(&rect4));

    let sq = Rectangle::square(1);
}