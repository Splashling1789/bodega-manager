mod connection_manager;
mod db_manager;

use clearscreen::clear;
use connection_manager::connection_manager::connect;
use db_manager::db_manager::*;
use mysql::prelude::Queryable;
use mysql::PooledConn;
use std::io::{stdin, BufRead, Read};
use std::ops::{Add, Deref};

///Título del programa
const TITLE: &str = "LA BODEGA ALBERO";
///Tamaño del título
const MENU_SIZE: u16 = 55;

#[doc = "Imprime el título determinado por la constante TITLE inicial con una anchura determinada por la constante MENU_SIZE"]
macro_rules! print_title {
    () => {
        for i in 1..MENU_SIZE {
            print!("-");
        }
        print!("\n");
        for i in 1..((MENU_SIZE - TITLE.len() as u16) / 2) {
            print!(" ");
        }
        print!("{}", TITLE);
        print!("\n");
        for i in 1..MENU_SIZE {
            print!("-");
        }
        print!("\n");
    };
}

#[doc = "Imprime un encabezado especificado en $title, con el tamaño especificado por la constante MENU_SIZE"]
macro_rules! print_header {
    ($title: expr) => {
        for i in 1..((MENU_SIZE - stringify!($title).len() as u16) / 2) {
            print!("-");
        }
        print!(" {} ", $title);
        for i in 1..((MENU_SIZE - stringify!($title).len() as u16) / 2) {
            print!("-");
        }
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
    stdin().read_line(option);
    //TODO: opción 3
    //TODO: opción 2
    match option.trim() {
        "1" => {
            *option = String::from("");
            clear();
            let list = read_objects(connection);
            print_header!("EXISTENCIAS");
            print_all_stock(connection, list);
        }
        "4" => {
            *option = String::from("");
            clear();
            print_categories(read_categories(connection));
            println!("Qué desea realizar?");
            println!("1. Agregar una categoría");
            println!("2. Eliminar una categoría");
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
                    match insert_category(
                        connection,
                        String::from(nombre.trim()),
                        String::from(desc.trim()),
                    ) {
                        Ok(()) => {
                            println!("La categoría se creó satisfactoriamente");
                        }
                        Err(e) => {
                            println!("Ocurrió un error al crear la categoría: {}", e);
                        }
                    }
                }
                "2" => {
                    *option = String::from("");
                    let mut inp = String::new();
                    println!("Introduce el ID de la categoría a eliminar: ");

                    stdin().read_line(&mut inp);
                    match inp.trim().parse::<i32>() {
                        Ok(id) => match delete_category(connection, id) {
                            Ok(()) => {
                                println!(
                                    "La categoría con id: {}, fue eliminada satisfactoriamente",
                                    id
                                );
                            }
                            Err(e) => {
                                println!("Ocurrió un error al eliminar la categoría: {}", e);
                            }
                        },
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
        "5" => {
            *option = String::from("");
            clear();
            print_objects(read_objects(connection));
            println!("Qué desea realizar?");
            println!("1. Agregar un objeto");
            println!("2. Eliminar un objeto");
            stdin().read_line(option);
            match option.trim() {
                "1" => {
                    print_categories(read_categories(connection));
                    let mut id_cat = String::new();
                    println!("Inserta el ID de la categoría a la que pertenece: ");
                    stdin().read_line(&mut id_cat);
                    match id_cat.trim().parse::<i32>() {
                        Ok(id) => match get_category_by_id(id, read_categories(connection)) {
                            Some(cat) => {
                                let mut nombre = String::new();
                                println!("Inserta el nombre del objeto: ");
                                stdin().read_line(&mut nombre);
                                let mut medida = String::new();
                                println!("Inserta la unidad de medida del objeto: ");
                                stdin().read_line(&mut medida);
                                nombre = String::from(nombre.trim());
                                medida = String::from(medida.trim());
                                match insert_object(connection, cat, nombre, medida) {
                                    Ok(()) => {
                                        println!("El objeto se creó satisfactoriamente");
                                    }
                                    Err(e) => {
                                        println!("Ocurrió un error al crear el objeto: {}", e);
                                    }
                                }
                            }
                            _ => {
                                println!("Ocurrió un error al buscar la categoría. ¿Es posible que el id dado no corresponda a ninguna categoría?");
                            }
                        },
                        Err(e) => {
                            println!(
                                "Ocurrió un error con el id proporcionado. ¿Ha dado un número? {}",
                                e
                            );
                        }
                    }
                }
                "2" => {
                    *option = String::from("");
                    let mut inp = String::new();
                    println!("Introduce el ID del objeto a eliminar: ");
                    stdin().read_line(&mut inp);
                    match inp.trim().parse::<i32>() {
                        Ok(id) => match delete_object(connection, id) {
                            Ok(()) => {
                                println!("Objeto eliminado satisfactoriamente");
                            }
                            Err(e) => {
                                println!("Ocurrió un error al eliminar el objeto: {}", e);
                            }
                        },
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
    let _ = stdin().lock().lines().next();
}

fn main() {
    //!Se conecta a la base de datos, y ejecuta el menú para comenzar a realizar operaciones sobre ella.
    println!("Conectando a la base de datos...");
    match connect(connection_manager::connection_manager::get_envs()) {
        Ok(c) => {
            let mut connection = c;
            let mut option = String::new();
            loop {
                clear();
                menu(&mut connection, &mut option);
            }
        }
        Err(e) => {
            println!("Ocurrió un error al conectarse a la base de datos: {}", e)
        }
    }
}
