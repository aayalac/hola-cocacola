// enum Option<T>{
//     None,
//     Some(T),
// }

fn main() {
    let valor: Option<i32> = Some(42);
    match valor{
        Some(numero) => println!("El numero es: {},", numero),
        None => println!("No hay numero"),
    }
}