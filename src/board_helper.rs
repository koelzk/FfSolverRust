use std::collections::HashSet;

use crate::*;

pub fn parse_board(cascade_string: &str, cell_string: Option<&str>) -> Result<Board, ()> {

    let card_strings = cascade_string.split_ascii_whitespace();
    let mut cascades: [Vec<Card>; CASCADE_COUNT as usize] = Default::default();

    for (index, card_str) in card_strings.enumerate() {
        let i = index % CASCADE_COUNT as usize;
        let j = index / CASCADE_COUNT as usize;

        let card = parse_card(card_str)?;
        if let Some(c) = card {

            if cascades[i].len() != j {
                return Err(())
            }

            cascades[i].push(c);
        }
    }

    let cell = match cell_string {
        Some(cs) => parse_card(cs)?,
        None => None
    };

    let mut card_set = HashSet::new();
    let mut has_duplicates = cascades.iter().flat_map(|cc| cc).any(|&c| !card_set.insert(u8::from(&c)));

    if let Some(card) = cell {
        has_duplicates &= !card_set.contains(&u8::from(&card));
    }
    
    if has_duplicates {
        return Err(());
    }

    Ok(Board::new(cascades, cell))
}

fn parse_rank(r: &[u8], &suit: &Suit) -> Result<u8, ()> {
    match r {
        [b @ b'0'..=b'9'] => Ok(b - b'0'),
        [b'J'] if suit != Suit::MajorArc => Ok(JACK_RANK),
        [b'Q'] if suit != Suit::MajorArc => Ok(QUEEN_RANK),
        [b'K'] if suit != Suit::MajorArc => Ok(KING_RANK),
        [b'1', b'0'] if suit != Suit::MajorArc => Ok(10),
        [b1 @ b'1'..=b'2', b2 @ b'0'..=b'9'] if suit == Suit::MajorArc => Ok((b1 - b'0') * 10 + (b2 - b'0')),
        _ => Err(())
    }
}

fn parse_suit(seq: &[u8]) -> Result<Suit, ()> {
    match seq[seq.len() - 1] {
        b'0'..=b'9' => Ok(Suit::MajorArc),
        b'R' => Ok(Suit::Red),
        b'G' => Ok(Suit::Green),
        b'B' => Ok(Suit::Blue),
        b'Y' => Ok(Suit::Yellow),
        _ => Err(())
    }
}

pub fn parse_card(card_str: &str) -> Result<Option<Card>, ()> {
    if card_str.len() == 0 || card_str.len() > 3 {
        return Err(());
    }

    if card_str == "-" {
        return Ok(None);
    }

    let seq = card_str.as_bytes();
    let suit = parse_suit(seq)?;

    match suit {
        Suit::MajorArc => {
            match parse_rank(seq, &suit)? {
                rank @ MAJOR_ARC_MIN_RANK..=MAJOR_ARC_MAX_RANK => Ok(Some(Card::new(rank, Suit::MajorArc))),
                _ => Err(())
            }
        }
        _ => {
            match parse_rank(&seq[0..seq.len() - 1], &suit)? {
                rank @ MINOR_ARC_MIN_RANK..=MINOR_ARC_MAX_RANK => Ok(Some(Card::new(rank, suit))),
                _ => Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{*};


    #[test]
    fn parse_card() {
        assert_eq!(Ok(Some(Card::new(2, Suit::Red))), board_helper::parse_card("2R"));
        assert_eq!(Ok(Some(Card::new(5, Suit::Red))), board_helper::parse_card("5R"));
        assert_eq!(Ok(Some(Card::new(10, Suit::Red))), board_helper::parse_card("10R"));
        assert_eq!(Ok(Some(Card::new(JACK_RANK, Suit::Red))), board_helper::parse_card("JR"));
        assert_eq!(Ok(Some(Card::new(QUEEN_RANK, Suit::Red))), board_helper::parse_card("QR"));
        assert_eq!(Ok(Some(Card::new(KING_RANK, Suit::Red))), board_helper::parse_card("KR"));

        assert_eq!(Ok(Some(Card::new(2, Suit::Green))), board_helper::parse_card("2G"));
        assert_eq!(Ok(Some(Card::new(5, Suit::Green))), board_helper::parse_card("5G"));
        assert_eq!(Ok(Some(Card::new(10, Suit::Green))), board_helper::parse_card("10G"));
        assert_eq!(Ok(Some(Card::new(JACK_RANK, Suit::Green))), board_helper::parse_card("JG"));
        assert_eq!(Ok(Some(Card::new(QUEEN_RANK, Suit::Green))), board_helper::parse_card("QG"));
        assert_eq!(Ok(Some(Card::new(KING_RANK, Suit::Green))), board_helper::parse_card("KG"));

        assert_eq!(Ok(Some(Card::new(2, Suit::Blue))), board_helper::parse_card("2B"));
        assert_eq!(Ok(Some(Card::new(5, Suit::Blue))), board_helper::parse_card("5B"));
        assert_eq!(Ok(Some(Card::new(10, Suit::Blue))), board_helper::parse_card("10B"));
        assert_eq!(Ok(Some(Card::new(JACK_RANK, Suit::Blue))), board_helper::parse_card("JB"));
        assert_eq!(Ok(Some(Card::new(QUEEN_RANK, Suit::Blue))), board_helper::parse_card("QB"));
        assert_eq!(Ok(Some(Card::new(KING_RANK, Suit::Blue))), board_helper::parse_card("KB"));

        assert_eq!(Ok(Some(Card::new(2, Suit::Yellow))), board_helper::parse_card("2Y"));
        assert_eq!(Ok(Some(Card::new(5, Suit::Yellow))), board_helper::parse_card("5Y"));
        assert_eq!(Ok(Some(Card::new(10, Suit::Yellow))), board_helper::parse_card("10Y"));
        assert_eq!(Ok(Some(Card::new(JACK_RANK, Suit::Yellow))), board_helper::parse_card("JY"));
        assert_eq!(Ok(Some(Card::new(QUEEN_RANK, Suit::Yellow))), board_helper::parse_card("QY"));
        assert_eq!(Ok(Some(Card::new(KING_RANK, Suit::Yellow))), board_helper::parse_card("KY"));

        assert_eq!(Err(()), board_helper::parse_card("AR"));
        assert_eq!(Err(()), board_helper::parse_card("0R"));
        assert_eq!(Err(()), board_helper::parse_card("1R"));
        assert_eq!(Err(()), board_helper::parse_card("11R"));
        assert_eq!(Err(()), board_helper::parse_card("21R"));
        assert_eq!(Err(()), board_helper::parse_card("R"));
        assert_eq!(Err(()), board_helper::parse_card("00R"));

        assert_eq!(Err(()), board_helper::parse_card("AG"));
        assert_eq!(Err(()), board_helper::parse_card("0G"));
        assert_eq!(Err(()), board_helper::parse_card("1G"));
        assert_eq!(Err(()), board_helper::parse_card("11G"));
        assert_eq!(Err(()), board_helper::parse_card("21G"));
        assert_eq!(Err(()), board_helper::parse_card("G"));
        assert_eq!(Err(()), board_helper::parse_card("00G"));

        assert_eq!(Err(()), board_helper::parse_card("22"));
        assert_eq!(Err(()), board_helper::parse_card("25"));

        for rank in MINOR_ARC_MIN_RANK..=MAJOR_ARC_MAX_RANK {
            assert_eq!(Ok(Some(Card::new(rank, Suit::MajorArc))), board_helper::parse_card(&rank.to_string()));
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

}