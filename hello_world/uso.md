# Creacion de ejecutable con rustc
Con **rustc** es posible crear un ejecutable.
``` zsh
$ rustc main.rs
$ ./main
> Hello World.
```
De esta forma se llama al compilador con el código fuente, similar a como funciona gcc o g++, generando asi el ejecutable.  
Compilar con rustc es util para programas simples.  

# Explicacion del codigo
## Funcion
~~~ rust
fn main() {

}
~~~
De esta forma se define una funcion llamada **main**, que es siempre lo primero en correr en un programa de Rust.  
En este caso la funcion que no regresa nada y no cuenta con parámetros, que serían definidos dentro de los parentesis ().  
El cuerpo de la funcion se encuentra dentro de las llaves {} y es convencion iniciarlos en la misma linea donde se define la funcion  
y con un espacio entre el nombre de la funcion y la llave.  
Tambien es posible utilizar **rustfmt** para acomodar el codigo si no piensas seguir el estandar.  

## Macro
~~~ rust
println!("Hello World.");
~~~
Esta linea de codigo imprime en la pantalla la palabra "Hello World.".  
La indentación en Rust se hace con 4 espacios.  
println! es una macro, que de ser una funcion, se usaria de la forma println() sin el signo de admiración "!".  
Las strings van entre comillas " ".
Las lineas de código terminan con punto y coma ";".

