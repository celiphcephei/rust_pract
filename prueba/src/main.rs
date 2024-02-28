fn main() {
    println!("Esto es una prueba");
    let mut x: i32 = 4;
    println!("{}", x);
    x = 2;
    println!("{x}");
    let x = 7;
    println!("{x}");

    println!("Resultado de la funcion");
    pepe();

    let variable: i32 = 20;
    assert_eq!(variable, 20);
    println!("quepaso");
}

fn pepe() {
    let x: &str = "uwu";
    println!("{x}");
}
