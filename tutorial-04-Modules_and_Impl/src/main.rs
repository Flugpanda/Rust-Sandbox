// mod = importieren die Module aus dem Verszeichnis [sample_modules] -> mod.rs
mod sample_modules;

// namespace benutzen
use sample_modules::SampleImpl;

fn main(){
	// Variable Instanzieeren
	// to_string = Verändert String -> &str
	let myObject = SampleImpl::new("Hans".to_string());

	// Methode aufrufen
	myObject.hello_world();
}
