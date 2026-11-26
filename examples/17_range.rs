fn main (){
    let numeros = 0..=5;
    let otros_numeros = numeros.clone();
    for num in numeros{
        println!("Consumí el número {}", num);
    }
    for num2 in otros_numeros{
        println!("Consumí el número clonado: {}", num2);
    }
}