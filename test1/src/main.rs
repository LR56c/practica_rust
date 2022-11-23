use std::collections::HashMap;
use std::io;
use std::io::Stdin;

fn main() {
    let productos = Vec::from(["LecheSoprole", "CervezaAustral", "yogurtSoprole", "mineralVital"]);
    let productos_codigo = Vec::from(["1", "2", "3", "4"]);
    let productos_precio = Vec::from([1500, 2000, 500, 2000]);
    let productos_stock = Vec::from([100, 1000, 500, 20000]);
    let productos_marca = Vec::from(["Soprole", "Austral", "Soprole", "Vital"]);
    let productos_tipo = Vec::from(["Lacteo", "Drinking", "Lacteo", "Drinking"]);
    let ventas = HashMap::from([
        ("1", 3000),
        ("2", 2000),
        ("1", 6000),
        ("3", 1000),
        ("4", 4000),
        ("1", 3000),
        ("2", 4000),
        ("1", 3000)
    ]);


    let mut opcion = 0;

    while opcion != 9 {

        println!("Ingrese opcion:");
        let input_parse = get_int();
        opcion = input_parse;

        let code;

        match opcion {
            1..=5 => {
                println!("Ingrese codigo a buscar:");
                code = get_int();
            },
            _ => {}
        }

        match opcion {
            1 => {
                println!("Nombre");
                println!("Tipo");
            },
            2 => {
                println!("Nombre");
                println!("Marca");
                println!("Stock");
            },
            3 => {
                println!("Nombre");
                println!("Precio");
                println!("Marca");
            },
            4 => {
                println!("Nombre");
                println!("Promedio");
            },
            5 => {
                println!("Nombre");
                println!("Stock");
            },
            6 => {
                println!("productos");
                println!("productos_codigo");
                println!("productos_precio");
                println!("productos_stock");
                println!("productos_marca");
                println!("productos_tipo");
            },
            7 => {
                println!("agregar venta, n° y cantidad");
            },
            8 => {
                println!("eliminar con codigo");
            },
            9 => {
                println!("saliendo del programa");
            },
            _ => {
                println!("opcion no valida, intente denuevo valores entre 1 y 9");
            }
        }
    };
}

fn get_int() -> i32 {
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.read_line(&mut input_line).expect("fail read line");
    let input_parse: i32 = input_line.trim().parse().expect("not int");
    input_parse
}
