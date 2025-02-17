fn greet_world () {
    let chinese = "你好";
    let english = "Hello";
    let japanese = "こんにちは";
    let french = "Bonjour";
    let german = "Hallo";
    let spanish = "Hola";
    let italian = "Ciao";
    let regions = [chinese, english, japanese, french, german, spanish, italian];
    for region in regions { 
        println!("{}", region);
    }
}

fn main () {
    greet_world();
}
