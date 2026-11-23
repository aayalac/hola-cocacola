fn main(){
    let mut numeros = vec![1,2,3];
    while let Some(valor) = numeros.pop(){
        println!("Obtuve: {}", valor);
    }
}