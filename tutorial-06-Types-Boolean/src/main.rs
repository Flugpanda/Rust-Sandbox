fn main() {
    //let myVariable = true;
    let myVariable:bool = true;

    if myVariable == true {
        println!("true");
    }else {
        println!("false");
    }

    let result = isThisGreaterThan(2, 14);
    println!("{}", result);
}

fn isThisGreaterThan(x:i32, y:i32) -> bool{

    if (x > y ){
        true
    }else {
        false
    }
}