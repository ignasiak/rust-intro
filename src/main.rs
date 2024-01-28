fn main() {
    println!("Hello, world!");

    // Zmienne
    let x = 10; // deklaracja zmiennej niemutowalnej, typ jest wnioskowany automatycznie, ale może być opcjonalnie zdefiniowany
    println!("The value of x is: {x}");
    // x = 6; błąd kompilacji - zmienna nie może zmienić swojej wartości

    let y; // nie ma konieczności deklaracji z jednoczesną inicjalizacją, ale musi ona nastąpić przed pierwszym użyciem
    y = 10;
    println!("The value of y is: {y}");

    let mut z = 10;
    z = 30;
    println!("The value of z is: {z}");

    let _o = 10; // zmienna nie będzie powodować warning, nawet jeśli nigdy nie będzie użyta

    // Shadowing
    let x = 20;
    {
        let mut x:f32 = x as f32 * 3.0;
        println!("The value of x in the inner scope {x}")
    }
    println!("The value of x in the outer scope {x}");

    // Shadowing przydaje się kiedy chcemy pracować ze zmienną i nie za bardzo przejmować się etapami pośrednimi
    let some_result = 5;
    let mut some_result = add(some_result, 3);
    let some_result = some_result + 5;

    /* Stałe

    - muszą mieć określony typ - nie jest on wnioskowany automatycznie
    - ich wartość musi być znana w czasie kompilacji
    - nie mogą zmieniać swojej wartości (użycie mut nie jest dozwolone)
    - mogą mieć dowolny zasięg, także globalny
    */

    const MONTH_OF_THE_YEAR: i8 = 4;
    const TIMEOUT: i64 = 3600 * 10;

    /* Typy danych

    - muszą być znane/określone w czasie kompilacji - Rust jest statycznie typowany
    - w większości przypadków mogą być wywnioskowane automatycznie przez kompilator
    - typy skalarne/proste - integers, floating-point numbers, booleans and characters
    - typy złożone - tuples, arrays
    */

    /* Integers

     Length	   Signed type    Unsigned type
     8-bit	   i8	          u8
     16-bit	   i16	          u16
     32-bit	   i32 // default u32
     64-bit	   i64	          u64
     128-bit   i128	          u128

     32/64bit  isize	      usize  // używane jako indeksy - nie mogą być ujemne, muszą być pojemne/duże, zależą od arch.

     Number literals	   Example
     Decimal	           98_222_000
     Hex	               0xff
     Octal	               0o77
     Binary	               0b1111_0000
     Byte (u8 only)	       b'A'

     let small_number = 10u8;
     let big_number = 100_000_000_i32;

     - w trybie debug kompilator dodaje weryfikację wystąpienia integer overflow (asercja) i przerywa wykonanie programu
       w przypadku jego wystąpienia
     */

    // let a: u8 = 300; // integer overflow

    /* Floating-point numbers

     - zgodne ze standardem IEEE-754

     Length	   Type
     32-bit	   f32
     64-bit	   f64 // default
     */

    println!("The smallest i8: {} The biggest i8: {}", i8::MIN, i8::MAX);
    println!("The smallest u8: {} The biggest u8: {}", u8::MIN, u8::MAX);
    println!("The smallest i16: {} The biggest i16: {}", i16::MIN, i16::MAX);
    println!("The smallest u16: {} and the biggest u16: {}", u16::MIN, u16::MAX);
    println!("The smallest i32: {} The biggest i32: {}", i32::MIN, i32::MAX);
    println!("The smallest u32: {} The biggest u32: {}", u32::MIN, u32::MAX);
    println!("The smallest i64: {} The biggest i64: {}", i64::MIN, i64::MAX);
    println!("The smallest u64: {} The biggest u64: {}", u64::MIN, u64::MAX);
    println!("The smallest i128: {} The biggest i128: {}", i128::MIN, i128::MAX);
    println!("The smallest u128: {} The biggest u128: {}", u128::MIN, u128::MAX);

    let value = 14.3;
    let other_value = 14.;
    let small_value: f32 = 0.1;

    // Wspierane są standardowe operatory matematyczne https://rust-book.cs.brown.edu/appendix-02-operators.html
    // Uwaga: w przeciwieństwie do innych języków należy zadbać o jawną konwersję typów

    let sum = 5 + 10;
    let difference = 11.5 - 4f32;
    let product = 4.0 * 12.5;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // obcięcie wyniku do 0
    let remainder = 54 % 5;

    let my_float: f32 = 5.0;
    let my_other_float = 8.5; // rust wnioskuje typ jako f32 z kontekstu (operacja poniżej)
    let result = my_float + my_other_float;

    /* Boolean

     - dopuszczalne wartości to true i false
     */

    let positive_result = true;
    let negative_result: bool = false;

    /* char

    - reprezentuje Unicode Scalar Value (może przechowywać złożone znaki)
    - ma rozmiar 4 bajtów
    */

    let c = 'a';
    let z: char = 'ℤ';
    let cat = '😻';

    // W przeciwieństwie do chars ciągi znaków mogą zajmować różną ilość bytów w pamięci (od 1 do 4 bytów optymalizacja)

    let str1 = "Lukasz";
    println!("str1 is {} bytes and {} chars", str1.len(), str1.chars().count());
    println!("str1 {:?} bytes.", "L".as_bytes());
    let str2 = "Łukasz";
    println!("str1 is {} bytes and {} chars", str2.len(), str2.chars().count());
    println!("str2 {:?} bytes.", "Ł".as_bytes());

}

fn add(value:i32, other_value: i32) -> i32 {
    // return value + other_value;
    value + other_value
}