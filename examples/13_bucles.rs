fn main(){
    let mut counter  = 0;

    let resultado = loop {
        counter += 1;
        println!("Contando: {}", counter);

        if counter == 5 {
            break "alcancé el 5";
        }  

    };

    println!("El loop regresó: {}", resultado);
}