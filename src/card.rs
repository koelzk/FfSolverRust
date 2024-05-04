use std::fmt;

pub const ACE_RANK:u8 = 1;
pub const JACK_RANK:u8 = 11;
pub const QUEEN_RANK:u8 = 12;
pub const KING_RANK:u8 = 13;
pub const MINOR_ARC_MIN_RANK:u8 = 2;
pub const MINOR_ARC_MAX_RANK:u8 = KING_RANK;
pub const MAJOR_ARC_MIN_RANK:u8 = 0;
pub const MAJOR_ARC_MAX_RANK:u8 = 21;

//const SUIT_STRINGS: &[&str; 5] = &["R", "G", "B", "Y", ""];
const SUIT_STRINGS: &[&str; 5] = &["♥", "♣", "♠", "♦", ""];
static MAJOR_ARC_RANK_STRINGS : &[&str; MAJOR_ARC_MAX_RANK as usize + 1] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21"];
static MINOR_ARC_RANK_STRINGS : &[&str; KING_RANK as usize + 1] = &["?", "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    value: u8
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        if let Suit::MajorArc = suit {
            if rank < MAJOR_ARC_MIN_RANK || rank > MAJOR_ARC_MAX_RANK {
                panic!("Rank is out of range [0:21]");
            }
        }
        else {
            if rank < ACE_RANK || rank > KING_RANK {
                panic!("Rank is out of range [1:13]");
            }
        }

        Card {
            value: ((suit as u8) << 5) | rank
        }
    }

    pub fn suit(&self) -> Suit {
        let i = self.value >> 5;
        Suit::from(i)
    }

    pub fn rank(&self) -> u8 {
        self.value & 0b1_1111
    }

    pub fn can_place_on(&self, other: &Card) -> bool {
        if self.suit() != other.suit()
        {
            return false;
        }

        (self.rank() == other.rank() - 1) || (self.rank() == other.rank() + 1)
    }

    pub fn create_deck() -> impl Iterator<Item = Card> {
        let major_arc_cards =
            (MAJOR_ARC_MIN_RANK..=MAJOR_ARC_MAX_RANK).into_iter().map(|i| Card::new(i, Suit::MajorArc));

        const MINOR_ARC_MIN_RANK: u8 = 2;
        const MINOR_ARC_COUNT: u8 = KING_RANK - MINOR_ARC_MIN_RANK + 1;
        let minor_arc_cards =
            (0..4 * MINOR_ARC_COUNT).into_iter().map(|i| Card::new(i % MINOR_ARC_COUNT + MINOR_ARC_MIN_RANK, Suit::from(i / MINOR_ARC_COUNT)));

        major_arc_cards.chain(minor_arc_cards)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = self.suit();
        let suit_string = SUIT_STRINGS[suit as usize];
        let rank_string = match suit {
            Suit::MajorArc => MAJOR_ARC_RANK_STRINGS[self.rank() as usize],
            _ => MINOR_ARC_RANK_STRINGS[self.rank() as usize]
        };

        write!(f, "{rank_string}{suit_string}")
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_string();
        let v = self.value;
        write!(f, "{s} ({v})")
    }
}

impl From<&Card> for u8 {
    fn from(card: &Card) -> Self {
        card.value
    }
}

/// Card suit
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Suit {
    /// Cups, Minor Arcana
    Red = 0,
    /// Batons, Minor Arcana
    Green = 1,
    /// Swords, Minor Arcana
    Blue = 2,
    /// Coins, Minor Arcana
    Yellow = 3,
    /// Major Arcana
    MajorArc = 4
}

impl From<u8> for Suit {
    fn from(value: u8) -> Self {
        match value {
            0 => Suit::Red,
            1 => Suit::Green,
            2 => Suit::Blue,
            3 => Suit::Yellow,
            4 => Suit::MajorArc,
            _ => panic!("Invalid suit value")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card::*;

    #[test]
    pub fn new() {
        let card = Card::new(6, Suit::Green);
        assert_eq!(6, card.rank());
        assert_eq!(Suit::Green, card.suit());
    }

    #[test]
    pub fn display() {
        assert_eq!("2♦", format!("{}", Card::new(2, Suit::Yellow)));
        assert_eq!("A♣", format!("{}", Card::new(ACE_RANK, Suit::Green)));
        assert_eq!("J♥", format!("{}", Card::new(JACK_RANK, Suit::Red)));
        assert_eq!("Q♦", format!("{}", Card::new(QUEEN_RANK, Suit::Yellow)));
        assert_eq!("K♠", format!("{}", Card::new(KING_RANK, Suit::Blue)));
        assert_eq!("21", format!("{}", Card::new(21, Suit::MajorArc)));
    }

    #[test]
    pub fn can_place_on() {
        assert_eq!(true, Card::new(2, Suit::MajorArc).can_place_on(&Card::new(3, Suit::MajorArc)));
        assert_eq!(true, Card::new(3, Suit::MajorArc).can_place_on(&Card::new(2, Suit::MajorArc)));
        assert_eq!(false, Card::new(2, Suit::MajorArc).can_place_on(&Card::new(4, Suit::MajorArc)));
        assert_eq!(false, Card::new(6, Suit::MajorArc).can_place_on(&Card::new(4, Suit::MajorArc)));

        assert_eq!(true, Card::new(2, Suit::Red).can_place_on(&Card::new(3, Suit::Red)));
        assert_eq!(true, Card::new(3, Suit::Red).can_place_on(&Card::new(2, Suit::Red)));
        assert_eq!(true, Card::new(JACK_RANK, Suit::Green).can_place_on(&Card::new(QUEEN_RANK, Suit::Green)));
        assert_eq!(true, Card::new(QUEEN_RANK, Suit::Green).can_place_on(&Card::new(JACK_RANK, Suit::Green)));
        assert_eq!(false, Card::new(JACK_RANK, Suit::Green).can_place_on(&Card::new(QUEEN_RANK, Suit::Red)));
        assert_eq!(false, Card::new(QUEEN_RANK, Suit::Green).can_place_on(&Card::new(JACK_RANK, Suit::Blue)));
    }

    #[test]
    pub fn create_deck() {
        let cards = &Card::create_deck().collect::<Vec<Card>>();

        assert_eq!(70, cards.len());
        assert!(cards.iter().take(22).enumerate().all(|(i, c)| c.rank() == i as u8 && c.suit() == Suit::MajorArc));
        assert!(cards.iter().skip(22).enumerate().all(|(i, c)| c.rank() == (i % 12 + 2) as u8 && c.suit() == Suit::from(i as u8 / 12)));
    }
}
