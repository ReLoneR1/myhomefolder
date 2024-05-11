fn main() {

    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Копируем `an_integer` в `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // Компилятор предупреждает о неиспользуемых переменных; эти предупреждения
    // можно скрыть, поставив знак подчёркивания в начало имени переменной
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Поставьте знак подчёркивания в начало имени, чтобы убрать предупреждение
    // Обратите внимание, что предупреждения могут не отображаться в браузере
    

    // Izmenyaemost //////////////////////////////////////////////////////////////
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Перед изменением: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("После изменения: {}", mutable_binding);

    // Ошибка!
    // _immutable_binding += 1;
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку
    
    
    // Oblast vidimosti ///////////////////////////////////////////////////////////////
    // Эта переменная живёт в функции main
    let long_lived_binding = 1;

    // Это блок, он имеет меньшую область видимости, чем функция main
    {
        // Эта переменная существует только в этом блоке
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // Конец блока

    // Ошибка! `short_lived_binding` нет в этой области видимости
    // println!("outer short: {}", short_lived_binding);
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    println!("outer long: {}", long_lived_binding);


    // Zatenenie //////////////////////////////////////////////////////////////////////
    let shadowed_binding = 1;

    {
        println!("До затенения: {}", shadowed_binding);

        // Эта переменная *затеняет* внешнюю
        let shadowed_binding = "abc";

        println!("затенённая во внутреннем блоке: {}", shadowed_binding);
    }
    println!("во внешнем блоке: {}", shadowed_binding);

    // Эта привязка *затеняет* предыдущую
    let shadowed_binding = 2;
    println!("затенённая во внешнем блоке: {}", shadowed_binding);

    // Objavlenie ////////////////////////////////////////////////////////////////////////////
    // Объявляем связь с переменной
    let a_binding;

    {
        let x = 2;

        // Инициализируем связь
        a_binding = x * x;
    }

    println!("связь а: {}", a_binding);

    let another_binding;

    // Ошибка! Использование неинициализированной связи с переменной
    // println!("другая связь: {}", another_binding);
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    another_binding = 1;

    println!("другая связь: {}", another_binding);


    // Zamorozka ///////////////////////////////////////////////////////////////////////////
    let mut _mutable_integer = 7i32;

    {
        // Неизменяемое затенение `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Ошибка! `_mutable_integer` заморожена в этой области
        // _mutable_integer = 50;
        // ИСПРАВЬТЕ ^ Закомментируйте эту строку

        // `_mutable_integer` выходит из области видимости
    }

    // Ok! `_mutable_integer` не заморожена в этой области
    _mutable_integer = 3;
}
