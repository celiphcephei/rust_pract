# Macros
* print!("Texto {}", variable); Imprime texto en pantalla.  
* prinln!("Texto {}", variable); Imprime texto en pantalla con salto de linea.  
* asssert_eq!(variable, valor_esperado); Se termina el programa si la variable no cuenta con el valor esperado.  
* assert!(variable == valor);

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
* Desde 8 hasta 128 bits.  
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

### Architecture dependent
usize / isize  
Cambia a 32 bits o a 64 bits dependiendo de la arquitectura.  
En pointers usize es útil porque asegura que podemos acceder a cualquier tipo de dato sin importar el numero de bytes.  
#### Word
A lo que refiere con tamaño de palabra es lo siguiente:  
El procesador no lee los bits de 1 a 1, sino que accede a cierto numero de bits de forma simultanea, esto es la palabra.  
Tamaño de palabra de acuerdo a la arquitectura:  
* 32 bits: 4 bytes word. (Lee 32 bits a la vez).  
* 64 bits: 8 bytes word. (Lee 64 bits a la vez).  

## Notas especiales
No podemos asignar un valor de una variable de un tipo a una de otro tipo, resulta obvio, pero un ejemplo es el siguiente.  
~~~ rust
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x; // No es posible hacer esto
}
~~~

Podemos asignar valores diciendo el numero y el tipo de dato que queremos que sea poniendo algo como "**_u8**".  
Por ejemplo, si queremos un numero **6** que sea de tipo **u8** simplemente escribirmos **8_u8** tambien puede omitirse el guion bajo.  
De igual manera podemos cambiar el tipo de cualquier numero con la palabra clave **as**.  
Por ejemplo:  
~~~ rust
fn main() {
    let v: u16 = 38_u8 as u16;
}
~~~
En este codigo, declaramos que queremos un tipo u16 pero le asignamos un tipo u8, pero simplemente lo cambiamos al tipo esperado u16 con **as**.  

Podemos hacer operaciones con los valores en distintas notaciones numericas, por ejemplo:  
~~~ rust
fn main(){
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255 = 1597
    assert!(v == 1597);
    println!("Success!");
}
~~~


# Trabajar con Char
De esta forma podemos obtener el **peso** en **bytes**.  
**El peso de un char es de 4 bytes, puede tener cualquier valor en la escala de unicode.**  
~~~ rust
use std::mem::size_of_val;

fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4);
}
~~~

En Rust, nos referimos exclusivamente a los char con las comillas simples ' ', y a las strings con las comillas dobles " ".  
~~~ rust
fn main() {
    let c1: char = 'a'; // Si usamos comillas dobles dara error porque se declaro que la funcion recibe un char
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
~~~


# Unit type
Es un tipo que no da ningun valor, su tamaño es es 0 bytes y se puede usar con una funcion que no regrese ningun valor y regrese de esa forma un unit type.  
Basicamente, una funcion que no regresa nada, implicitamente regresa un unit type.  
~~~ rust
fn main() {
    let _v: () = (); // Una variable de tipo unit type que tiene un unit type, una tupa vacia
    let v = (2, 3); // Una variable con una tupa de dos numeros tipo i32. let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicity_ret_unit());
    println!("Success");
}

fn implicity_ret_unit() {
    println!("I will return a ()");
}

fn explicity_ret_unit() -> () {
    println!("I will return a ()");
}
~~~


# Operaciones con bits
Podemos realizar operaciones booleanas con bits con los operadores OR | AND & y XOR ^  
Si utilizamos los numeros con el formato de 0b0101.  
Tambien podemos realizar operaciones de desplazamiento con los operadores "<<"(izquierda) ">>"(derecha)  
En las mismas se pueden usar los numeros en alguna notación numerica con hexadecimal y tambien va a funcionar.  
Ejemplo:  
~~~ rust
fn main() {
    println!("0011 AND 0101 = {}", 0b0011u32 & 0b0101); // 0001
    println!("0011 OR 0101 = {}", 0b0011u32 | 0b0101); // 0111
    println!("0011 XOR 0101 = {}", 0b0011U32 ^ 0b0101); // 0110
    println!("1 << 5 = {}", 1u32 << 5); // 32
    println!("0x80 >> 2 = {}", 0x80u32 >> 2); // 0x20
}
~~~



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


# Bucles
**Bucle for**  
~~~ rust
fn main() {
    let x: i32 = 0;

    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' { // El signo de = dice que queremos incluir el ultimo valor dentro del rango
        println!("{}", c); // Podemos imprimir el valor numerico ASCII de cada caracter si lo pasamos como integer con as, println!("{}", c as u8);
    }
}
~~~


# Palabras especiales
**MAX** Vale el maximo numero posible del tipo de dato que está usando.  
Por ejemplo:  
* i8::MAX = 127  
* u8::MAX = 255  


