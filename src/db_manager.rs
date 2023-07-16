///Módulo que gestiona la base de datos a través de una conexión
pub mod db_manager {
    use mysql::{params, PooledConn};
    use mysql::prelude::Queryable;
    #[derive(Clone)]
    ///Estructura basada en la tabla categorias de la base de datos bodega-db
    pub struct Categoria {
        id:i32,
        nombre:String,
        descripcion:String
    }

    ///Estructura basada en la tabla objetos de la base de datos bodega-db
    pub struct Objeto {
        id:i32,
        categoria: Categoria,
        nombre:String,
        medida:String
    }

    ///Estructura basada en la tabla existencias-home o existencias-tara de la base de datos bodega-db
    pub struct Existencia {
        objeto: Objeto,
        cantidad:i32
    }

    pub fn read_objects(connection: &mut PooledConn, categories: Vec<Categoria>) -> Vec<Objeto> {
        //!Lee la tabla objetos, y la devuelve como un vector de estructuras de "Categoría". En lugar de guardar la categoría como un id, obtiene la categoría que corresponde a ese id, y la guarda dentro de la estructura "Objeto".
        let mut result: Vec<Objeto> = Vec::new();
        let list:Vec<(i32, i32, String, String)> = connection.query_map("SELECT id, categoria, nombre, medida FROM objetos;", |(id, id_cat, nombre, medida)| (id, id_cat, nombre, medida)).unwrap();
        for o in list {
            match get_category_by_id(o.1, categories.clone()) {
                Some(cat) => {
                    result.push(Objeto {
                        categoria: cat,
                        id: o.0,
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
                        id: o.0,
                        nombre: o.2,
                        medida: o.3});
                }
            }
        }
        return result;
    }

    pub fn print_objects(objects:Vec<Objeto>) {
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
        },);
    }

    pub fn delete_object(conn: &mut PooledConn, id:i32) -> Result<(), mysql::Error> {
        //!Borra un registro de la tabla objetos dado su id
        return conn.exec_drop("DELETE FROM objetos WHERE id=:id;", params! {
            "id" => id,
        },);
    }

    pub fn print_categories(categories:Vec<Categoria>) {
        //!Imprime las categorías de la tabla categorias de la base de datos.
        for c in categories {
            println!("({}) {}: {}", c.id, c.nombre, c.descripcion);
        }
    }

    pub fn insert_category(conn: &mut PooledConn, name: String, desc: String) -> Result<(), mysql::Error>{
        //!Inserta un registro de la tabla categorías dado nombre y descripción.
        return conn.exec_drop("INSERT INTO categorias (nombre, descripcion) VALUES (:nombre, :descripcion);", params! {
        "nombre" => name,
        "descripcion" => desc,
    },);
    }

    pub fn delete_category(conn: &mut PooledConn, id: i32) -> Result<(), mysql::Error> {
        //!Elimina un registro de la tabla categorías dado su id.
        return conn.exec_drop("DELETE FROM categorias WHERE id=:id;", params!("id" => id));
    }

    pub fn read_categories(connection: &mut PooledConn) -> Vec<Categoria> {
        //!Lee la tabla categorias y la devuelve como un vector de estructuras "Categoria"
        return connection.query_map("SELECT id, nombre, descripcion FROM categorias;", |(id, nombre, descripcion)| {
            Categoria {
                id, nombre, descripcion
            }
        }).unwrap();
    }

    pub fn get_category_by_id(id:i32, categories: Vec<Categoria>) -> Option<Categoria>{
        //!Busca la categoría dentro de un vector de "Categoria" que corresponde a un id dado.
        for c in categories {
            if c.id == id {
                return Some(c);
            }
        }
        return None;
    }
}