/*
#![allow(unused)]
fn main() {
// Импортируем (с помощью `use`) модуль `fmt`, чтобы мы могли его использовать.
use std::fmt;

// Определяем структуру, для которой будет реализован `fmt::Display`.
// Это простая кортежная структура c именем `Structure`, которая хранит в себе `i32`.
struct Structure(i32);

// Чтобы была возможность использовать маркер `{}`
// `типаж (trait) fmt::Display` должен быть реализован вручную
// для данного типа.
impl fmt::Display for Structure {
    // Этот типаж требует реализацию метода `fmt` с данной сигнатурой:
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Записываем первый элемент в предоставленный выходной поток: `f`.
        // Возвращаем `fmt::Result`, который показывает выполнилась операция
        // успешно или нет. Обратите внимание на то, что синтаксис `write!`
        // похож на синтаксис `println!`.
        write!(f, "{}", self.0)
    }
}
}
*/


use std::fmt; // Импортируем `fmt`

// Структура, которая хранит в себе два числа.
// Вывод типажа `Debug` добавлен для сравнения с `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Реализуем `Display` для `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Используем `self.номер`, чтобы получить доступ к каждому полю структуры.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Объявим структуру с именованными полями, для сравнения
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// По аналогии, реализуем `Display` для Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Point2D будет по имени
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Структура Complex для задания
#[derive(Debug)]
struct Complex{
    real: f64,
    imag: f64,
}

// По аналогии, реализуем `Display` для Point2D
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Point2D будет по имени
        write!(f, "{} + {}i", self.real, self.imag)
    }
}



fn main() {
    let minmax = MinMax(0, 14);

    println!("Сравниваем структуры:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("Большой диапазон - {big} и маленький диапазон {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Сравниваем точки:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Ошибка. Типажи `Debug` и `Display` были реализованы, но `{:b}`
    // необходима реализация `fmt::Binary`. Следующий код не сработает.
    // println!("Как выглядит Point2D в виде двоичного кода: {:b}?", point);

    let compl = Complex { real: 3.3, imag: 7.2 };
    
    println!("Сравниваем комплексное число (задание):");
    println!("Display: {}", compl);
    println!("Debug: {:?}", compl);

}

