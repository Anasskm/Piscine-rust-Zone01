use rand::Rng;
#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut random = rand::thread_rng();
        let i: i32 = random.gen_range(1..5);
        match i {
            1 => Suit::Club,
            2 => Suit::Heart,
            3 => Suit::Spade,
            _ => Suit::Diamond,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("invalid suit!!"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut random = rand::thread_rng();
        let i: i32 = random.gen_range(1..14);
        match i {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            i => Rank::Number(i as u8),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            value if value >= 2 && value < 11 => Rank::Number(value),
            _ => panic!("invalid Rank!!"),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card
        == (&Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        })
    {
        return true;
    }
    false
}