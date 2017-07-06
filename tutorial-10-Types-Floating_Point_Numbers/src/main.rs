fn main() {
    // f32 enspricht einem float
    let x:f32 = std::f32::MAX;

    // f64 enspricht einem Double
    let y:f64 = std::f64::MAX;

    println!("f32 enspricht einem FLOAT und hat {:?} Bits.", x);
    println!("f64 enspricht einem DOUBLE und hat {:?} Bits.", y);
}
