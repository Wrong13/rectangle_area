fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
        name : String::from("прямоуго1ник"),
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
        name : String::from("прямо2гольник"),
    };

    if rect.can_hold(&rect2) == true {println!("В {} влезет {}",
        rect.name,rect2.name);}
    else {
        println!("В {} не влезет {}", rect.name,rect2.name);
    }

    println!("{:?}",rect);
    println!("Площадь {}",rect.area());
}

#[derive(Debug)]
struct Rectangle {  // Описание
    width: u32,
    height: u32,
    name : String,
}

impl Rectangle {    // Реализация
    fn area(&self) -> u32 {     // Метод
        self.width * self.height    // self-псевдоним типа для которого реализован блок
    }   // Тобишь сам себя 

    fn can_hold(&self,other: &Rectangle) -> bool {      // Проверка вместится ли иной прямоугольник в текущий
        self.width > other.width && self.height > other.height
    }
}