struct Usuario {
    nombre: String,
    email: String,
    activo: bool,
}

struct Grupo {
    nombre: String,
    usuarios: Vec<Usuario>,
}

fn main() {
    let usuario1 = Usuario {
        nombre: String::from("Alice"),
        email: String::from("alice1@mail.com"),
        activo: true,
    };
    let usuario2 = Usuario {
        nombre: String::from("Bob"),
        email: String::from("bob@mail.com"),
        activo: false,
    };

    let mut grupo = Grupo {
        nombre: String::from("Administradores"),
        usuarios: vec![usuario1, usuario2],
    };

    imprimir_usuarios(&grupo);
    activar_usuario(&mut grupo);
    imprimir_usuarios(&grupo);
}

fn imprimir_usuarios(grupo: &Grupo) {
    println!("Grupo: {}", grupo.nombre);
    for usuario in &grupo.usuarios {
        println!("Usuario: {}, Activo: {}", usuario.nombre, usuario.activo);
    }
}

fn activar_usuario(grupo: &mut Grupo) {
    for usuario in &mut grupo.usuarios {
        usuario.activo = true;
    }
}

