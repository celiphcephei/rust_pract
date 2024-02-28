# Cargo
~~~ zsh
cargo new hello_cargo
~~~
Esta es la forma de crear un proyecto nuevo con **Cargo**  
Esto genera un directorio con el nombre elegido, contiendo dentro otro directorio y dos archivos nuevos:  
* Un _archivo_ **Cargo.toml**.
* Un _directorio_ **src**.
* Un _archivo_ **main.rs** dentro del _directorio_ **src**.  
Esto también genera un archivo .gitignore unicamente si no corres el comando dentro de un archivo que ya sea un repositorio de github.  
Tambien puede anularse este comportamiento usando --vcs=git.  

Cargo espera que todo el código fuente se encuentre dentro de la carpeta **src** y el nivel superior sea utilizado unicamente para informacion de licencias,  
archivos readme, archivos de configuración y nada que tenga relación directa con el código fuente.  
Cargo ayuda a mantener un orden en el proyecto.  

## cargo build
~~~zsh
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
~~~
**cargo build** nos crea una carpe

# TOML (Tom's Obvious, Minimal Language)
~~~ txt
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
~~~
**[package]** Es un header que indica que las siguientes instrucciones van a configurar los package.  
**[dependencies]** Es una sección para listar las dependencias del proyecto.  
