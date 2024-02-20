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

# TOML (Tom's Obvious, Minimal Language)
~~~ txt
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
~~~

