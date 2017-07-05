fn main() {
    // Variable, bei der der Typ automatisch ermittelt wird
    let x = "Hello";

    // Variable vom Typ int32
    let y:i32 = 54;

    println!("{}", x);

    //////////
    // Varriablen in Rust sind per default immer immutable
    // D.h. sie fest und können nicht neu zugewiesen werden
    //
    // Möchte man eine Variable im klassichen Sinne, dann muss diese mit einem Mutex
    // versehen werden. Dadurch wird das klassiche Resource-Racing wie man es aus C
    // oder C++ kennt vermiednen
    /////////

    // FALSCH - Führt zu Compilerfehler
    //    let z = "Hello";
    //z = "World";

    //RICHTIG
    let mut z = "Hello";

    z = "World";

    //  = note: `+` can't be used to concatenate two `&str` strings
    // help: to_owned() can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left.

    let text = x.to_owned() + " " + z;

    println!("{} ",text);
}
