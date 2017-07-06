fn main() {
    let maxInt32 : i32 = std::i32::MAX;

    println!("Int32 maximale Größe: {:?}", maxInt32);


    // isize und i64 sidn gleich groß, da das Betriebssystem ein 64-Bit Version ist
    let maxU64 :i64 = std::i64::MAX;
    let maxOS :isize = std::isize::MAX;

    println!("Int64: {:?}", maxU64);
    println!("isize: {:?}", maxOS);

}
