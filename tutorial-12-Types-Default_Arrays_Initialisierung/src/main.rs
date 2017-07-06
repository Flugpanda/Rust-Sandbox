fn main() {
    // Arrays in Rust sind per Definition immer immutable.
    // D.h. sie können nachträglich nicht geändert werden!
    let array = [1,2,3,4,8,9,12];
    println!("Ein einfaches Array: {:?}",array );

    let array_init = [15;4];
    println!("Das das Array let array_init = [<Zahl>;<Dimension>] ist 4 Elemente gross und jedes Element wird mit der Zahl 15 befüllt. {:?}", array_init);

    // Array mit fester Größe und einem Datentyp
    let array_mit_typ: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array_mit_typ {:?}", array_mit_typ);
    // Arrays are stack allocated
    println!("array occupies {} bytes", std::mem::size_of_val(&array_mit_typ));


}
