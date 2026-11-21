struct Punto{
    x: i32,
    y: i32,
}

fn main() {
    let mut punto = Punto { x: 5, y: 10 };
    mover_punto(&mut punto);
    println!("Punto movido: ({}, {})", punto.x, punto.y);
}

fn mover_punto(p: &mut Punto) {
    p.x+= 10;
    p.y+= 10;
}