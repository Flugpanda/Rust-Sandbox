// in Rust gibt es keine Klassen oder Objektorientierung!!

// anlegen eines Structs
pub struct SampleImpl {
	myName:String
}

// Anlegen einer Implementierung
impl SampleImpl {

	// Konstruktormethode
	// Vergleichbar einem Konstruktor in Java/C++/C#
	// new = Konvention
	// -> liefert ein "Objekt"
	pub fn new(name:String) -> SampleImpl{
		SampleImpl{myName: name}
	}

	// nicht statische Methode
	// &self = Referenz auf instanziertes Objekt
	pub fn hello_world(&self){
		println!("My Name is: {:?}", self.myName);
	}
}
