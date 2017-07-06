fn main() {
    // Arrays in Rust sind per Definition immer immutable.
    // D.h. sie können nachträglich nicht geändert werden!
    let array = [1,2,3,4,8,9,12];
    println!("Ein einfaches Array: {:?}",array );

    // Zugriff auf ein Element des Arrays
    println!("Zugrif auf das dritte Element: {:?}", array[2]);

    // Um die Elemente eines Arrays ändern zu können, bedarf es eines Mutex
    // mut = Mutex
    let mut einfache_liste = [10,9,8,7,5,4];
    println!("Das neue Array vor der Änderung: {:?}",einfache_liste );

    einfache_liste[0] = 100;
    println!("Das neue Array nach der Änderung: {:?}",einfache_liste );

}
