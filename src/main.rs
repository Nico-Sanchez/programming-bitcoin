mod finite_field;
mod point;

// use Point::

fn main() {
    let punto_infinito = point::Point::new(None, None, 1, 2);
    println!("{:?}", punto_infinito);
    let punto_valido = point::Point::new(Option::<i32>::from(-1), Option::<i32>::from(-1), 5, 7);
    println!("{:?}", punto_valido);
    let punto_valido2 = point::Point::new(Option::<i32>::from(-1), Option::<i32>::from(-1), 5, 7); 
    let resultado = &punto_valido.eq(punto_valido2);
    let resultado2 = &punto_valido.eq(punto_infinito);
    println!("{:?}", resultado);
    println!("{:?}", resultado2);
    

    let punto_infinito2 = point::Point::new(None, None, 5, 7);
    let punto_valido2 = point::Point::new(Option::<i32>::from(-1), Option::<i32>::from(-1), 5, 7);

    let suma = &punto_infinito2.add(punto_valido2);
    println!("{:?}", suma);
    
    println!("Champions league!");
    let punto_invalido = point::Point::new(Option::<i32>::from(2), Option::<i32>::from(4), 5, 7);
    println!("{:?}", punto_invalido);
}
