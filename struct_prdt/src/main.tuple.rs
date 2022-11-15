fn main() {
    let rec1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(demesions: (u32, u32)) -> u32 {
    demesions.0 * demesions.1
}
