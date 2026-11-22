fn main(){
    printnumber(Some(42));
}

fn printnumber(num: Option<i32>) {
    match num {
        Some(n) => println!("El número es: {}", n),
        None => println!("No se proporcionó ningún número"),
    }
}