
fn main() {
    let verses = [
        "And a Partridge in a pear tree.",  
        "Two turtle doves",
        "Three French Hens",
        "Four calling birds", 
        "Five golden rings", 
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    
    let days = [
        "primeiro",
        "segundo",
        "terceiro",
        "quarto",
        "quinto",
        "sexto",
        "setimo",
        "oitavo",
        "nono",
        "decimo",
        "decimo_primeiro",
        "decimo_segundo"
    ];

    for i in 0..days.len(){
        println!("\nOn the {} day of Christmas", days[i]);
        println!("My true love sent to me");

        for j in 0..=i {
            println!("{}", verses[j]);
        }
    };
    
}
