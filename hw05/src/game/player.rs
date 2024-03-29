use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::curio::Curio;
use super::room::Room;

const MAX_HP: i32 = 25;

pub enum Command {
    Go(String),
    Shoot(String),
}

pub struct Player {
    pub location: Rc<RefCell<Room>>,
    pub hp: i32,
    pub gold: i32,
    won: bool,
}

impl Player {
    pub fn new(location: Rc<RefCell<Room>>) -> Player {
        Player {
            location: location,
            hp: MAX_HP,
            gold: 0,
            won: false,
        }
    }

    pub fn use_curio(&mut self, curio: Curio) {
        match curio {
            Curio::Chest(gold) => {
                println!("You open the chest and gain {} gold.", gold);
                self.gold += gold;
            }
            Curio::SpikeTrap(dmg) => {
                println!("You take {} damage from the spikes.", dmg);
                self.hp -= dmg;
            }
            Curio::Food(heal) => {
                println!(
                    "You shove a wall chicken into your gob and heal {} HP.",
                    heal
                );
                self.hp = std::cmp::min(MAX_HP, self.hp + heal);
            }
            Curio::IronMaiden(sub, dmg) => {
                println!("Dude I love Iron Maiden! This one's pointy, though.");
                println!("You cut yourself on the spikes inside for {} damage.", dmg);
                self.hp -= dmg;
                println!("You open the iron maiden and...");
                self.use_curio(*sub);
            }
            Curio::FallenAdventurer(sub) => {
                println!("You pilfer the corpse and...");
                self.use_curio(*sub);
            }
        }
    }

    /// Execute the given command on the player and board state.
    pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
        match cmd {
            Command::Go(room) => {
                let dst_room = self.find_room(room)?;
                while let Some(curio) = dst_room.borrow_mut().consume_content() {
                    self.use_curio(curio)
                }

                self.location = dst_room;
            }
            Command::Shoot(room) => {
                let dst_room = self.find_room(room)?;
                if dst_room.borrow().wumpus {
                    println!("You just kill the wumpus :)");
                    dst_room.borrow_mut().wumpus = false;
                    self.won = true;
                } else {
                    println!("Wumpus is not there :(")
                }
            }
        }
        Ok(())
    }

    /// Find one of the neighbors of the current room based on its name. Case insensitive.
    fn find_room(&self, room: String) -> Result<Rc<RefCell<Room>>, ()> {
        let room = room.to_lowercase();
        for hall in self.location.borrow().halls.iter() {
            let other_side = hall.other(&self.location.borrow());
            if other_side.borrow().name.to_lowercase() == room {
                return Ok(other_side);
            }
        }
        Err(())
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "You find yourself in {}.\n\nYou have {} HP and {} gold.",
            self.location.borrow().name,
            self.hp,
            self.gold
        )
    }
}
