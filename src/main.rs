mod connection_manager;
mod db_manager;

use clearscreen::clear;
use connection_manager::connection_manager::connect;
use db_manager::db_manager::*;
use mysql::PooledConn;
use std::io::{stdin, BufRead};

///Título del programa
const TITLE: &str = "LA BODEGA ALBERO";
///Tamaño del título
const MENU_SIZE: u16 = 55;

#[doc = "Imprime el título determinado por la constante TITLE inicial con una anchura determinada por la constante MENU_SIZE"]
macro_rules! print_title {
    () => {
        for _ in 1..MENU_SIZE {
            print!("-");
        }
        print!("\n");
        for _ in 1..((MENU_SIZE - TITLE.len() as u16) / 2) {
            print!(" ");
        }
        print!("{}", TITLE);
        print!("\n");
        for _ in 1..MENU_SIZE {
            print!("-");
        }
        print!("\n");
    };
}

#[doc = "Imprime un encabezado especificado en $title, con el tamaño especificado por la constante MENU_SIZE"]
macro_rules! print_header {
    ($title: expr) => {
        for _ in 1..((MENU_SIZE - stringify!($title).len() as u16) / 2) {
            print!("-");
        }
        print!(" {} ", $title);
        for _ in 1..((MENU_SIZE - stringify!($title).len() as u16) / 2) {
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
    match option.trim() {
        "1" => {
            *option = String::from("");
            clear();
            let list = read_objects(connection);
            print_header!("EXISTENCIAS");
            print_all_stock(connection, list, false);
        }
        "2" => {
            *option = String::from("");
            clear();
            print_objects(read_objects(connection));
            let mut id = String::from("");
            println!("\nInserta el ID del objeto que desea añadir o retirar:");
            stdin().read_line(&mut id);
            match id.trim().parse::<i32>() {
                Ok(obj_id) => {
                    let id = obj_id;
                    match get_object_by_id(obj_id, read_objects(connection)) {
                        Some(obj) => {
                            clear();
                            let mut mode = String::from("");
                            print_all_stock(connection, vec![obj.clone()], false);
                            println!(
                                "\n1. SET: El número que introduzcas sobrescribirá la cantidad"
                            );
                            println!("2. ADD: El número que introduzcas se sumará, o se restará si es negativo");
                            println!("Especifica el modo de inserción:");
                            let mut set_mode: bool = false;
                            stdin().read_line(&mut mode);
                            match mode.trim() {
                                "1" => {
                                    set_mode = true;
                                }
                                "2" => {
                                    set_mode = false;
                                }
                                _ => {
                                    println!(
                                        "Ningún modo fue seleccionado. Se asignará el modo ADD"
                                    );
                                }
                            }
                            let mut cantidad = String::from("");
                            println!("Ingresa la cantidad a realizar la operación");
                            stdin().read_line(&mut cantidad);
                            match cantidad.trim().parse::<f32>() {
                                Ok(cantidad) => {
                                    let mut loc = String::from("");
                                    println!("1. Aplicar cambios en CASA");
                                    println!("2. Aplicar cambios en TARA");
                                    println!("\nSelecciona el lugar en el que hacer la operación");
                                    stdin().read_line(&mut loc);
                                    let mut location = Procedencia::Casa;
                                    match loc.trim() {
                                        "1" => {
                                            location = Procedencia::Casa;
                                        }
                                        "2" => {
                                            location = Procedencia::Tara;
                                        }
                                        _ => {
                                            println!(
                                                "Ningún lugar fue seleccionado. Se asignará CASA"
                                            );
                                        }
                                    }
                                    println!("Se realizará una operación con la siguiente configuración:");
                                    println!(
                                        "OBJ:{}\nSET: {}\nCAN:{}\nLOC:{:?}",
                                        obj.nombre, set_mode, cantidad, location
                                    );
                                    println!("\nContinuar? (Pon S para aceptar, y cualquier cosa para cancelar)");
                                    stdin().read_line(option);
                                    match option.trim() {
                                        "S" | "s" => {
                                            match update_stock(
                                                connection, id, set_mode, cantidad, &location,
                                            ) {
                                                Ok(()) => {
                                                    println!("La base de datos se actualizó satisfactoriamente");
                                                }
                                                Err(e) => {
                                                    println!("Ocurrió un error al actualizar la base de datos: {}", e);
                                                }
                                            }
                                        }
                                        _ => {
                                            println!("Operación cancelada.");
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Hubo un error con la cantidad ingresada. ¿Ha dado un número? {}", e);
                                }
                            }
                        }
                        None => {
                            println!(
                                "No se encontró el id en la base de datos. Vuelve a intentarlo"
                            );
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "Ocurrió un error con el id proporcionado. ¿Ha dado un número? {}",
                        e
                    );
                }
            }
        }
        "3" => {
            *option = String::from("");
            let objs = read_objects(connection);
            print_all_stock(connection, objs.clone(), true);
            let mut id = String::new();
            println!("Introduce el ID del objeto a transladar");
            stdin().read_line(&mut id);
            match id.trim().parse::<i32>() {
                Ok(id) => match get_object_by_id(id, objs) {
                    Some(obj) => {
                        println!("1. Transferir de Casa a Tara\n2. Transferir de Tara a Casa");
                        let mut proc = String::new();
                        stdin().read_line(&mut proc);
                        let mut procedence = Procedencia::Casa;
                        match proc.trim() {
                            "1" => {
                                procedence = Procedencia::Casa;
                            }
                            "2" => {
                                procedence = Procedencia::Tara;
                            }
                            _ => {
                                println!("Opción inválida, usando por defecto: de Casa a Tara");
                            }
                        }
                        println!("Ingresa la cantidad a transladar: ");
                        let mut quant = String::new();
                        stdin().read_line(&mut quant);
                        match quant.trim().parse::<f32>() {
                            Ok(quant) => {
                                println!("¿Transladar {} {} de {}, {} -> {}? (Pon S para aceptar, cualquier otra cosa para cancelar)", quant, obj.medida, obj.nombre, get_string_name(&procedence), get_string_name(&contrary(&procedence)));
                                stdin().read_line(option);
                                match option.trim() {
                                    "S" | "s" => {
                                        match update_stock(
                                            connection,
                                            obj.id,
                                            false,
                                            -quant,
                                            &procedence,
                                        ) {
                                            Ok(()) => {
                                                match update_stock(
                                                    connection,
                                                    obj.id,
                                                    false,
                                                    quant,
                                                    &contrary(&procedence),
                                                ) {
                                                    Ok(()) => {
                                                        println!("Operación realizada satisfactoriamente.");
                                                    }
                                                    Err(e) => {
                                                        println!("Error al hacer la solicitud de añadir en la base de datos: {}", e);
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                println!("Error al hacer la solicitud de retirada en la base de datos: {}", e);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {
                                println!("Hubo un error con la cantidad proporcionada. ¿Has puesto un número? {}", e);
                            }
                        }
                    }
                    None => {
                        println!("La id proporcionada no corresponde a ningún objeto existente");
                    }
                },
                Err(e) => {
                    println!("Error con el id proporcionado. ¿Ha puesto un número? {}", e);
                }
            }
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
        "6" => {
            *option = String::from("");
            clear();
            print_title!();
            println!("Creada por Javier Albero para una necesidad personal y para aprender Rust y SQL.\nVer. alpha 1.0");
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
