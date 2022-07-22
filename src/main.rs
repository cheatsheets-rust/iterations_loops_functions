fn main() {

    // CALCUL
    println!("----------- CALCUL -----------");
    // addition entier
    let mut mon_entier_addition : i32 = 42;
    mon_entier_addition = mon_entier_addition + 2;
    dbg!(mon_entier_addition);

    // soustraction entier
    let mut mon_entier_soustraction : i32 = 42;
    mon_entier_soustraction = mon_entier_soustraction - 2;
    dbg!(mon_entier_soustraction);

    // division euclidienne entier
    let mut mon_entier_division : i32 = 42;
    mon_entier_division = mon_entier_division / 10;
    dbg!(mon_entier_division);

    // modulo entier
    let mut mon_entier_modulo : i32 = 42;
    mon_entier_modulo = mon_entier_modulo % 10;
    dbg!(mon_entier_modulo);

    // addition decimal
    let mut decimal_addition : f64 = 42.0;
    decimal_addition = decimal_addition + 2.0;
    dbg!(decimal_addition);

    // substraction decimal
    let mut decimal_substraction : f64 = 42.0;
    decimal_substraction = decimal_substraction - 2.0;
    dbg!(decimal_substraction);

    // division decimal
    let mut decimal_division : f64 = 42.0;
    decimal_division = decimal_division - 2.0;
    dbg!(decimal_division);

    // boolean
    let mut boolean : bool = true;
    boolean = !boolean;
    dbg!(boolean);

    // true/false conditional boolean
    let boolean : bool = true;
    if  boolean {
        println!("mon boolean est 'true'")
    } else {
        println!("mon boolean est 'false'")
    }

    // OR conditional boolean
    let or : bool = true || false;
    dbg!(or);

    // AND contional boolean
    let and : bool = true && false;
    dbg!(and);

    // entier conditionnal
    let entier_condition : i8 = 43;
    if entier_condition > 42 {
        println!("Mon entier est supérieur à 42");
    }

    // ternaire entier conditionnal
    let premier_entier_condition : i8 = 43;
    let mut second_entier_condition : i8 = 0;
    if entier_condition < 42 {
        println!("Mon entier est supérieur à 42");
    } else {
        second_entier_condition = premier_entier_condition + 1;
    }
    dbg!(second_entier_condition);

    // LOOP
    println!("----------- LOOP -----------");
    //for
    for i_for in 0..=5 {
        dbg!(i_for);
    }
    //while
    let mut i_while : i32 = 0;
    while i_while <= 5{
        dbg!(i_while);
        i_while += 1
    }
    //loop (especialy in rust)
    let mut i_loop : i32 = 1;
    loop{
        dbg!(i_loop);
        i_loop += 1;
        if i_loop == 10 {
            break;
        }
    }

    // FUNCTIONS
    println!("----------- FUNCTIONS -----------");
    //basic
    fn ma_function() {
        println!("je suis dans ma fonction")
    }
    ma_function();

    // with one parameter
    fn ma_function_parameters(nombre : i8) {
        for i_function_paramaters in 0..=nombre {
            dbg!(i_function_paramaters);
        }
    }
    ma_function_parameters(10);

    // with two parameters
    fn ma_function_two_parameters(message: &str, nombre : i8) {
        println!("{} {}", message, nombre)
    }
    ma_function_two_parameters("j'ai compté jusqu'à", 10);

    // REFERENCES
    println!("----------- REFERENCES -----------");
    let mon_entier_references : u8 = 42;
    println!("mon entier:");
    println!("valeur: {}", mon_entier_references);
    // representation hexadecimal, base16, d'un emplacement dans la mémoire
    println!("addresse mémoire: {:p}", &mon_entier_references);
    // reférence
    println!("-----------");
    let ma_reference : &u8 = &mon_entier_references;
    println!("ma reference:");
    println!("valeur: {}", ma_reference);
    // representation hexadecimal, base16, d'un emplacement dans la mémoire
    println!("addresse mémoire: {:p}", &ma_reference);
    println!("addresse cible: {:p}", ma_reference);

    // passage de paramètre par valeur
    let coucou : String = "Coucou".to_string();
    fn julien (phrase : &String) {
        println!("{} julien!", phrase)
    }
    julien(&coucou);
    julien(&coucou);

}
