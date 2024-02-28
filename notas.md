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
        <th>Unsigned</th>
    </tr>
    <tr>
        <td>8-bit</td>
        <td>i8</td>
        <td>u8</td>
    </tr>
    <tr>
        <td>16-bit</td>
        <td>i16</td>
        <td>u16</td>
    </tr>
    <tr>
        <td>32-bit</td>
        <td>i32</td>
        <td>u32</td>
    </tr>
    <tr>
        <td>64-bit</td>
        <td>i64</td>
        <td>u64</td>
    </tr>
    <tr>
        <td>128-bit</td>
        <td>i128</td>
        <td>u128</td>
    </tr>
    <tr>
        <td>arch</td>
        <td>isize</td>
        <td>usize</td>
    </tr>
</table>

## Default types
* Integers: i32
* Floats: f64

## Range of bits

**8 bits**.  
* Unsigned: 0 / 255.  
* Signed: -128 / 127.  
**16 bits**.  
* Unsigned: 0 / 65535.  
* Signed: -32768 / 32767.  
**32 bits**.  
* Unsigned: 0 / 4294967295.  
* Signed: -2147483648 / 2147483647.  
**64 bits**.  
* Unsigned: 0 / 18446744073709551615.  
* Signed: -9223372036854775808 / 9223372036854775807.  
**128 bits**.  
* Unsigned: 0 / 340282366920938463463374607431768211455.  
* Signed: -170141183460469231731687303715884105728 / 170141183460469231731687303715884105727.  

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


