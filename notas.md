# Macros
* print!("Texto {}", variable); Imprime texto en pantalla.  
* prinln!("Texto {}", variable); Imprime texto en pantalla con salto de linea.  
* asssert_eq!(variable, valor_esperado); Se termina el programa si la variable no cuenta con el valor esperado.  

# Tipos de datos
* i32
* i64
* &str

<table>
    <tr>
        <th>Lenght</th>
        <th>Signed</th>
    </tr>
</table>

# Omitir variables sin usar
Puede agregarse al inicio del programa el siguiente comentario:  
\#[allow(unused_variables)]  
O puede simplemente ponerse un "_" a la izquierda del nombre de la variable:  
let \_variable: i32 = 1;

# Destructuring
Podemos separar los valores de una tupla con let de la siguiente forma:  
~~~ rust
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;
}
~~~

Podemos declarar dos variables en una sola linea:  
~~~ rust
let (x, y); // Es equivalente a lo siguiente:
let x;
let y;
~~~
Podemos hacer el Destructuring tomando solo uno de los dos valores y pueden usarse listas tambien:  
~~~ rust
fn main() {
    let (x, y);

    (x,..) = (3, 4);
    [.., y] = [1, 2];
    asssert_eq!([x, y], [3, 2]);

    println!("Success!");
}
~~~


