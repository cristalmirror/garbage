use std::io;
use rand::Rng;

fn random_number(a: i64){

    println!("RN: {a}");
}


fn main() {

    println!("hola mundo");

    println!("input data insert: ");

    //crea una variable de tipo string
    let mut dato_inpt = String::new();


    let num_alia = rand::thread_rng().gen_range(1..=100);

    random_number(num_alia);
    
    //intrada standarde del sistema que lanza mensaje de error por defecto
    io::stdin().read_line(&mut dato_inpt).expect("Error in execution");

    println!("el valor  insertado: {dato_inpt}");
        
    
    
}
