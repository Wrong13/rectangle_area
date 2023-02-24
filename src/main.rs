fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}",rect);
    println!("Площадь {}",area(&rect));
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}