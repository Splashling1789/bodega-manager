mod connection_manager;

use clearscreen::clear;
use connection_manager::connection_manager::connect;
use std::io::{BufRead, Read, stdin};
use std::ops::{Add, Deref};
use mysql::{params, PooledConn};
use mysql::prelude::Queryable;


///Título del programa
const TITLE:&str = "LA BODEGA ALBERO";
///Tamaño del título
const MENU_SIZE:u16 = 55;

#[doc= "Imprime el título determinado por la constante TITLE inicial con una anchura determinada por la constante MENU_SIZE"]
macro_rules! print_title {
    () => {    for i in 1..MENU_SIZE { print!("-"); }
    print!("\n");
    for i in 1..((MENU_SIZE - TITLE.len() as u16) / 2) { print!(" "); }
    print!("{}", TITLE);
    print!("\n");
    for i in 1..MENU_SIZE { print!("-"); }
    print!("\n");};
}

#[derive(Clone)]
///Estructura basada en la tabla categorias de la base de datos bodega-db
struct Categoria {
    id:i32,
    nombre:String,
    descripcion:String
}

///Estructura basada en la tabla objetos de la base de datos bodega-db
struct Objeto {
    id:i32,
    categoria: Categoria,
    nombre:String,
    medida:String
}

///Estructura basada en la tabla existencias-home o existencias-tara de la base de datos bodega-db
struct Existencia {
    objeto: Objeto,
    cantidad:i32
}


//TODO: reorganizar funciones read, insert, get y print en un módulo aparte
fn read_categories(connection: &mut PooledConn) -> Vec<Categoria> {
    //!Lee la tabla categorias y la devuelve como un vector de estructuras "Categoria"
    return connection.query_map("SELECT id, nombre, descripcion FROM categorias;", |(id, nombre, descripcion)| {
        Categoria {
            id, nombre, descripcion
        }
    }).unwrap();
}

fn get_category_by_id(id:i32, categories: Vec<Categoria>) -> Option<Categoria>{
    //!Busca la categoría dentro de un vector de "Categoria" que corresponde a un id dado.
    for c in categories {
        if c.id == id {
            return Some(c);
        }
    }
    return None;
}

fn read_objects(connection: &mut PooledConn, categories: Vec<Categoria>) -> Vec<Objeto> {
    //!Lee la tabla objetos, y la devuelve como un vector de estructuras de "Categoría". En lugar de guardar la categoría como un id, obtiene la categoría que corresponde a ese id, y la guarda dentro de la estructura "Objeto".
    let mut result: Vec<Objeto> = Vec::new();
    let list:Vec<(i32, i32, String, String)> = connection.query_map("SELECT id, categoria, nombre, medida FROM objetos;", |(id, id_cat, nombre, medida)| (id, id_cat, nombre, medida)).unwrap();
    for o in list {
        match get_category_by_id(o.1, categories.clone()) {
            Some(cat) => {
                result.push(Objeto {
                    categoria: cat,
                    id: o.1,
                    nombre: o.2,
                    medida: o.3
                })
            }
            None => {
                result.push( Objeto{
                categoria: Categoria {
                    id:0,
                    descripcion: String::from("Error al obtener la categoría. "),
                    nombre:String::from("ERROR"),
                },
                id: o.1,
                nombre: o.2,
                medida: o.3});
            }
        }
    }
    return result;
}

fn print_categories(categories:Vec<Categoria>) {
    //!Imprime las categorías de la tabla categorias de la base de datos.
    for c in categories {
        println!("{}. {}: {}", c.id, c.nombre, c.descripcion);
    }
}

fn print_objects(objects:Vec<Objeto>) {
    //!Imprime los objetos de la tabla objetos de la base de datos.
    for o in objects {
        println!("De {}, {}. Se mide en {} y su id interna es {}", o.categoria.nombre, o.nombre, o.medida, o.id);
    }
}

fn insert_category(conn: &mut PooledConn, name: String, desc: String) -> Result<(), mysql::Error>{
    //!Inserta un registro de la tabla categorías dado nombre y descripción.
    return conn.exec_drop("INSERT INTO categorias (nombre, descripcion), VALUES (:nombre, :descripcion)", params! {
        "name" => name,
        "descripcion" => desc,
    });
}

fn delete_category(conn: &mut PooledConn, id: i32) -> Result<(), mysql::Error> {
    //!Elimina un registro de la tabla categorías dado su id.
    return conn.exec_drop("DELETE FROM categorias WHERE id=:id;", params!("id" => id));
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