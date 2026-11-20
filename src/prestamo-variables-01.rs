struct Punto{
    x: i32,
    y: i32,
}

fn main() {
    let mut punto = Punto { x: 5, y: 10 };
    moverPunto(&mut punto);
    println!("Punto movido: ({}, {})", punto.x, punto.y);
}

fn moverPunto(p: &mut Punto) {
    p.x+= 10;
    p.y+= 10;
}