use std::mem;

// Эта функция заимствует срез
fn analyze_slice(slice: &[i32]) {
    println!("первый элемент среза: {}", slice[0]);
    println!("второй эемент среза: {}", slice[1]);
    println!("в срезе {} элементов", slice.len());
}

fn main() {
    // Массив фиксированного размера (указывать сигнатуру типа необязательно)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Все элементы могут быть инициализированы одним и тем же значением
    let ys: [i32; 500] = [0; 500];

    // Индекс начинается с 0
    println!("первый элемент массива: {}", xs[0]);
    println!("второй элемент массива: {}", xs[1]);

    // `len` возвращает длину массива
    println!("размер массива: {}", xs.len());

    // Память для массивов выделяется в стеке
    println!("массив занимает {} байт", mem::size_of_val(&xs));

    // Массивы могут быть автоматически заимствованы как срез
    println!("заимствуем весь массив, используя срез");
    analyze_slice(&xs);

    // Срезы могут указывать на часть массива
    // Они имеют форму [starting_index..ending_index]
    // starting_index — это первая позиция в срезе
    // ending_index — на 1 больше, чем последняя позиция в срезе
    println!("заимствуем часть массива как срез");
    analyze_slice(&ys[2 .. 100]);

    // Выход за границу массива заставит компилятор паниковать.
    // Не надо так.
    println!("{}", xs[4]);
    println!("{}", ys.len());

}

