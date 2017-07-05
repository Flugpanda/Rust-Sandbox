fn main() {
    // Funktionsaufruf
    simple_function();
    
    let x = 5;
    let y = 4;
    
    println!("x = {}",x);
    println!("y = {}",y);
    
    // Funktionsaufruf mit Parametern
    let z = complex_function(x,y);
    println!("x + y = {}",z);
}

fn simple_function(){
    println!("Hello, world!");
}

// Rückgabewerte in Rust werden mit einem Pfeil gekennzeichnet
fn complex_function(x:i32, y:i32) -> i32{
    let result = x + y;
    
    // der Rückgabewerte erhält kein Semikolon am ende
    result
}
