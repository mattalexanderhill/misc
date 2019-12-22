#![allow(dead_code)]

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

    fn get_category(&self) -> Category {
        if self.is_full_house() { return Category::FullHouse }
        if self.is_straight_flush() { return Category::StraightFlush }
        if self.is_x_of_a_kind(4) { return Category::FourOfAKind }
        if self.is_full_house() { return Category::FourOfAKind }
        if self.is_flush() { return Category::Flush }
        if self.is_straight() { return Category::Straight }
        if self.is_x_of_a_kind(3) { return Category::ThreeOfAKind }
        if self.is_two_pair() { return  Category::TwoPairs }
        if self.is_x_of_a_kind(2) { return  Category::OnePair }
        Category::HighCard
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

    fn is_x_of_a_kind(&self, x: u8) -> bool {
        for i in 0..5 {
            let mut counter = 0;

            for j in i..5 {
                if &self[j].rank == &self[i].rank {
                    counter += 1;
                }
                if counter >= x {
                    return true;
                }
            }
        }
        false
    }

    fn is_straight(&self) -> bool {
        let mut lowest = &self.one.rank;
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

    fn is_flush(&self) -> bool {
        for i in 0..5 {
            if &self[i].suit != &self[0].suit {
                return false;
            }
        }
        true
    }

    fn is_two_pair(&self) -> bool {
        match self.rank_counts().as_slice() {
            [1, 2, 2] => true,
            [2, 1, 2] => true,
            [2, 2, 1] => true,
            _ => false
        }
    }

    fn is_straight_flush(&self) -> bool {
        self.is_flush() && self.is_straight()
    }

    fn is_royal_flush(&self) -> bool {
        self.contains_rank(&Rank::Ace) && self.is_straight_flush()
    }

    fn full_house(&self) -> bool {
        match self.rank_counts().as_slice() {
            [3, 2] => true,
            [2, 3] => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn test_is_x_of_a_kind() {
        let hand = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::Two,   suit: Suit::Hearts},
            two:   Card{rank: Rank::Three, suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::Two,   suit: Suit::Hearts},
        };

        assert!(hand.is_x_of_a_kind(3));
        assert!(!hand.is_x_of_a_kind(4));
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
            zero:  Card{rank: Rank::One,  suit: Suit::Hearts},
            one:   Card{rank: Rank::Two,  suit: Suit::Hearts},
            two:   Card{rank: Rank::Three, suit: Suit::Hearts},
            three: Card{rank: Rank::Four, suit: Suit::Hearts},
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
    fn test_is_two_pair() {
        let a = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::Two,   suit: Suit::Hearts},
            two:   Card{rank: Rank::Three,   suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::One, suit: Suit::Hearts},
        };

        assert!(a.is_two_pair());

        let b = Hand {
            zero:  Card{rank: Rank::One,   suit: Suit::Hearts},
            one:   Card{rank: Rank::One,   suit: Suit::Hearts},
            two:   Card{rank: Rank::One,   suit: Suit::Hearts},
            three: Card{rank: Rank::Two,   suit: Suit::Hearts},
            four:  Card{rank: Rank::Three, suit: Suit::Hearts},
        };

        assert!(!b.is_two_pair());
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
}
