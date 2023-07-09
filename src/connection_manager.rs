pub mod connection_manager {
    use std::env;
    use std::env::VarError;
    use dotenv::{Error, from_path};
    use mysql::{Pool, PooledConn};

    pub fn get_envs() -> (Result<String, VarError>, Result<String, VarError>, Result<String, VarError>) {
    match from_path(".env") {
        Ok(()) => {
            return (env::var("DB_USER"), env::var("DB_PASSWORD"), env::var("DB_HOST"));
        }
        Err(e) => {
            println!("No se pudieron leer las variables de entorno desde el directorio .env: {}", e);
            return (env::var("DB_USER"), env::var("DB_PASSWORD"), env::var("DB_HOST"));
            }
        }
    }

    fn get_connection(user:String, password:String, host:String) -> Result<PooledConn, mysql::Error>{
        let url = format!("mysql://{}:{}@{}", user, password, host);
        return Pool::new(url.as_str())?.get_conn();
    }

    pub fn connect(data:(Result<String, VarError>, Result<String, VarError>, Result<String, VarError>)) -> Result<PooledConn, mysql::Error>{
        match data.0 {
            Ok(user) => {
                match data.1 {
                    Ok(password) => {
                        match data.2 {
                            Ok(host) => {
                                return get_connection(user, password, host);
                            }
                            Err(e) => {
                                println!("Error al obtener la varuable DB_HOST: {}", e);
                                return get_connection(user, password, format!("NULL"));
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error al obtener la variable DB_PASSWORD: {}", e);
                        return get_connection(user, format!("NULL"), format!("NULL"));
                    }
                }
            }
            Err(e) => {
                println!("Error al obtener la variable DB_USER: {}", e);
                return get_connection(format!("NULL"), format!("NULL"), format!("NULL"));
            }
        }
    }
}