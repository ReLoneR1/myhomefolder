// Создаём `enum` для классификации web-событий. Обратите внимание,
// как имена и информация о типе определяют вариант:
// `PageLoad != PageUnload` и `KeyPress(char) != Paste(String)`.
// Все они разные и независимые.
enum WebEvent {
    // `enum` может быть как `unit-подобным`,
    PageLoad,
    PageUnload,
    // так и кортежной структурой,
    KeyPress(char),
    Paste(String),
    // или С-подобной структурой.
    Click { x: i64, y: i64 },
}

// Функция, которая получает на вход `WebEvent` и ничего не возвращает.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("страница загружена"),
        WebEvent::PageUnload => println!("страница выгружена"),
        // Извлечём `c` из `enum`.
        WebEvent::KeyPress(c) => println!("нажата '{}'.", c),
        WebEvent::Paste(s) => println!("нажата \"{}\".", s),
        // Разберём `Click` на `x` и `y`.
        WebEvent::Click { x, y } => {
            println!("кликнуто на x={}, y={}.", x, y);
        },
    }
}



enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Создаётся псевдоним типа

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` создаст `String` из строкового среза.
    let pasted  = WebEvent::Paste("мой текст".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);


     // Мы можем обратиться к каждому варианту перечисления через его
    // псевдоним, а не через его длинное неудобное имя.
    let x = Operations::Add;
    let a = 5;
    let b = 7;
    let c = Operations::Add.run(a,b);
    let v = x.run(a,b);
    println!("c = {}, v = {} (тот же долбаеб)", c, v);
}
