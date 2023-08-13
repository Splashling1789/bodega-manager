CREATE DATABASE IF NOT EXISTS bodega;
USE bodega;
CREATE TABLE IF NOT EXISTS categorias (
                                          id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL ,
                                          nombre VARCHAR(255) NOT NULL,
                                          descripcion TEXT
);

CREATE TABLE IF NOT EXISTS objetos (
                                       id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
                                       categoria BIGINT NOT NULL,
                                       nombre VARCHAR(255) NOT NULL,
                                       medida VARCHAR(255) NOT NULL,
                                       FOREIGN KEY (categoria) REFERENCES categorias(id)
);

CREATE TABLE IF NOT EXISTS existencias_home (
                                                id_objeto BIGINT PRIMARY KEY NOT NULL,
                                                cantidad DOUBLE(8,2),
                                                FOREIGN KEY (id_objeto)  REFERENCES objetos(id)
);

CREATE TABLE IF NOT EXISTS existencias_tara (
                                                id_objeto BIGINT PRIMARY KEY NOT NULL,
                                                cantidad DOUBLE(8,2),
                                                FOREIGN KEY (id_objeto)  REFERENCES objetos(id)
);