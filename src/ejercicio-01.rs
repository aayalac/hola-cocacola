fn main() {
    let nombre: &str = "Miguel";
    let puntero: &str = &nombre;
    {
        let nombre: &str = "Maria";
        println!("El puntero apunta a: {}", nombre);
    }
    let animal: &str = "gato";
    println!("Hola, mi nombre es {}, Yo tengo una mascota y es un {}", nombre, animal);    
}
