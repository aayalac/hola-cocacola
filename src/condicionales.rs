fn condicional{
    let x: bool = true;
    let y: bool = false;

    let numero: i32 = if y { 
        5 }
         else  if x 
        { 10 
        } else { 
            20 
        };

    println!("El valor de numero es: {}", numero);
    println!("x : {}", x );

    // if y {
    //     println!("y es verdadero");
    // } else {
    //     println!("y es falso");
    // }
}