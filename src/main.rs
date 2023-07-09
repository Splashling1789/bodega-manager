mod connection_manager;

use connection_manager::connection_manager::connect;
use std::env;
use std::env::VarError;
use std::io::{BufRead, stdin};
use std::ops::{Add, Deref};
use dotenv::{Error, from_path};
use mysql::{Pool, PooledConn};
use mysql::prelude::Queryable;

macro_rules! print_title {
    () => {    for i in 1..MENU_SIZE { print!("-"); }
    print!("\n");
    for i in 1..((MENU_SIZE - TITULO.len() as u16) / 2) { print!(" "); }
    print!("LA BODEGA ALBERO");
    //for i in 1..((MENU_SIZE - TITULO.len() as u16) / 2) { print!(" "); }
    print!("\n");
    for i in 1..MENU_SIZE { print!("-"); }
    print!("\n");};
}

#[derive(Clone)]
struct Categoria {
    id:i32,
    nombre:String,
    descripcion:String
}

struct Objeto {
    id:i32,
    categoria: Categoria,
    nombre:String,
    medida:String
}

struct Existencia {
    objeto: Objeto,
    cantidad:i32
}
const DEBUG:bool = true;
const TITULO:&str = "LA BODEGA ALBERO";
const MENU_SIZE:u16 = 55;

/*fn read_categories(connection: &mut PooledConn) -> Vec<(i32, String, String)> {
    return connection.query_map("SELECT id, nombre, descripcion FROM categorias;", |(id, nombre, descripcion)| (id, nombre, descripcion),).unwrap();
}*/

fn read_categories(connection: &mut PooledConn) -> Vec<Categoria> {
    return connection.query_map("SELECT id, nombre, descripcion FROM categorias;", |(id, nombre, descripcion)| {
        Categoria {
            id, nombre, descripcion
        }
    }).unwrap();
}

fn get_category_by_id(id:i32, categories: Vec<Categoria>) -> Option<Categoria>{
    for c in categories {
        if c.id == id {
            return Some(c);
        }
    }
    return None;
}

fn read_objects(connection: &mut PooledConn, categories: Vec<Categoria>) -> Vec<Objeto> {
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
    /*print!("| id | Categoría |");
    for i in 1..=MENU_SIZE/5 as u16 {
        print!(" ");
    }
    print!("Descripción");
    for i in 1..=MENU_SIZE/5 as u16 {
        print!(" ");
    }
    println!("|");
    for c in categories {
        //println!("{} con id {} tiene de descripción: {}", c.nombre, c.id, c.descripcion);
        let mut s = format!("|{}|{}|", c.id, c.nombre);
        let size = MENU_SIZE*2/5 + 11;
        if c.descripcion.len() > size {
            s.add(c.descripcion.)
        }
    }*/
    for c in categories {
        println!("{}. {}: {}", c.id, c.nombre, c.descripcion);
    }
}

fn print_objects(objects:Vec<Objeto>) {
    for o in objects {
        println!("De {}, {}. Se mide en {} y su id interna es {}", o.categoria.nombre, o.nombre, o.medida, o.id);
    }
}

fn menu() {
    println!("1. Consultar existencias");
    println!("2. Añadir o retirar existencias");
    println!("3. Transladar existencias");
    println!("4. Editar categorías");
    println!("5. Editar objetos");
    println!("6. Sobre el programa");
    println!("\n¿Qué deseas hacer?");
    let mut option = String::new();
    stdin().read_line(&mut option);

}


fn main() {
    match connect(connection_manager::connection_manager::get_envs()) {
        Ok(c) => {
            let mut connection =c;
            print_title!();
            print_categories(read_categories(&mut connection));
            menu();
        }
        Err(e) => {
            println!("Ocurrió un error al conectarse a la base de datos: {}", e)
        }
    }
}
