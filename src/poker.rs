#![allow(dead_code)]

use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(PartialOrd, PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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

type Hand = [Card; 5];

#[derive(PartialOrd, Debug)]
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

impl Hand {
    fn get_score

    fn is_x_of_a_kind(x: u8, hand: &Hand) -> Some(Rank) {
        let counter = 1;
        let rank: Some(Rank) = None;

        for card in hand.iter() {
            if rank.contains(&card.rank) {
                counter += 1;
            }
            if counter >= x {
                return rank;
            }
            rank = Some(card.rank);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn test_x_of_a_kind() {
        let Hand = [
            Card{rank: Rank::One, suit: Suit::Hearts},
            Card{rank: Rank::Two, suit: Suit::Hearts},
            Card{rank: Rank::Three, suit: Suit::Hearts},
            Card{rank: Rank::Two, suit: Suit::Hearts},
            Card{rank: Rank::One, suit: Suit::Hearts},
        ];

        assert(is_x_of_a_kind(2, &Hand));
        assert(!is_x_of_a_kind(3, &Hand));
    }
}   
