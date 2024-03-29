use std::rc::Rc;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    // TODO: Implement the necessary methods for Rooms.
    pub fn new() -> Self {
        Room {
            name: String::new(),
            contents: Vec::new(),
            halls: Vec::new(),
            wumpus: false,
        }
    }

    pub fn consume_content(&mut self) -> Option<Curio> {
        self.contents.pop()
    }

    pub fn neighbors_string(&self) -> String {
        let description = format!(
            "{}",
            self.halls
                .iter()
                .map(|hall| hall.other(&self).borrow().name.clone())
                .collect::<Vec<_>>()
                .join(", "),
        );

        description
    }
}
