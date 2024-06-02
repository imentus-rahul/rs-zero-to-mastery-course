// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    // let some_tile = Tile::Brick(BrickStyle::Red);
    // let some_tile = Tile::Dirt;
    // let some_tile = Tile::Treasure(TreasureChest {
    //     content: TreasureItem::Gold,
    //     amount: 100,
    // });
    let some_tile = Tile::Water(Pressure(7));
    match some_tile {
        Tile::Brick(brick @ BrickStyle::Red | brick @ BrickStyle::Gray) => {
            println!("The brick color is: {:?}", brick)
        }
        Tile::Brick(other) => println!("{:?} brick", other),
        Tile::Dirt | Tile::Grass | Tile::Sand => println!("Ground Tile"),
        Tile::Treasure(TreasureChest { amount, .. }) if amount >= 100 => println!("Lots of gold!"),
        Tile::Water(Pressure(pressLvl)) if pressLvl >= 10 => println!("High water pressure!"),
        Tile::Water(Pressure(pressLvl)) if pressLvl < 10 => {
            println!("Water pressure level: {:?}", pressLvl)
        }
        _ => println!("else anything"),
    }
}
