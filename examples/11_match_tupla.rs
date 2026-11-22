fn main() {
    let tuple = (5, 10);
    match tuple {
        (x, y) if x == y => println!("Los valores son iguales: {}", x),
        (x, y) if x > y => println!("El primer valor es mayor: {}", x),
        (x, y) if x < y => println!("El segundo valor es mayor: {}", y),
        _ => println!("No se cumple ninguna condición"),
    }
}