#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit-структура
struct Unit;

// Кортежная структура
struct Pair(i32, f32);

// Структура с двумя полями
struct Point {
    x: f32,
    y: f32,
}

// Структуры могут быть использованы в качестве полей другой структуры
#[allow(dead_code)]
struct Rectangle {
    // Прямоугольник может быть определён по расположению в пространстве
    // его верхнего левого и нижнего правого углов
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(points: (f32, f32)) -> f32 {
    let ploshad = points.0 * points.1;
    println!("Площадь этого... равна {:02}", ploshad);
    return ploshad
}

fn square(topleft: Point,  sh_i_v: f32) -> Rectangle {
    let (px, py) = (topleft.x, topleft.y);
    let pointdva = Point { x: px + sh_i_v, y: py + sh_i_v };
    let rec = Rectangle { top_left: topleft, bottom_right: pointdva };
    println!("оно работает)");
    return rec
}

fn main() {
    // Создадим структуру при помощи сокращённой инициализации полей
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Распечатаем отладочную информацию о структуре
    println!("{:?}", peter);


    // Инициализируем `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Получаем доступ к полям структуры
    println!("координаты точки: ({}, {})", point.x, point.y);

    // Создадим новую точку, используя синтаксис обновления структуры и нашу
    // существующую точку
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` будет тем же самым, что и `point.y`, так как мы взяли
    // это поле из `point`
    println!("вторая точка: ({}, {})", bottom_right.x, bottom_right.y);

    // Деструктурируем структуру при помощи `let`
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // создание структуры также является выражением
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Создадим unit-структуру
    let _unit = Unit;

    // Создадим кортежную структуру
    let pair = Pair(1, 0.1);

    // Доступ к полям кортежной структуры
    println!("pair содержит {:?} и {:?}", pair.0, pair.1);

    // Деструктурируем кортежную структуру
    let Pair(integer, decimal) = pair;

    println!("pair содержит {:?} и {:?}", integer, decimal);

    let test_rec = square(point, decimal);
    let p1 = test_rec.top_left;
    let (x1, x2) = (p1.x, p1.y);
    let s = rect_area((x1 , x2));  

}


