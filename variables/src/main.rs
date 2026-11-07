fn main() {
    // let  x = 5;
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("O valor de x dentro desse escopo é: {x}");
    // }
    // println!("O valor de x é: {x}");

    // let spaces = "    ";
    // let spaces = spaces.len();

    // println!("A quantidade de espaço é: {spaces}");

    // println!("O valor de x é: {x}");
    // x = 6;
    // println!("O valor de x é: {x}");
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("Tres horas em segundos: {THREE_HOURS_IN_SECONDS}")

    // let num_1 = 2.0; //f64
    // let num_2: f32 = 3.0; //f32

    // let sum = num_1 + num_2;

    // println!("O resultado da soma é: {sum}");

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let quinhentos = tup.0;

    // let (x, y, z) = tup;


    // println!("Elemento acessado diretamente com . {quinhentos}");
    // println!("Elemento acessado por desestruturação: {x}");

    let x = five();
    println!("O valor de x é: {x}");
    let x = plus_one(x);
    println!("Novo valor de x e: {x}");

}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}