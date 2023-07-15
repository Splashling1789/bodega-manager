mod connection_manager;
mod db_manager;

use clearscreen::clear;
use connection_manager::connection_manager::connect;
use std::io::{BufRead, Read, stdin};
use std::ops::{Add, Deref};
use mysql::{PooledConn};
use mysql::prelude::Queryable;
use db_manager::db_manager::*;

///Título del programa
const TITLE:&str = "LA BODEGA ALBERO";
///Tamaño del título
const MENU_SIZE:u16 = 55;

#[doc= "Imprime el título determinado por la constante TITLE inicial con una anchura determinada por la constante MENU_SIZE"]
macro_rules! print_title {
    () => {for i in 1..MENU_SIZE { print!("-"); }
    print!("\n");
    for i in 1..((MENU_SIZE - TITLE.len() as u16) / 2) { print!(" "); }
    print!("{}", TITLE);
    print!("\n");
    for i in 1..MENU_SIZE { print!("-"); }
    print!("\n");
    };
}

fn menu(connection: &mut PooledConn, option: &mut String) {
    //!Ejecuta el menú de selección de operaciones.
    print_title!();
    println!("1. Consultar existencias");
    println!("2. Añadir o retirar existencias");
    println!("3. Transladar existencias");
    println!("4. Editar categorías");
    println!("5. Editar objetos");
    println!("6. Sobre el programa");
    println!("\n¿Qué deseas hacer?");
    *option = String::from("");
    stdin().read_line( option);
    //TODO: opción 5
    //TODO: opción 3
    //TODO: opción 1
    //TODO: opción 2
    match option.trim() {
        "4" => {
            *option = String::from("");
            clear();
            print_categories(read_categories(connection));
            println!("Qué desea realizar?");
            println!("1. Agregar una categoría");
            println!("2. Eliminar una categoría");
            println!("{:?}", option);
            stdin().read_line(option);
            match option.trim() {
                "1" => {
                    *option = String::from("");
                    let mut nombre = String::new();
                    let mut desc = String::new();
                    println!("Nombre de la nueva categoría: ");
                    stdin().read_line(&mut nombre);
                    println!("\nDescripción de la nueva categoría: ");
                    stdin().read_line(&mut desc);
                    match insert_category(connection, nombre, desc) {
                        Ok(()) => {
                            println!("La categoría se creó satisfactoriamente");
                        }
                        Err(e) => {
                            println!("Ocurrió un error al crear la categoría: {}", e);
                        }
                    }
                }
                "2"  => {
                    *option = String::from("");
                    let mut inp = String::new();
                    println!("Introduce el ID de la categoría a eliminar: ");

                    stdin().read_line(&mut inp);
                    match inp.trim().parse::<i32>() {
                        Ok(id) => {
                            match delete_category(connection, id) {
                                Ok(()) => {
                                    println!("La categoría con id: {}, fue eliminada satisfactoriamente", id);
                                }
                                Err(e) => {
                                    println!("Ocurrió un error al eliminar la categoría: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Ocurrió un error con los datos que ha proporcionado. ¿Ha puesto algo que no sea un número? {}", e);
                        }
                    }
                }
                _ => {
                    println!("No ha seleccionado ninguna opción. Volviendo al menú");
                }
            }
        }
        _ => {
            println!("No ha seleccionado ninguna opción. Volviendo al menú");
        }
    }
}


fn main() {
    //!Se conecta a la base de datos, y ejecuta el menú para comenzar a realizar operaciones sobre ella.
    println!("Conectando a la base de datos...");
    match connect(connection_manager::connection_manager::get_envs()) {
        Ok(c) => {
            clear();
            let mut connection =c;
            let mut option = String::new();
            loop {menu(&mut connection, &mut option)};
        }
        Err(e) => {
            println!("Ocurrió un error al conectarse a la base de datos: {}", e)
        }
    }
}