fn main (){
    let numeros = vec![10, 20, 30];
    for num in &numeros{ //usar el puntero &
        println!("El numero es: {}", num);
    }
    println!("Numeros es = {}", numeros[1]);
}