use std::collections::HashSet;

use itertools::Itertools;
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use thiserror::Error;

use crate::*;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("the string `{0}` is not a valid card")]
    InvalidCard(String),
    #[error("card sequence has duplicates")]
    DuplicateCards,
    #[error("unexpected card {card} (cascade {column}, row {row})")]
    UnexpectedCard {
        column: u8,
        row: u8,
        card: Card
    },
}

pub fn parse_board(cascade_string: &str, cell_string: Option<&str>) -> Result<Board, ParseError> {

    let card_strings = cascade_string.split_ascii_whitespace();
    let mut cascades: [Vec<Card>; CASCADE_COUNT as usize] = Default::default();

    for (index, card_str) in card_strings.enumerate() {
        let i = index % CASCADE_COUNT as usize;
        let j = index / CASCADE_COUNT as usize;

        let card = parse_card(card_str)?;
        if let Some(c) = card {

            if cascades[i].len() != j {
                return Err(ParseError::UnexpectedCard { column: i as u8, row: j as u8, card: c })
            }

            cascades[i].push(c);
        }
    }

    let cell = match cell_string {
        Some(cs) => parse_card(cs)?,
        None => None
    };

    let mut card_set = HashSet::new();
    let mut has_duplicates = cascades.iter().flatten().any(|&c| !card_set.insert(u8::from(&c)));

    if let Some(card) = cell {
        has_duplicates &= !card_set.contains(&u8::from(&card));
    }
    
    if has_duplicates {
        return Err(ParseError::DuplicateCards);
    }

    Ok(Board::new(cascades, cell))
}

fn parse_rank(r: &[u8], &suit: &Suit) -> Option<u8> {
    match r {
        [b @ b'0'..=b'9'] => Some(b - b'0'),
        [b'J'] if suit != Suit::MajorArc => Some(JACK_RANK),
        [b'Q'] if suit != Suit::MajorArc => Some(QUEEN_RANK),
        [b'K'] if suit != Suit::MajorArc => Some(KING_RANK),
        [b'1', b'0'] if suit != Suit::MajorArc => Some(10),
        [b1 @ b'1'..=b'2', b2 @ b'0'..=b'9'] if suit == Suit::MajorArc => Some((b1 - b'0') * 10 + (b2 - b'0')),
        _ => None
    }
}

fn parse_suit(seq: &[u8]) -> Option<Suit> {
    match seq[seq.len() - 1] {
        b'0'..=b'9' => Some(Suit::MajorArc),
        b'R' => Some(Suit::Red),
        b'G' => Some(Suit::Green),
        b'B' => Some(Suit::Blue),
        b'Y' => Some(Suit::Yellow),
        _ => None
    }
}

pub fn parse_card(card_str: &str) -> Result<Option<Card>, ParseError> {
    if card_str.is_empty() || card_str.len() > 3 {
        return Err(ParseError::InvalidCard(card_str.to_owned()));
    }

    if card_str == "-" {
        return Ok(None);
    }

    let seq = card_str.as_bytes();
    let suit = parse_suit(seq)
        .ok_or(ParseError::InvalidCard(card_str.to_owned()))?;

    match suit {
        Suit::MajorArc => {
            match parse_rank(seq, &suit) {
                Some(rank) if (MAJOR_ARC_MIN_RANK..=MAJOR_ARC_MAX_RANK).contains(&rank) => Ok(Some(Card::new(rank, Suit::MajorArc))),
                _ => Err(ParseError::InvalidCard(card_str.to_owned()))
            }
        }
        _ => {
            match parse_rank(&seq[0..seq.len() - 1], &suit) {
                Some(rank) if (MINOR_ARC_MIN_RANK..=MINOR_ARC_MAX_RANK).contains(&rank) => Ok(Some(Card::new(rank, suit))),
                _ => Err(ParseError::InvalidCard(card_str.to_owned()))
            }
        }
    }
}

pub fn create_random_board(seed: u64) -> Board {
    let hashes = (0..70)
        .scan(Xoshiro256PlusPlus::seed_from_u64(seed), |rng, _| Some(rng.next_u64()));

    let deck = Card::create_deck()
        .zip(hashes)
        .sorted_by_key(|(_, i)| *i)
        .map(|(card, _)| card)
        .collect::<Vec<Card>>();

    let mut cascades: [Vec<Card>; CASCADE_COUNT as usize] = Default::default();

    for (card, i) in deck.iter().enumerate().map(|(i, c)| (c, i / 7)) {
        match i > 4 {
            false => cascades[i].push(*card),
            true => cascades[i + 1].push(*card),
        }
    }
    Board::new(cascades, None)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{*};

    #[test]
    fn parse_card() {
        assert_eq!(Some(Card::new(2, Suit::Red)), board_helper::parse_card("2R").unwrap());
        assert_eq!(Some(Card::new(5, Suit::Red)), board_helper::parse_card("5R").unwrap());
        assert_eq!(Some(Card::new(10, Suit::Red)), board_helper::parse_card("10R").unwrap());
        assert_eq!(Some(Card::new(JACK_RANK, Suit::Red)), board_helper::parse_card("JR").unwrap());
        assert_eq!(Some(Card::new(QUEEN_RANK, Suit::Red)), board_helper::parse_card("QR").unwrap());
        assert_eq!(Some(Card::new(KING_RANK, Suit::Red)), board_helper::parse_card("KR").unwrap());

        assert_eq!(Some(Card::new(2, Suit::Green)), board_helper::parse_card("2G").unwrap());
        assert_eq!(Some(Card::new(5, Suit::Green)), board_helper::parse_card("5G").unwrap());
        assert_eq!(Some(Card::new(10, Suit::Green)), board_helper::parse_card("10G").unwrap());
        assert_eq!(Some(Card::new(JACK_RANK, Suit::Green)), board_helper::parse_card("JG").unwrap());
        assert_eq!(Some(Card::new(QUEEN_RANK, Suit::Green)), board_helper::parse_card("QG").unwrap());
        assert_eq!(Some(Card::new(KING_RANK, Suit::Green)), board_helper::parse_card("KG").unwrap());

        assert_eq!(Some(Card::new(2, Suit::Blue)), board_helper::parse_card("2B").unwrap());
        assert_eq!(Some(Card::new(5, Suit::Blue)), board_helper::parse_card("5B").unwrap());
        assert_eq!(Some(Card::new(10, Suit::Blue)), board_helper::parse_card("10B").unwrap());
        assert_eq!(Some(Card::new(JACK_RANK, Suit::Blue)), board_helper::parse_card("JB").unwrap());
        assert_eq!(Some(Card::new(QUEEN_RANK, Suit::Blue)), board_helper::parse_card("QB").unwrap());
        assert_eq!(Some(Card::new(KING_RANK, Suit::Blue)), board_helper::parse_card("KB").unwrap());

        assert_eq!(Some(Card::new(2, Suit::Yellow)), board_helper::parse_card("2Y").unwrap());
        assert_eq!(Some(Card::new(5, Suit::Yellow)), board_helper::parse_card("5Y").unwrap());
        assert_eq!(Some(Card::new(10, Suit::Yellow)), board_helper::parse_card("10Y").unwrap());
        assert_eq!(Some(Card::new(JACK_RANK, Suit::Yellow)), board_helper::parse_card("JY").unwrap());
        assert_eq!(Some(Card::new(QUEEN_RANK, Suit::Yellow)), board_helper::parse_card("QY").unwrap());
        assert_eq!(Some(Card::new(KING_RANK, Suit::Yellow)), board_helper::parse_card("KY").unwrap());

        assert!(matches!(board_helper::parse_card("AR"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("0R"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("1R"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("11R"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("21R"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("R"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("00R"), Err(ParseError::InvalidCard(_))));

        assert!(matches!(board_helper::parse_card("AG"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("0G"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("1G"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("11G"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("21G"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("G"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("00G"), Err(ParseError::InvalidCard(_))));

        assert!(matches!(board_helper::parse_card("22"), Err(ParseError::InvalidCard(_))));
        assert!(matches!(board_helper::parse_card("25"), Err(ParseError::InvalidCard(_))));

        for rank in MINOR_ARC_MIN_RANK..=MAJOR_ARC_MAX_RANK {
            assert_eq!(Some(Card::new(rank, Suit::MajorArc)), board_helper::parse_card(&rank.to_string()).unwrap());
        }
    }

    #[test]
    fn parse_board() {
        let cascade_string = "10	11	KR	6	QG	-	7Y	6Y	QR	JR	20
5	1	4	JG	5G	-	7B	2Y	15	5Y	7G
8Y	3R	5B	2G	18	-	6G	19	JB	4Y	21
9	KB	KG	3Y	KY	-	8R	9B	14	6B	2B
0	2R	5R	QY	2	-	4B	4G	10Y	6R	9R
8B	3	12	7R	7	-	13	9Y	10R	QB	17
8	10B	10G	4R	16	-	8G	JY	9G	3B	3G";
        let board = board_helper::parse_board(&cascade_string, None).unwrap();

        let card_count = board.cascades().iter().map(|cc| cc.len()).sum::<usize>();
        assert_eq!(card_count, 70);

        assert_eq!(board.cascades()[0][0], Card::new(10, Suit::MajorArc));
        assert_eq!(board.cascades()[1][0], Card::new(11, Suit::MajorArc));
        assert_eq!(board.cascades()[2][0], Card::new(KING_RANK, Suit::Red));
        assert_eq!(board.cascades()[3][0], Card::new(6, Suit::MajorArc));
        assert_eq!(board.cascades()[4][0], Card::new(QUEEN_RANK, Suit::Green));
        assert_eq!(board.cascades()[6][0], Card::new(7, Suit::Yellow));
        assert_eq!(board.cascades()[7][0], Card::new(6, Suit::Yellow));
        assert_eq!(board.cascades()[8][0], Card::new(QUEEN_RANK, Suit::Red));
        assert_eq!(board.cascades()[9][0], Card::new(JACK_RANK, Suit::Red));
        assert_eq!(board.cascades()[10][0], Card::new(20, Suit::MajorArc));

        assert_eq!(board.cascades()[0][6], Card::new(8, Suit::MajorArc));
        assert_eq!(board.cascades()[1][6], Card::new(10, Suit::Blue));
        assert_eq!(board.cascades()[2][6], Card::new(10, Suit::Green));
        assert_eq!(board.cascades()[3][6], Card::new(4, Suit::Red));
        assert_eq!(board.cascades()[4][6], Card::new(16, Suit::MajorArc));
        assert_eq!(board.cascades()[6][6], Card::new(8, Suit::Green));
        assert_eq!(board.cascades()[7][6], Card::new(JACK_RANK, Suit::Yellow));
        assert_eq!(board.cascades()[8][6], Card::new(9, Suit::Green));
        assert_eq!(board.cascades()[9][6], Card::new(3, Suit::Blue));
        assert_eq!(board.cascades()[10][6], Card::new(3, Suit::Green));        

        assert_eq!(board.cascades()[5].is_empty(), true);
    }

    #[test]
    pub fn create_random_board() {
        let mut solved = 0;
        let start = Instant::now();

        for seed in 0..10 {
            let board = board_helper::create_random_board(seed);

            let solver = Solver::new(&board);
            let result = solver.solve(100_000, 100, true);

            println!("{seed} => {:?}", result.status);

            if result.solved() {
                solved += 1;
            }
        }

        let elapsed = (Instant::now() - start).as_secs_f64();
        println!("{solved} solved. Took {elapsed:.3} sec");
    }

}