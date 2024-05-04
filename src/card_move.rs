use std::fmt::Display;

/// Represents the cell as source or target location
pub const CELL: u8 = 255;

/// Represents the foundation as target location
pub const FOUNDATION: u8 = 254;

#[derive(Debug, Eq, PartialEq)]
pub struct CardMove {
    from: u8,
    to: u8,
    count: u8
}

impl CardMove {
    pub fn new(from: u8, to: u8, count: u8) -> CardMove {
        if from > 10 && from != CELL {
            panic!("Invalid value for 'from' specified.")
        }

        if to > 10 && to != CELL && to != FOUNDATION {
            panic!("Invalid value for 'to' specified.")
        }

        if count == 0 {
            panic!("Invalid value for 'count' specified.")
        }

        CardMove {
            from,
            to,
            count,
        }
    }
    pub fn from(&self) -> u8 {
        self.from
    }

    pub fn to(&self) -> u8 {
        self.to
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    fn get_location_string(index: u8) -> String {
        match index {
            CELL => String::from("cell"),
            FOUNDATION => String::from("foundation"),
            _ => format!("cascade {index}")
        }
    }
}

impl Display for CardMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_string = CardMove::get_location_string(self.from);
        let to_string = CardMove::get_location_string(self.to);
        
        match self.count {
            1 => write!(f, "Move card from {from_string} to {to_string}"),
            _ => {
                let count = self.count;
                write!(f, "Move {count} cards from {from_string} to {to_string}")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card_move::*;
    
    #[test]
    pub fn new() {
        let card_move = CardMove::new(1, 4, 5);
        assert_eq!(1, card_move.from());
        assert_eq!(4, card_move.to());
        assert_eq!(5, card_move.count());
    }

    #[test]
    pub fn display() {
        assert_eq!("Move 5 cards from cell to foundation", CardMove::new(CELL, FOUNDATION, 5).to_string());
        assert_eq!("Move card from cell to foundation", CardMove::new(CELL, FOUNDATION, 1).to_string());
        assert_eq!("Move 3 cards from cascade 0 to cascade 10", CardMove::new(0, 10, 3).to_string());
    }    
}