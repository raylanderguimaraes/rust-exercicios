fn main() {
    let  x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("O valor de x dentro desse escopo é: {x}");
    }
    println!("O valor de x é: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("A quantidade de espaço é: {spaces}");

    // println!("O valor de x é: {x}");
    // x = 6;
    // println!("O valor de x é: {x}");
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("Tres horas em segundos: {THREE_HOURS_IN_SECONDS}")
}
