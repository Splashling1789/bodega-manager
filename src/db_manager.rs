///Módulo que gestiona la base de datos a través de una conexión
pub mod db_manager {
    use mysql::{params, PooledConn};
    use mysql::prelude::Queryable;

    ///Enum para indicar la base de datos de la que procede la existencia.
    #[derive(PartialEq, Clone)]
    pub enum Procedencia {
        Casa,
        Tara,
    }

    fn contrary(loc: &Procedencia) -> Procedencia {
        if *loc == Procedencia::Casa {
            return Procedencia::Tara;
        } else if *loc == Procedencia::Tara {
            return Procedencia::Casa;
        } else { return Procedencia::Casa; }
    }

    fn get_string_name(loc: &Procedencia) -> String {
        if *loc == Procedencia::Casa {
            return String::from("Casa");
        } else if *loc == Procedencia::Tara {
            return String::from("Tara");
        } else { return String::from("???"); }
    }

    #[derive(Clone)]
    ///Estructura basada en la tabla categorias de la base de datos bodega-db
    pub struct Categoria {
        id: i32,
        nombre: String,
        descripcion: String,
    }

    #[derive(Clone)]
    ///Estructura basada en la tabla objetos de la base de datos bodega-db
    pub struct Objeto {
        id: i32,
        categoria: Categoria,
        nombre: String,
        medida: String,
    }


    ///Estructura basada en la tabla existencias-home o existencias-tara de la base de datos bodega-db
    #[derive(Clone)]
    pub struct Existencia {
        objeto: Objeto,
        cantidad: f64,
        procedencia: Procedencia,
    }

    pub fn read_objects(connection: &mut PooledConn) -> Vec<Objeto> {
        //!Lee la tabla objetos, y la devuelve como un vector de estructuras de "Categoría". En lugar de guardar la categoría como un id, obtiene la categoría que corresponde a ese id, y la guarda dentro de la estructura "Objeto".
        let categories = read_categories(connection);
        let mut result: Vec<Objeto> = Vec::new();
        let list: Vec<(i32, i32, String, String)> = connection.query_map("SELECT id, categoria, nombre, medida FROM objetos;", |(id, id_cat, nombre, medida)| (id, id_cat, nombre, medida)).unwrap();
        for o in list {
            match get_category_by_id(o.1, categories.clone()) {
                Some(cat) => {
                    result.push(Objeto {
                        categoria: cat,
                        id: o.0,
                        nombre: o.2,
                        medida: o.3,
                    })
                }
                None => {
                    result.push(Objeto {
                        categoria: Categoria {
                            id: 0,
                            descripcion: String::from("Error al obtener la categoría. "),
                            nombre: String::from("ERROR"),
                        },
                        id: o.0,
                        nombre: o.2,
                        medida: o.3,
                    });
                }
            }
        }
        return result;
    }

    pub fn print_objects(objects: Vec<Objeto>) {
        //!Imprime los objetos de la tabla objetos de la base de datos.
        for o in objects {
            println!("{} (ID:{}, CT:{}, MD:{})", o.nombre, o.id, o.categoria.nombre, o.medida);
        }
    }

    pub fn insert_object(conn: &mut PooledConn, cat: Categoria, name: String, measure: String) -> Result<(), mysql::Error> {
        //!Inserta un registro en la tabla objeto dada su categoría, nombre y unidad de medida
        return conn.exec_drop("INSERT INTO objetos (categoria, nombre, medida) VALUES (:cat, :name, :measure)", params! {
            "cat" => cat.id,
            "name" => name,
            "measure" => measure,
        }, );
    }

    pub fn delete_object(conn: &mut PooledConn, id: i32) -> Result<(), mysql::Error> {
        //!Borra un registro de la tabla objetos dado su id
        return conn.exec_drop("DELETE FROM objetos WHERE id=:id;", params! {
            "id" => id,
        }, );
    }

    pub fn get_object_by_id(id: i32, objects: Vec<Objeto>) -> Option<Objeto> {
        //!Busca el objeto dentro de un vector de objetos que corresponde a un id dado.
        for o in objects {
            if o.id == id {
                return Some(o);
            }
        }
        return None;
    }

    pub fn print_categories(categories: Vec<Categoria>) {
        //!Imprime las categorías de la tabla categorias de la base de datos.
        for c in categories {
            println!("({}) {}: {}", c.id, c.nombre, c.descripcion);
        }
    }

    pub fn insert_category(conn: &mut PooledConn, name: String, desc: String) -> Result<(), mysql::Error> {
        //!Inserta un registro de la tabla categorías dado nombre y descripción.
        return conn.exec_drop("INSERT INTO categorias (nombre, descripcion) VALUES (:nombre, :descripcion);", params! {
        "nombre" => name,
        "descripcion" => desc,
    }, );
    }

    pub fn delete_category(conn: &mut PooledConn, id: i32) -> Result<(), mysql::Error> {
        //!Elimina un registro de la tabla categorías dado su id.
        return conn.exec_drop("DELETE FROM categorias WHERE id=:id;", params!("id" => id));
    }

    pub fn read_categories(connection: &mut PooledConn) -> Vec<Categoria> {
        //!Lee la tabla categorias y la devuelve como un vector de estructuras "Categoria"
        return connection.query_map("SELECT id, nombre, descripcion FROM categorias;", |(id, nombre, descripcion)| {
            Categoria {
                id,
                nombre,
                descripcion,
            }
        }).unwrap();
    }

    pub fn get_category_by_id(id: i32, categories: Vec<Categoria>) -> Option<Categoria> {
        //!Busca la categoría dentro de un vector de "Categoria" que corresponde a un id dado.
        for c in categories {
            if c.id == id {
                return Some(c);
            }
        }
        return None;
    }

    pub fn read_stock(conn: &mut PooledConn, location: Option<Procedencia>) -> Vec<Existencia> {
        //!Lee las existencias en la tabla indicada en location. Si se indica "None", se devolverán ambas tablas: existencias_home y existencias_tara
        let mut stock: Vec<Existencia> = vec![];
        let objects = read_objects(conn);
        match location {
            Some(pr) => {
                match pr {
                    Procedencia::Casa => {
                        let list: Vec<(i32, f64)> = conn.query_map("SELECT id_objeto, cantidad FROM existencias_home", |(obj, quantity)| (obj, quantity)).unwrap();
                        for o in list {
                            stock.push(Existencia {
                                objeto: get_object_by_id(o.0, objects.clone()).unwrap(),
                                cantidad: o.1,
                                procedencia: Procedencia::Casa,
                            })
                        }
                    }
                    Procedencia::Tara => {
                        let list: Vec<(i32, f64)> = conn.query_map("SELECT id_objeto, cantidad FROM existencias_tara", |(obj, quantity)| (obj, quantity)).unwrap();
                        for o in list {
                            stock.push(Existencia {
                                objeto: get_object_by_id(o.0, objects.clone()).unwrap(),
                                cantidad: o.1,
                                procedencia: Procedencia::Tara,
                            })
                        }
                    }
                }
            }
            None => {
                let list_home: Vec<(i32, f64)> = conn.query_map("SELECT id_objeto, cantidad FROM existencias_home", |(obj, quantity)| (obj, quantity)).unwrap();
                for o in list_home {
                    stock.push(Existencia {
                        objeto: get_object_by_id(o.0, objects.clone()).unwrap(),
                        cantidad: o.1,
                        procedencia: Procedencia::Casa,
                    })
                }
                let list_tara: Vec<(i32, f64)> = conn.query_map("SELECT id_objeto, cantidad FROM existencias_home", |(obj, quantity)| (obj, quantity)).unwrap();
                for o in list_tara {
                    stock.push(Existencia {
                        objeto: get_object_by_id(o.0, objects.clone()).unwrap(),
                        cantidad: o.1,
                        procedencia: Procedencia::Tara,
                    })
                }
            }
        }
        return stock;
    }

    pub fn get_item_index(list: Vec<Existencia>, id: i32, location: Procedencia) -> Option<usize> {
        //!Obtiene la cantidad de un objeto y su posición en la lista, dado su id, de una de las tablas indicadas en location
        for i in 0..list.len() {
            if list.get(i).unwrap().objeto.id == id && list.get(i).unwrap().procedencia == location {
                return Some(i);
            }
        }
        return None;
    }

    pub fn get_stock_by_id(conn: &mut PooledConn, obj_id: i32) -> (Option<Existencia>, Option<Existencia>) {
        let objs = read_objects(conn);
        let vec_home: Vec<Existencia> = conn.exec_map("SELECT * FROM existencias_home WHERE id_objeto=:id",params! {"id"=>obj_id}, |(id_objeto, cantidad)| {
            Existencia {
                objeto: get_object_by_id(id_objeto, objs.clone()).unwrap(),
                cantidad: cantidad,
                procedencia: Procedencia::Casa,
            }
        }).unwrap();
        let vec_tara: Vec<Existencia> = conn.exec_map("SELECT * FROM existencias_tara WHERE id_objeto=:id", params! {"id"=>obj_id},|(id_objeto, cantidad)| {
            Existencia {
                objeto: get_object_by_id(id_objeto, objs.clone()).unwrap(),
                cantidad: cantidad,
                procedencia: Procedencia::Tara,
            }
        }).unwrap();
        return (vec_home.get(0).cloned(), vec_tara.get(0).cloned());
    }

    pub fn print_all_stock(conn: &mut PooledConn, list: Vec<Objeto>) {
        for o in list {
            match get_stock_by_id(conn, o.id) {
                (Some(h), Some(t)) => {
                    println!("{}: x{} {} en Casa; x{} {} en Tara", o.nombre, h.cantidad, o.medida, t.cantidad, o.medida);
                }
                (Some(h), None) => {
                    println!("{}: x{} {} en Casa; x0 {} en Tara", o.nombre, h.cantidad, o.medida, o.medida);
                }
                (None, Some(t)) => {
                    println!("{}: x0 {} en Casa; x{} {} en Tara", o.nombre, o.medida, t.cantidad, o.medida);
                }
                (None, None) => {
                    //println!("{}: Sin existencias", o.nombre);
                }
            }
        }
    }
}