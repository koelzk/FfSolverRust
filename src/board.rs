use std::{collections::HashMap, fmt::Display};

use crate::*;

use self::card_move::{CardMove, CELL, FOUNDATION};

use rand::Rng;
use rand::seq::SliceRandom;
use itertools::Itertools;

pub const CASCADE_COUNT: u8 = 11;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Board {
    cell: Option<Card>,
    cascades: [Vec<Card>; CASCADE_COUNT as usize],
    major_fdn_low: i8,
    major_fdn_high: i8,
    minor_fdns: [u8; 4]
}

impl Board {
    pub fn new(cascades: [Vec<Card>; CASCADE_COUNT as usize], cell: Option<Card>) -> Board {
        let mut board = Board {
            cell,
            cascades,
            major_fdn_low: (MAJOR_ARC_MIN_RANK as i8) - 1,
            major_fdn_high: (MAJOR_ARC_MAX_RANK as i8) + 1,
            minor_fdns: [ACE_RANK; 4]
        };

        board.update_foundations();

        return board;
    }

    pub fn is_game_won(&self) -> bool {
        self.major_fdn_low == self.major_fdn_high && self.minor_fdns.iter().all(|&x| x == KING_RANK)
    }

    pub fn cascades(&self) -> &[Vec<Card>; CASCADE_COUNT as usize] {
        &self.cascades
    }

    pub fn cell(&self) -> Option<Card> {
        self.cell
    }

    pub fn minor_fdns(&self) -> impl Iterator<Item = Option<Card>> + '_ {
        self.minor_fdns.iter()
            .enumerate()
            .map(|(index, &rank)| match rank > ACE_RANK {
                true => Some(Card::new(rank.clone(), Suit::from(index as u8))),
                false => None
            })
    }

    pub fn major_fdn_low(&self) -> Option<Card> {
        match self.major_fdn_low >= MAJOR_ARC_MIN_RANK as i8 {
            true => Some(Card::new(self.major_fdn_low as u8, Suit::MajorArc)),
            false => None
        }
    }

    pub fn major_fdn_high(&self) -> Option<Card> {
        match self.major_fdn_high <= MAJOR_ARC_MAX_RANK as i8 {
            true => Some(Card::new(self.major_fdn_high as u8, Suit::MajorArc)),
            false => None
        }
    }

    pub fn apply_auto_moves(&mut self) {
        loop
        {
            let mut moves = Vec::<CardMove>::new();
            self.enumerate_auto_moves(&mut moves);

            if moves.len() > 0 {
                for card_move in moves {
                    self.apply_move(&card_move);
                }
            }
            else {
                break;
            }
        }        
    }

    pub fn normalize_order(&mut self)
    {
        self.cascades.sort_by_key(|cc| BoardNormalization::cascade_rank(cc));

        if self.major_fdn_low == self.major_fdn_high {
            self.major_fdn_low = 21;
            self.major_fdn_high = 21;
        }
    }

    pub fn normalize(&mut self)    
    {
        self.apply_auto_moves();
        self.normalize_order();
    }    

    pub fn enumerate_auto_moves(&self, moves: &mut Vec<CardMove>) {

        let cascade_moves = self.cascades.iter()
            .enumerate()
            .filter(|(_, cascade)| cascade.len() > 0)
            .filter_map(|(i, cascade)| {
                match self.can_remove_card(cascade.last().unwrap()) {
                    true => Some(CardMove::new(i as u8, FOUNDATION, 1)),
                    false => None
                }
            });

        moves.extend(cascade_moves);

        if matches!(self.cell, Some(card) if self.can_remove_card(&card)) {
            moves.push(CardMove::new(CELL, FOUNDATION, 1));
        }
    }

    pub fn enumerate_moves(&self, moves: &mut Vec<CardMove>) {
        // Move k cards from cascade i to cascade j:
        let from_cascades = self.cascades.iter()
            .enumerate()
            .filter(|(_, cascade)| cascade.len() > 0);

        for (i, from) in from_cascades {
            let stack_size = Board::get_stack_size(from);

            let to_cascades = self.cascades.iter()
                .enumerate()
                .filter(|&(j, _)| i != j);

            for (j, to) in to_cascades {
                if to.len() == 0 || from.last().unwrap().can_place_on(to.last().unwrap()) {
                    for k in (1..=stack_size).rev() {
                        moves.push(CardMove::new(i as u8, j as u8, k))
                    }
                }
            }
        }

        match self.cell {
            Some(card) => {
                // Move 1 card from cell to cascade j:
                for (j, to) in self.cascades.iter().enumerate() {
                    if to.len() == 0 || card.can_place_on(to.last().unwrap()) {
                        moves.push(CardMove::new(CELL, j as u8, 1))
                    }
                }
            } 
            None => {
                // Move 1 card from cascade to cell:
                for (i, from) in self.cascades.iter().enumerate() {
                    if from.len() > 0 {
                        moves.push(CardMove::new(i as u8, CELL, 1))
                    }
                }            
            },
        }
    }

    fn get_cascades(&mut self, from: u8, to: u8) -> (&mut Vec<Card>, &mut Vec<Card>) {
        if from > to {
            let t = self.get_cascades(to, from);
            return (t.1, t.0);
        }

        let (left, right) = self.cascades.split_at_mut(to as usize);
        (&mut left[from as usize], &mut right[0])
    }

    fn update_foundation(&mut self, removed: &Card) {
        if removed.suit() == Suit::MajorArc {
            //assert!(removed.rank() == self.major_fdn_low + 1 || removed.rank() == self.major_fdn_high - 1);
            if removed.rank() as i8 == self.major_fdn_low + 1 {
                self.major_fdn_low += 1;
            }
            else if removed.rank() as i8 == self.major_fdn_high - 1 {
                self.major_fdn_high -= 1;
            }
        }
        else {
            let suit_index = removed.suit() as usize;
            assert!(removed.rank() == self.minor_fdns[suit_index] + 1);
            self.minor_fdns[suit_index] += 1;
        }
    }

    fn update_foundations(&mut self) {
        let cell_card: Card;
        let mut all_cards: Vec<&Card> = self.cascades.iter()
            .flat_map(|cc| cc)
            .collect();

        if self.cell.is_some() {
            cell_card = self.cell.unwrap();
            all_cards.push(&cell_card);
        }

        all_cards.sort_by_key(|&c| c.suit() as u8);

        let low_fdns = all_cards.iter()
            .group_by(|&c| c.suit())
            .into_iter()
            .map(|(key, group)| {
                (key as usize, group.map(|c| c.rank() as u8).min().unwrap())
            })
            .collect::<HashMap<usize, u8>>();

        for i in 0..4 {
            self.minor_fdns[i] = match low_fdns.get(&i) {
                Some(&f) => f - 1,
                None => KING_RANK,
            }
        }

        match low_fdns.get(&(Suit::MajorArc as usize)) {
            Some(&f) => {
                self.major_fdn_low = f as i8 - 1;
                self.major_fdn_high = all_cards.iter()
                    .filter(|c| c.suit() == Suit::MajorArc)
                    .map(|c| c.rank())
                    .max()
                    .unwrap() as i8 + 1;
            },
            None => {
                self.major_fdn_low = 21;
                self.major_fdn_high = 21;
            },
        }
    }

    pub fn apply_move(&mut self, m: &CardMove) {
        if m.from() < CASCADE_COUNT && m.to() < CASCADE_COUNT { // Move from cascade to cascade:
            assert!(m.from() < CASCADE_COUNT);
            assert!(m.to() < CASCADE_COUNT && m.to() != m.from());
    
            let (from, to) = self.get_cascades(m.from(), m.to());
            assert!(from.len() >= m.count() as usize);
    
            for _ in 0..m.count() {
                to.push(from.pop().unwrap())
            }

            return;
        }
        else if m.to() == FOUNDATION { // Move card to foundation:
            if m.from() == CELL {
                self.update_foundation(&self.cell.unwrap());
                self.cell = None;
            }
            else {
                assert!(m.from() < CASCADE_COUNT);
                let card = self.cascades[m.from() as usize].pop().unwrap();
                self.update_foundation(&card);
            }

            return;
        }
        else if m.from() == CELL { // Move from cell to cascade:
            assert!(self.cell.is_some() && m.to() < CASCADE_COUNT);

            let to: &mut Vec<Card> = &mut self.cascades[m.to() as usize];
            to.push(self.cell.unwrap());            
            self.cell = None;

            return;
        }
        else { // Move from cascade to cell:
            assert!(m.from() < CASCADE_COUNT && self.cell.is_none());

            let card = self.cascades[m.from() as usize].pop().unwrap();
            self.cell = Some(card);

            return;
        }


    }

    pub fn score(&self, step: u32) -> i32 {
        let mut score = 0i32;

        score -= self.cascades.iter()
            .map(|cascade| cascade.len() as i32)
            .sum::<i32>();

        score += self.cascades.iter().map(|cascade| {
            let stack_size = Board::get_stack_size(cascade) as i32;
            if cascade.len() == 0 {
                return 20;
            }
            else if cascade.len() as i32 == stack_size {
                return stack_size * 2;
            }
            else {
                return stack_size;
            }
        }).sum::<i32>();

        score -= match self.cell {
            Some(_) => 10,
            None => 0
        };

        score -= step as i32;

        return score;
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let mut deck = Card::create_deck().collect::<Vec<Card>>();
        deck.shuffle(rng);

        let mut cascades: [Vec<Card>; CASCADE_COUNT as usize] = Default::default();

        for (card, i) in deck.iter().enumerate().map(|(i, c)| (c, i / 7)) {
            match i > 4 {
                false => cascades[i].push(*card),
                true => cascades[i + 1].push(*card),
            }            
        }

        Board {
            cascades,
            cell: None,
            major_fdn_low: (MAJOR_ARC_MIN_RANK as i8) - 1,
            major_fdn_high: (MAJOR_ARC_MAX_RANK as i8) + 1,
            minor_fdns: [ACE_RANK; 4]
        }
    }

    fn  can_remove_card(&self, card: &Card) -> bool {
        if card.suit() == Suit::MajorArc {
            let r = card.rank() as i8;
            return (r == self.major_fdn_low + 1) || (r == self.major_fdn_high - 1);
        }

        if self.cell.is_some() { // Cell must be empty to remove color cards
            return false;
        }

        let fdn = self.minor_fdns[card.suit() as usize];
        return card.rank() == fdn + 1;
    }

    fn get_stack_size(cascade: &Vec<Card>) -> u8 {
        if cascade.len() < 2 {
            return cascade.len() as u8;
        }

        let mut card: &Card;
        let mut previous = cascade.last().unwrap();
        let mut stack_size = 1;
        for i in (0..=cascade.len() - 2).rev() {
            card = &cascade[i];
            if !previous.can_place_on(card) {
                break;
            }

            previous = &card;
            stack_size += 1;
        }

        return stack_size;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        fn card_str(card: &Option<Card>) -> String {
            match card {
                Some(c) => format!("{:<4}", c.to_string()),
                None => "-   ".to_owned()
            }
        }

        f.write_str(&card_str(&self.major_fdn_low()))?;
        f.write_str(" ")?;
        f.write_str(&card_str(&self.major_fdn_high()))?;
        f.write_str("     ")?;
        f.write_str(&card_str(&self.cell()))?;
        f.write_str("      ")?;
        for card in self.minor_fdns() {
            f.write_str(&card_str(&card))?;
        }

        let max_count = self.cascades.iter().map(|cc| cc.len()).max().unwrap();

        for row in 0..max_count {
            f.write_str("\n")?;
            let row_cards = self.cascades.iter().map(|cc| match row < cc.len() {
                true => Some(cc[row]),
                false => None
            });

            for card in row_cards {
                match card {
                    Some(_) => f.write_str(&card_str(&card))?,
                    None => f.write_str("    ")?
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use super::Board;

    #[test]
    fn random() {
        let board = Board::random(&mut rand::thread_rng());

        print!("{board}");
    }


    #[test]
    fn get_stack_size() {
        let cascade = vec![
            Card::new(2, Suit::Yellow),
            Card::new(3, Suit::Red),
            Card::new(4, Suit::Red),
            Card::new(5, Suit::Red)];
        
        assert_eq!(3, Board::get_stack_size(&cascade));
        assert_eq!(0, Board::get_stack_size(&Vec::<Card>::new()));
        assert_eq!(1, Board::get_stack_size(&vec![Card::new(2, Suit::Yellow)]));
        assert_eq!(2, Board::get_stack_size(&vec![Card::new(2, Suit::Yellow), Card::new(3, Suit::Yellow)]));
        assert_eq!(2, Board::get_stack_size(&vec![Card::new(3, Suit::Yellow), Card::new(2, Suit::Yellow)]));
    }
}