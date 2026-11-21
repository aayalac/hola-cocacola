fn main() {
    let nombre: &str = "pablo";
    let puntero: &str = &nombre;
    let nombre: &str = "carlos";
    println!("Hola, {}!", nombre);
    println!("El puntero apunta a: {}", puntero);
}
