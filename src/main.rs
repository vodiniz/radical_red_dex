use owo_colors::{DynColors, OwoColorize};


const colors: [DynColors; 4] = [
      "#ffcb05", "#2a75bb", "#c7a008", "#3c5aa6"
    ].map(|color| color.parse().unwrap());

fn print_pokemon_logo() {

    let pokemon_ascii = include_str!("../pokemon_ascii.txt");

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

    println!("\n\n\n\n{}", "This is a quick pokedex in which you can look about pokemons, \
    evolution levels, moves, types, routes encounters, itens locations and much more. \
    \nEverything is also acording to pokemon radical red. If you need some help to see the commands type help.".color(default_color[0]));


}

fn help() {
    const HELP_TEXT: &str = r#"
\n\n\n
This is a pokedex with Radical red changes integrated.
USAGE: [OPTIONS] [NAME]

[OPTIONS]
    POKEMON                     - POKEMON NAME/DEX NUMBER
    ROUTE                       - POKEMON ENCOUNTER IN EACH ROUTE
    MOVES                       - POKEMONS MOVES
    POKEMONLOCATION             - WHERE TO FIND A POKEMON
    CHANGELOG                   - VERSION 2.3 CHANGELOG
    EVOLUTIONCHANGE             - COMPILED EVOLUTIONS CHANGES
    HARDCORE                    - HARDCORE/RESTRICTED MODE CHANGES
    ITEMSLOCATION               - ITEMS/TM/MOVE TUTOR LOCATIONS
    BUGS                        - KNOW BUGS LIST
    HARDCORETRAINERS            - HARDCORE BOSS TRAINERS
    "#;

    println!("{}", HELP_TEXT.color(colors[0]));
}


fn main() {

    print_pokemon_logo();
    introduction();

    let mut running: bool = true;

    while running {

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input {
            "help" => help(),
        }
    
        


    }

}
