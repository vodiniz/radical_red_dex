use owo_colors::{DynColors, OwoColorize};


fn print_pokemon_logo() {

   let colors: [DynColors; 4] = [
      "#ffcb05", "#2a75bb", "#c7a008", "#3c5aa6"
    ].map(|color| color.parse().unwrap());

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
    \nEverything is also acording to pokemon radical red. If you need some help to see the commands type help.\n".color(default_color[0]));


}

fn help() {

    let colors: [DynColors; 4] = [
      "#ffcb05", "#2a75bb", "#c7a008", "#3c5aa6"
    ].map(|color| color.parse().unwrap());


    const HELP_TEXT: &str = r#"
This is a pokedex with Radical red changes integrated.
USAGE: [OPTIONS] [NAME]

[OPTIONS]
    pokemon                     - POKEMON NAME/DEX NUMBER
    route                       - POKEMON ENCOUNTER IN EACH ROUTE
    moves                       - POKEMONS MOVES
    pokemonlocation             - WHERE TO FIND A POKEMON
    changelog                   - VERSION 2.3 CHANGELOG
    evolutionchange             - COMPILED EVOLUTIONS CHANGES
    hardcore                    - HARDCORE/RESTRICTED MODE CHANGES
    tmlocations                 - TM/HM LOCATIONS
    megalocations               - MEGA STONE LOCATIONS
    itemslocation               - ITEMS LOCATIONS
    movetutors                  - MOVE TUTOR LOCATIONS
    pokemart                    - POKEMART ITEMS
    usefulitems                 - USEFUL ITEMS
    bugs                        - KNOWN BUGS LIST
    hardcoretrainers            - HARDCORE BOSS TRAINERS
    tradingpokemon              - LIST OF POKEMONS THAT CAN BE OBTAINED ONLY THROUGH TRADING
    pokemongift                 - POKEMON THAT ARE GIFTED
    eggvendors                  - POKEMONS OBTAINED THROUGH EGG VENDORS
    fossil                      - FOSSIL POKEMON OBTAINED THROUGH TRADE/RESTORING
    unobtainable                - LIST OF UNOBTAINABLE POKEMON

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

        if input.trim() == "help" {
            help();
        } else {

        }
    
        


    }

}
