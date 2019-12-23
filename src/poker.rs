#![allow(dead_code)]

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp::Ordering;
use std::ops::Index;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Clone, Copy)]
enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn next(&self) -> Option<Rank> {
        match self {
            Rank::One   => Some(Rank::Two),
            Rank::Two   => Some(Rank::Three),
            Rank::Three => Some(Rank::Four),
            Rank::Four  => Some(Rank::Five),
            Rank::Five  => Some(Rank::Six),
            Rank::Six   => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Nine),
            Rank::Nine  => Some(Rank::Ten),
            Rank::Ten   => Some(Rank::Jack),
            Rank::Jack  => Some(Rank::Queen),
            Rank::Queen => Some(Rank::King),
            Rank::King  => Some(Rank::Ace),
            Rank::Ace   => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Card {
    fn from_code(code: &str) -> Option<Self> {
        let mut chars = code.chars();

        let rank = match chars.next() {
            Some('1') => Rank::One,
            Some('2') => Rank::Two,
            Some('3') => Rank::Three,
            Some('4') => Rank::Four,
            Some('5') => Rank::Five,
            Some('6') => Rank::Six,
            Some('7') => Rank::Seven,
            Some('8') => Rank::Eight,
            Some('9') => Rank::Nine,
            Some('T') => Rank::Ten,
            Some('J') => Rank::Jack,
            Some('Q') => Rank::Queen,
            Some('K') => Rank::King,
            Some('A') => Rank::Ace,
            _ => return None,
        };

        let suit = match chars.next() {
            Some('H') => Suit::Hearts,
            Some('D') => Suit::Diamonds,
            Some('C') => Suit::Clubs,
            Some('S') => Suit::Spades,
            _ => return None,
        };

        return Some(Card{rank, suit});
    }
}

struct Hand {
    zero:  Card,
    one:   Card,
    two:   Card,
    three: Card,
    four:  Card,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Category {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl Index<u8> for Hand {
    type Output = Card;

    fn index(&self, i: u8) -> &Self::Output {
        match i {
            0 => &self.zero,
            1 => &self.one,
            2 => &self.two,
            3 => &self.three,
            4 => &self.four,
            _ => panic!("Hand Index Error!")
        }
    }
}

impl Hand {

    fn from_str(s: &str) -> Option<Self> {
        // Format RS RS RS RS RS 
        // where R is one of [1-10JKQA]
        //   and S is one of [CDHS]
        let mut n = 0;
        let mut cards_str = String::with_capacity(2);

        let mut cards: [Option<Card>; 5] = [None; 5];

        for c in s.chars() {
            if n > 4 { return None; }
            if c.is_whitespace() { continue; }

            cards_str.push(c);

            if cards_str.len() == 2 {
                cards[n] = Card::from_code(&cards_str);
                if cards[n].is_none() { return None; }
                n += 1;
                cards_str.clear();
            }
        }
        Some(Hand::from_cards(cards))
    }

    fn from_cards(cards: [Option<Card>; 5]) -> Self {
        Hand {
            zero: cards[0].unwrap(),
            one: cards[1].unwrap(),
            two: cards[2].unwrap(),
            three: cards[3].unwrap(),
            four: cards[4].unwrap(),
        }
    }

    fn high_rank(&self) -> Rank {
        let mut highest: Rank = self[0].rank;

        for i in 1..5 {
            if self[i].rank > highest {
                highest = self[i].rank;
            }
        }

        highest
    }

    fn cmp(&self, other: Self) -> Ordering {
      let (score, rank) = self.score();
      let (score_other, rank_other) = other.score();

      if score > score_other { return Ordering::Greater; } 
      if score < score_other { return Ordering::Less; } 

      if rank > rank_other { return Ordering::Greater; } 
      if rank < rank_other { return Ordering::Less; } 

      let mut ranks = self.ranks();
      let mut other_ranks = other.ranks();
      ranks.sort();
      other_ranks.sort();

      for i in (0..5).rev() {
        if ranks[i] > other_ranks[i] { return Ordering::Greater}
        if ranks[i] < other_ranks[i] { return Ordering::Less}
      }

      Ordering::Equal
    } 

    fn score(&self) -> (Category, Rank) {
        if self.is_royal_flush() {
            return (Category::RoyalFlush, Rank::Ace);
        }
        if let Some(r) = self.straight_flush() {
            return (Category::StraightFlush, r);
        }
        if let Some(r) = self.x_of_a_kind(4) {
            return (Category::FourOfAKind, r)
        }
        if let Some(r) = self.full_house() {
            return (Category::FullHouse, r);
        }
        if let Some(r) = self.flush() {
            return (Category::Flush, r);
        }
        if let Some(r) = self.straight() {
            return (Category::Straight, r);
        }
        if let Some(r) = self.x_of_a_kind(3) {
            return (Category::ThreeOfAKind, r);
        }
        if let Some(r) = self.two_pair() {
            return (Category::TwoPairs, r);
        }
        if let Some(r) = self.x_of_a_kind(2) {
            return (Category::OnePair, r);
        }

        return (Category::HighCard, self.high_rank());
    }

    fn ranks(&self) -> Vec<&Rank>{
        vec![
            &self.zero.rank,
            &self.one.rank,
            &self.two.rank,
            &self.three.rank,
            &self.four.rank,
        ]
    }

    fn contains_rank(&self, other: &Rank) -> bool {
        if &self.zero.rank == other { return true}
        if &self.one.rank == other { return true}
        if &self.two.rank == other { return true}
        if &self.three.rank == other { return true}
        if &self.four.rank == other { return true}
        false
    }

    fn rank_counts(&self) -> Vec<u8> {
        let mut ranks = self.ranks();
        ranks.sort();

        let mut last = ranks[0];
        let mut counts: Vec<u8> = vec![];
        let mut counter:  u8 = 1;

        for i in 1..5 {
            if ranks[i] == last {
                counter += 1;
            } else {
                counts.push(counter);
                counter = 1;
            }
            last = ranks[i];
        }
        counts.push(counter);
        counts
    }

    fn x_of_a_kind(&self, x: u8) -> Option<Rank> {
        let mut rank: Option<Rank> = None;

        for i in 0..5 {
            let mut counter = 0;

            for j in i..5 {
                if &self[j].rank == &self[i].rank {
                    counter += 1;
                    if counter >= x {
                        match rank {
                            None => {
                                rank = Some(self[i].rank.clone());
                            },
                            Some(r) if r < self[i].rank => {
                                rank = Some(self[i].rank.clone());
                            },
                            _ => (),
                        }
                    }
                }
            }
        }
        rank
    }

    fn is_x_of_a_kind(&self, x: u8) -> bool {
        self.x_of_a_kind(x).is_some()
    }

    fn is_straight(&self) -> bool{
        let mut lowest = &self[0].rank;
        for i in 1..5 {
            if &self[i].rank < lowest {
                lowest = &self[i].rank;
            }
        }

        let mut required = lowest.clone();
        for _ in 1..5 {
            match required.next() {
                Some(r) => {
                    if !self.contains_rank(&r) {
                        return false;
                    }
                    required = r.clone();
                },
                None => return false
            }
        }
        true
    }

    fn straight(&self) -> Option<Rank> {
        if self.is_straight() {
            Some(self.high_rank())
        } else {
            None
        }
    }

    fn flush(&self) -> Option<Rank> {
        for i in 0..5 {
            if &self[i].suit != &self[0].suit {
                return None;
            }
        }
        Some(self.high_rank())
    }

    fn is_flush(&self) -> bool {
        self.flush().is_some()
    }

    fn two_pair(&self) -> Option<Rank> {
        match self.rank_counts().as_slice() {
            [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => {
                self.x_of_a_kind(2)
            },
            _ => None
        }
    }

    fn is_two_pair(&self) -> bool {
        self.two_pair().is_some()
    }

    fn straight_flush(&self) -> Option<Rank> {
        if self.is_flush() && self.is_straight() {
            Some(self.high_rank())
        } else {
            None
        }
    }

    fn is_straight_flush(&self) -> bool {
        self.straight_flush().is_some()
    }

    fn is_royal_flush(&self) -> bool {
        match self.straight_flush() {
            Some(Rank::Ace) => true,
            _ => false
        }
    }

    fn full_house(&self) -> Option<Rank> {
        match self.rank_counts().as_slice() {
            [3, 2] | [2, 3] => self.x_of_a_kind(3),
            _ => None
        }
    }

    fn is_full_house(&self) -> bool {
        self.full_house().is_some()
    }
}

fn problem() -> std::io::Result<(u32, u32, u32)> {
    let f = File::open("resources/poker.txt")?;
    let reader = BufReader::new(f);

    let mut wins_one = 0;
    let mut wins_two = 0;
    let mut draws = 0;

    for line in reader.lines() {
      let line = line.unwrap();
      let (one, two) = line.split_at(14);
      let hand_one = Hand::from_str(&one).unwrap();
      let hand_two = Hand::from_str(&two).unwrap();

      match hand_one.cmp(hand_two) {
        Ordering::Greater => wins_one += 1,
        Ordering::Less    => wins_two += 1,
        Ordering::Equal   => draws += 1,
      }
    }

    Ok((wins_one, wins_two, draws))
}

#[cfg(test)]
mod poker_tests {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(Rank::One.next(), Some(Rank::Two));
        assert_eq!(Rank::Ace.next(), None);
    }

    #[test]
    fn test_card_from_code() {
        assert_eq!(
            Card::from_code("JH").unwrap(),
            Card{rank: Rank::Jack, suit: Suit::Hearts}
        );
    }

    #[test]
    fn test_best_card() {
        assert!(
            Card{rank: Rank::Queen, suit: Suit::Hearts} >
            Card{rank: Rank::Jack, suit: Suit::Hearts}
        );
    }

    #[test]
    fn test_hand_from_str() {
        let hand = Hand::from_str("1H 2C 3S 2H 2C").unwrap();

        assert_eq!(hand.zero,  Card{rank: Rank::One,   suit: Suit::Hearts});
        assert_eq!(hand.one,   Card{rank: Rank::Two,   suit: Suit::Clubs});
        assert_eq!(hand.two,   Card{rank: Rank::Three, suit: Suit::Spades});
        assert_eq!(hand.three, Card{rank: Rank::Two,   suit: Suit::Hearts});
        assert_eq!(hand.four,  Card{rank: Rank::Two,   suit: Suit::Clubs});
    }

    #[test]
    fn test_x_of_a_kind() {
        let hand = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::Two,   suit: Suit::Hearts},
            two:   Card{rank: Rank::Three, suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::Two,   suit: Suit::Hearts},
        };

        assert_eq!(hand.is_x_of_a_kind(3), true);
        assert_eq!(hand.x_of_a_kind(3), Some(Rank::Two));
        assert_eq!(hand.is_x_of_a_kind(4), false);
        assert_eq!(hand.x_of_a_kind(4), None);
    }

    #[test]
    fn test_is_straight() {
        let a = Hand {
            zero:  Card{rank: Rank::Four,  suit: Suit::Hearts},
            one:   Card{rank: Rank::Five,  suit: Suit::Hearts},
            two:   Card{rank: Rank::Seven, suit: Suit::Hearts},
            three: Card{rank: Rank::Three, suit: Suit::Hearts},
            four:  Card{rank: Rank::Six,   suit: Suit::Hearts},
        };

        assert!(a.is_straight());

        let b = Hand {
            zero:  Card{rank: Rank::Four,  suit: Suit::Hearts},
            one:   Card{rank: Rank::Nine,  suit: Suit::Hearts},
            two:   Card{rank: Rank::Seven, suit: Suit::Hearts},
            three: Card{rank: Rank::Three, suit: Suit::Hearts},
            four:  Card{rank: Rank::Six,   suit: Suit::Hearts},
        };

        assert!(!b.is_straight());
    }

    #[test]
    fn test_is_flush() {
        let a = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::Two,   suit: Suit::Hearts},
            two:   Card{rank: Rank::Three, suit: Suit::Hearts},
            three: Card{rank: Rank::Three, suit: Suit::Hearts},
            four:  Card{rank: Rank::Six,   suit: Suit::Hearts},
        };

        assert!(a.is_flush());

        let b = Hand {
            zero:  Card{rank: Rank::Four,  suit: Suit::Hearts},
            one:   Card{rank: Rank::Nine,  suit: Suit::Hearts},
            two:   Card{rank: Rank::Seven, suit: Suit::Hearts},
            three: Card{rank: Rank::Three, suit: Suit::Hearts},
            four:  Card{rank: Rank::Six,   suit: Suit::Clubs},
        };

        assert!(!b.is_flush());
    }

    #[test]
    fn test_rank_counts() {
        let a = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::One,   suit: Suit::Hearts},
            two:   Card{rank: Rank::Two,   suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::Three, suit: Suit::Hearts},
        };

        assert_eq!(a.rank_counts(), vec![2, 2, 1]);

        let b = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::Two,   suit: Suit::Hearts},
            two:   Card{rank: Rank::Three, suit: Suit::Hearts},
            three: Card{rank: Rank::Four,  suit: Suit::Hearts},
            four:  Card{rank: Rank::One,   suit: Suit::Clubs},
        };

        assert_eq!(b.rank_counts(), [2, 1, 1, 1]);

        let c = Hand {
            zero:  Card{rank: Rank::One,  suit: Suit::Hearts},
            one:   Card{rank: Rank::One,  suit: Suit::Hearts},
            two:   Card{rank: Rank::One, suit: Suit::Hearts},
            three: Card{rank: Rank::One, suit: Suit::Hearts},
            four:  Card{rank: Rank::One,   suit: Suit::Clubs},
        };

        assert_eq!(c.rank_counts(), [5]);
    }

    #[test]
    fn test_two_pair() {
        let a = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::Two,   suit: Suit::Hearts},
            two:   Card{rank: Rank::Three, suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::One,   suit: Suit::Hearts},
        };

        assert_eq!(a.two_pair(), Some(Rank::Two));

        let b = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::One,   suit: Suit::Hearts},
            two:   Card{rank: Rank::One,   suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::Three, suit: Suit::Hearts},
        };

        assert_eq!(b.two_pair(), None);
    }

    #[test]
    fn test_is_full_house() {
        let a = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::One,   suit: Suit::Hearts},
            two:   Card{rank: Rank::One,   suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::Two, suit: Suit::Hearts},
        };

        assert!(a.is_full_house());

        let b = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::One,   suit: Suit::Hearts},
            two:   Card{rank: Rank::One,   suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::Three, suit: Suit::Hearts},
        };

        assert!(!b.is_full_house());
    }

    #[test]
    fn test_score() {
        let a = Hand::from_str("8C 8S KC 9H 9S").unwrap();
        let b = Hand::from_str("7D 2S 5D 3S AC").unwrap();
        let c = Hand::from_str("5C AC 5C AC 9C").unwrap();

        assert_eq!(a.score(), (Category::TwoPairs, Rank::Nine));
        assert_eq!(b.score(), (Category::HighCard, Rank::Ace));
        assert_eq!(c.score(), (Category::Flush, Rank::Ace));
    }

    #[test]
    fn test_cmp() {
        let a = Hand::from_str("5H 5C 6S 7S KD").unwrap();
        let b = Hand::from_str("2C 3S 8S 8D TD").unwrap();

        assert_eq!(a.cmp(b), Ordering::Less);

        let a = Hand::from_str("5D 8C 9S JS AC").unwrap();
        let b = Hand::from_str("2C 5C 7D 8S QH").unwrap();

        assert_eq!(a.cmp(b), Ordering::Greater);

        let a = Hand::from_str("2D 9C AS AH AC").unwrap();
        let b = Hand::from_str("3D 6D 7D TD QD").unwrap();

        assert_eq!(a.cmp(b), Ordering::Less);

        let a = Hand::from_str("4D 6S 9H QH QC").unwrap();
        let b = Hand::from_str("3D 6D 7H QD QS").unwrap();

        assert_eq!(a.cmp(b), Ordering::Greater);

        let a = Hand::from_str("2H 2D 4C 4D 4S").unwrap();
        let b = Hand::from_str("3C 3D 3S 9S 9D").unwrap();

        assert_eq!(a.cmp(b), Ordering::Greater);

        let a = Hand::from_str("6D 7C 5D 5H 3S").unwrap();
        let b = Hand::from_str("5C JC 2H 5S 3D").unwrap();

        assert_eq!(a.cmp(b), Ordering::Less);
    }

    #[test]
    fn test_problem() {
      let (wins_one, wins_two, draws) = problem().unwrap();

      assert_eq!(wins_one, 376);
      assert_eq!(wins_two, 624);
      assert_eq!(draws,    0);
    }
}
