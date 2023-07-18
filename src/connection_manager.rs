///Módulo que gestiona la conexión a la base de datos de mysql con el uso del archivo .env
pub mod connection_manager {
    use dotenv::from_path;
    use mysql::{Pool, PooledConn};
    use std::env;
    use std::env::VarError;

    ///Especifica el nombre de la variable de entorno del usuario de la base de datos
    const VAR_USER: &str = "DB_USER";
    ///Especifica el nombre de la variable de entorno de la contraseña de la base de datos
    const VAR_PASSWORD: &str = "DB_PASSWORD";
    ///Especifica el nombre de la variable de entorno del host de la base de datos
    const VAR_HOST: &str = "DB_HOST";

    pub fn get_envs() -> (
        Result<String, VarError>,
        Result<String, VarError>,
        Result<String, VarError>,
    ) {
        //!Obtiene las variables de entorno especificadas en las constantes VAR_USER, VAR_PASSWORD y VAR_HOST del fichero .env en forma de resultados (pueden ser correctos o erróneos).
        match from_path(".env") {
            Ok(()) => {
                return (
                    env::var(VAR_USER),
                    env::var(VAR_PASSWORD),
                    env::var(VAR_HOST),
                );
            }
            Err(e) => {
                println!(
                    "No se pudieron leer las variables de entorno desde el directorio .env: {}",
                    e
                );
                return (
                    env::var(VAR_USER),
                    env::var(VAR_PASSWORD),
                    env::var("VAR_HOST"),
                );
            }
        }
    }

    fn get_connection(
        user: String,
        password: String,
        host: String,
    ) -> Result<PooledConn, mysql::Error> {
        //!Devuelve una conexión en forma de resultado a un servidor mysql a partir del usuario, contraseña y host.
        let url = format!("mysql://{}:{}@{}", user, password, host);
        return Pool::new(url.as_str())?.get_conn();
    }

    pub fn connect(
        data: (
            Result<String, VarError>,
            Result<String, VarError>,
            Result<String, VarError>,
        ),
    ) -> Result<PooledConn, mysql::Error> {
        //!Comprueba que las variables obtenidas en get_envs() no son erróneas, y devuelve la conexión usando el método get_connection()
        match data.0 {
            Ok(user) => match data.1 {
                Ok(password) => match data.2 {
                    Ok(host) => {
                        return get_connection(user, password, host);
                    }
                    Err(e) => {
                        println!("Error al obtener la variable {}: {}", VAR_HOST, e);
                        return get_connection(user, password, format!("NULL"));
                    }
                },
                Err(e) => {
                    println!("Error al obtener la variable {}: {}", VAR_PASSWORD, e);
                    return get_connection(user, format!("NULL"), format!("NULL"));
                }
            },
            Err(e) => {
                println!("Error al obtener la variable {}: {}", VAR_USER, e);
                return get_connection(format!("NULL"), format!("NULL"), format!("NULL"));
            }
        }
    }
}
