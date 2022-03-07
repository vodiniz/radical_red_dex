use std::fs;
use owo_colors::{DynColors, OwoColorize};

const POKEMON_LOGO: &str = r#"

"#;





fn print_pokemon_logo() {

    let pokemon_ascii = fs::read_to_string("pokemon_ascii.txt")
        .expect("Something went wrong reading the file");

    let colors: [DynColors; 4] = [
      "#ffcb05", "#2a75bb", "#c7a008", "#3c5aa6"
    ].map(|color| color.parse().unwrap());


    for line in pokemon_ascii.split_inclusive("\n") {
        for character in line.chars() {
              match character {
                  '#' => print!("{}", character.color(colors[0])),
                  '.' => print!("{}", character.color(colors[3])),
                  c if "*%".contains(c) => print!("{}", character.color(colors[2])),
                  c if "+-:=".contains(c) => print!("{}", character.color(colors[1])),
                  c => print!("{}", c),
              }
      }
    }

}

fn introduction() {
    let default_color: [DynColors; 1] = [
      "#ffcb05"
    ].map(|color| color.parse().unwrap());


}




fn main() {

    print_pokemon_logo();

    let mut running: bool = true;

    while running {
    
    }

    

}
