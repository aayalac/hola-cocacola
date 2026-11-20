#[derive(Debug)]

struct Usuario{
    nombre: String,
    edad: u8,
}

fn main() {
    let usuario: Option<Usuario> = Some(Usuario{
        nombre: String::from("Alice"),
        edad: 30,
    });

    match usuario {
        Some(u) => println!("Usuario es : {}, y su edad es : {}", u.nombre, u.edad),
        None => println!("No hay usuario"),
    }
}