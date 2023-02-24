fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}",rect);
    println!("Площадь {}",rect.area());
}

#[derive(Debug)]
struct Rectangle {  // Описание
    width: u32,
    height: u32,
}

impl Rectangle {    // Реализация
    fn area(&self) -> u32 {     // Метод
        self.width * self.height    // self-псевдоним типа для которого реализован блок
    }   // Тобишь сам себя 
}