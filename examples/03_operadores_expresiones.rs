// Tema 03: operadores aritméticos, de comparación y lógicos.

fn main() {
    operaciones();
    comparaciones();
    booleanos();
}

fn operaciones() {
    let suma: i32 = 5 + 3;
    let resta: i32 = 5 - 3;
    let multiplicacion: i32 = 5 * 3;
    let division: i32 = 4 / 3; // división entera: el resultado es 1
    println!("La suma es: {}", suma);
    println!("La resta es: {}", resta);
    println!("La multiplicación es: {}", multiplicacion);
    println!("La división es: {}", division);
}

fn comparaciones() {
    let a: i32 = 5;
    let b: i32 = 10;

    println!("a==b: {}", a == b);
    println!("a!=b: {}", a != b);
    println!("a>b: {}", a > b);
    println!("a<b: {}", a < b);
}

fn booleanos() {
    let x: bool = true;
    let y: bool = false;

    println!("x && y: {}", x && y);
    println!("x || y: {}", x || y);
    println!("!x: {}", !x);
    println!("!y: {}", !y);
}
