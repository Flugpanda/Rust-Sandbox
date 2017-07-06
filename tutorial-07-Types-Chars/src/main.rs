fn main(){

    let bar = 'a';
    let foo: char = 'b';

    let heart_eyed_cat = '😻';
    println!("{:?}", heart_eyed_cat);


    let hello = String::from("Hello, world!");
    let chars = hello.chars();
    println!("chars : {:?}",chars);

    for letter in hello.chars() {
        println!("{:?}",letter.to_string() );
    }
}
