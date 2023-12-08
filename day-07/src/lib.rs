use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{self, BufRead},
    iter::zip,
    path::Path,
};

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn get_value(&self) -> u32 {
        match self {
            Self::FiveOfAKind => 6,
            Self::FourOfAKind => 5,
            Self::FullHouse => 4,
            Self::ThreeOfAKind => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }

    pub fn get_hand_type(cards: &Vec<Card>) -> Result<Self, &'static str> {
        if cards.len() != 5 {
            return Err("Hand contained invalid number of cards");
        }
        let counter = Self::count_cards(cards);
        let hand_type = match counter.values().max().unwrap() {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => {
                if counter.len() == 2 {
                    Self::FullHouse
                } else {
                    Self::ThreeOfAKind
                }
            }
            2 => {
                if counter.len() == 3 {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            }
            1 => Self::HighCard,
            _ => return Err("Something went wrong counting the numbers of cards"),
        };
        Ok(hand_type)
    }

    pub fn count_cards(cards: &Vec<Card>) -> HashMap<Card, u32> {
        let mut counter: HashMap<Card, u32> = HashMap::new();
        for c in cards {
            counter.entry(*c).and_modify(|x| *x += 1).or_insert(1);
        }
        counter
    }

    pub fn apply_joker_rule(cards: &Vec<Card>) -> Result<Self, &'static str> {
        let mut counter: HashMap<Card, u32> = HashMap::new();
        let mut jokers: u32 = 0;
        for card in cards {
            if card == &Card::J {
                jokers += 1;
            } else {
                counter.entry(*card).and_modify(|x| *x += 1).or_insert(1);
            }
        }

        // i hate this and myself
        match jokers {
            0 => Self::get_hand_type(cards),
            1 => {
                if counter.len() == 4 {
                    // all four cards are different e.g JAQT6
                    Ok(Self::OnePair)
                } else if counter.len() == 3 {
                    // three different cards e.g 7JK73
                    Ok(Self::ThreeOfAKind)
                } else if counter.len() == 2 {
                    // two different cards e.g 99J55 or 999J5
                    match counter.iter().map(|(_, y)| y).max_by(|x, y| x.cmp(y)) {
                        Some(n) => {
                            if *n == 3 {
                                return Ok(Self::FourOfAKind);
                            } else if *n == 2 {
                                return Ok(Self::FullHouse);
                            } else {
                                return Err("I have no idea how this could happen");
                            }
                        }
                        None => Err("What the heck"),
                    }
                } else {
                    // all four cards are the same e.g AAAAJ
                    Ok(Self::FiveOfAKind)
                }
            }
            2 => {
                if counter.len() == 3 {
                    // three different cards e.g JJAQK
                    Ok(Self::ThreeOfAKind)
                } else if counter.len() == 2 {
                    // two different cards e.g JJKK8
                    Ok(Self::FourOfAKind)
                } else {
                    // other three cards are three of a kind
                    Ok(Self::FiveOfAKind)
                }
            }
            3 => {
                if counter.len() == 2 {
                    // two different cards
                    Ok(Self::FourOfAKind)
                } else {
                    Ok(Self::FiveOfAKind)
                }
            }
            4 | 5 => Ok(Self::FiveOfAKind),
            _ => Err("wtf"),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_value().cmp(&other.get_value()))
    }
}

// there has to be a better way
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => Ordering::Equal,
            (HandType::FiveOfAKind, HandType::FourOfAKind) => Ordering::Greater,
            (HandType::FiveOfAKind, HandType::FullHouse) => Ordering::Greater,
            (HandType::FiveOfAKind, HandType::ThreeOfAKind) => Ordering::Greater,
            (HandType::FiveOfAKind, HandType::TwoPair) => Ordering::Greater,
            (HandType::FiveOfAKind, HandType::OnePair) => Ordering::Greater,
            (HandType::FiveOfAKind, HandType::HighCard) => Ordering::Greater,
            (HandType::FourOfAKind, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::FourOfAKind, HandType::FourOfAKind) => Ordering::Equal,
            (HandType::FourOfAKind, HandType::FullHouse) => Ordering::Greater,
            (HandType::FourOfAKind, HandType::ThreeOfAKind) => Ordering::Greater,
            (HandType::FourOfAKind, HandType::TwoPair) => Ordering::Greater,
            (HandType::FourOfAKind, HandType::OnePair) => Ordering::Greater,
            (HandType::FourOfAKind, HandType::HighCard) => Ordering::Greater,
            (HandType::FullHouse, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::FullHouse, HandType::FourOfAKind) => Ordering::Less,
            (HandType::FullHouse, HandType::FullHouse) => Ordering::Equal,
            (HandType::FullHouse, HandType::ThreeOfAKind) => Ordering::Greater,
            (HandType::FullHouse, HandType::TwoPair) => Ordering::Greater,
            (HandType::FullHouse, HandType::OnePair) => Ordering::Greater,
            (HandType::FullHouse, HandType::HighCard) => Ordering::Greater,
            (HandType::ThreeOfAKind, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::ThreeOfAKind, HandType::FourOfAKind) => Ordering::Less,
            (HandType::ThreeOfAKind, HandType::FullHouse) => Ordering::Less,
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => Ordering::Equal,
            (HandType::ThreeOfAKind, HandType::TwoPair) => Ordering::Greater,
            (HandType::ThreeOfAKind, HandType::OnePair) => Ordering::Greater,
            (HandType::ThreeOfAKind, HandType::HighCard) => Ordering::Greater,
            (HandType::TwoPair, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::TwoPair, HandType::FourOfAKind) => Ordering::Less,
            (HandType::TwoPair, HandType::FullHouse) => Ordering::Less,
            (HandType::TwoPair, HandType::ThreeOfAKind) => Ordering::Less,
            (HandType::TwoPair, HandType::TwoPair) => Ordering::Equal,
            (HandType::TwoPair, HandType::OnePair) => Ordering::Greater,
            (HandType::TwoPair, HandType::HighCard) => Ordering::Greater,
            (HandType::OnePair, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::OnePair, HandType::FourOfAKind) => Ordering::Less,
            (HandType::OnePair, HandType::FullHouse) => Ordering::Less,
            (HandType::OnePair, HandType::ThreeOfAKind) => Ordering::Less,
            (HandType::OnePair, HandType::TwoPair) => Ordering::Less,
            (HandType::OnePair, HandType::OnePair) => Ordering::Equal,
            (HandType::OnePair, HandType::HighCard) => Ordering::Greater,
            (HandType::HighCard, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::HighCard, HandType::FourOfAKind) => Ordering::Less,
            (HandType::HighCard, HandType::FullHouse) => Ordering::Less,
            (HandType::HighCard, HandType::ThreeOfAKind) => Ordering::Less,
            (HandType::HighCard, HandType::TwoPair) => Ordering::Less,
            (HandType::HighCard, HandType::OnePair) => Ordering::Less,
            (HandType::HighCard, HandType::HighCard) => Ordering::Equal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    // for part 1
    /*     pub fn get_value(&self) -> u32 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    } */

    pub fn get_value(&self) -> u32 {
        match self {
            Card::A => 13,
            Card::K => 12,
            Card::Q => 11,
            Card::T => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::J => 0,
        }
    }
    pub fn parse(c: char) -> Result<Card, &'static str> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err("Found invalid character"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_value().cmp(&other.get_value()))
    }
}

// why
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Card::A, Card::A) => Ordering::Equal,
            (Card::A, Card::K) => Ordering::Greater,
            (Card::A, Card::Q) => Ordering::Greater,
            (Card::A, Card::J) => Ordering::Greater,
            (Card::A, Card::T) => Ordering::Greater,
            (Card::A, Card::Nine) => Ordering::Greater,
            (Card::A, Card::Eight) => Ordering::Greater,
            (Card::A, Card::Seven) => Ordering::Greater,
            (Card::A, Card::Six) => Ordering::Greater,
            (Card::A, Card::Five) => Ordering::Greater,
            (Card::A, Card::Four) => Ordering::Greater,
            (Card::A, Card::Three) => Ordering::Greater,
            (Card::A, Card::Two) => Ordering::Greater,
            (Card::K, Card::A) => Ordering::Less,
            (Card::K, Card::K) => Ordering::Equal,
            (Card::K, Card::Q) => Ordering::Greater,
            (Card::K, Card::J) => Ordering::Greater,
            (Card::K, Card::T) => Ordering::Greater,
            (Card::K, Card::Nine) => Ordering::Greater,
            (Card::K, Card::Eight) => Ordering::Greater,
            (Card::K, Card::Seven) => Ordering::Greater,
            (Card::K, Card::Six) => Ordering::Greater,
            (Card::K, Card::Five) => Ordering::Greater,
            (Card::K, Card::Four) => Ordering::Greater,
            (Card::K, Card::Three) => Ordering::Greater,
            (Card::K, Card::Two) => Ordering::Greater,
            (Card::Q, Card::A) => Ordering::Less,
            (Card::Q, Card::K) => Ordering::Less,
            (Card::Q, Card::Q) => Ordering::Equal,
            (Card::Q, Card::J) => Ordering::Greater,
            (Card::Q, Card::T) => Ordering::Greater,
            (Card::Q, Card::Nine) => Ordering::Greater,
            (Card::Q, Card::Eight) => Ordering::Greater,
            (Card::Q, Card::Seven) => Ordering::Greater,
            (Card::Q, Card::Six) => Ordering::Greater,
            (Card::Q, Card::Five) => Ordering::Greater,
            (Card::Q, Card::Four) => Ordering::Greater,
            (Card::Q, Card::Three) => Ordering::Greater,
            (Card::Q, Card::Two) => Ordering::Greater,
            (Card::J, Card::A) => Ordering::Less,
            (Card::J, Card::K) => Ordering::Less,
            (Card::J, Card::Q) => Ordering::Less,
            (Card::J, Card::J) => Ordering::Equal,
            /*             (Card::J, Card::T) => Ordering::Greater,
            (Card::J, Card::Nine) => Ordering::Greater,
            (Card::J, Card::Eight) => Ordering::Greater,
            (Card::J, Card::Seven) => Ordering::Greater,
            (Card::J, Card::Six) => Ordering::Greater,
            (Card::J, Card::Five) => Ordering::Greater,
            (Card::J, Card::Four) => Ordering::Greater,
            (Card::J, Card::Three) => Ordering::Greater,
            (Card::J, Card::Two) => Ordering::Greater, */
            (Card::J, Card::T) => Ordering::Less,
            (Card::J, Card::Nine) => Ordering::Less,
            (Card::J, Card::Eight) => Ordering::Less,
            (Card::J, Card::Seven) => Ordering::Less,
            (Card::J, Card::Six) => Ordering::Less,
            (Card::J, Card::Five) => Ordering::Less,
            (Card::J, Card::Four) => Ordering::Less,
            (Card::J, Card::Three) => Ordering::Less,
            (Card::J, Card::Two) => Ordering::Less,
            (Card::T, Card::A) => Ordering::Less,
            (Card::T, Card::K) => Ordering::Less,
            (Card::T, Card::Q) => Ordering::Less,
            //(Card::T, Card::J) => Ordering::Less,
            (Card::T, Card::J) => Ordering::Greater,
            (Card::T, Card::T) => Ordering::Equal,
            (Card::T, Card::Nine) => Ordering::Greater,
            (Card::T, Card::Eight) => Ordering::Greater,
            (Card::T, Card::Seven) => Ordering::Greater,
            (Card::T, Card::Six) => Ordering::Greater,
            (Card::T, Card::Five) => Ordering::Greater,
            (Card::T, Card::Four) => Ordering::Greater,
            (Card::T, Card::Three) => Ordering::Greater,
            (Card::T, Card::Two) => Ordering::Greater,
            (Card::Nine, Card::A) => Ordering::Less,
            (Card::Nine, Card::K) => Ordering::Less,
            (Card::Nine, Card::Q) => Ordering::Less,
            //(Card::Nine, Card::J) => Ordering::Less,
            (Card::Nine, Card::J) => Ordering::Greater,
            (Card::Nine, Card::T) => Ordering::Less,
            (Card::Nine, Card::Nine) => Ordering::Equal,
            (Card::Nine, Card::Eight) => Ordering::Greater,
            (Card::Nine, Card::Seven) => Ordering::Greater,
            (Card::Nine, Card::Six) => Ordering::Greater,
            (Card::Nine, Card::Five) => Ordering::Greater,
            (Card::Nine, Card::Four) => Ordering::Greater,
            (Card::Nine, Card::Three) => Ordering::Greater,
            (Card::Nine, Card::Two) => Ordering::Greater,
            (Card::Eight, Card::A) => Ordering::Less,
            (Card::Eight, Card::K) => Ordering::Less,
            (Card::Eight, Card::Q) => Ordering::Less,
            //(Card::Eight, Card::J) => Ordering::Less,
            (Card::Eight, Card::J) => Ordering::Greater,
            (Card::Eight, Card::T) => Ordering::Less,
            (Card::Eight, Card::Nine) => Ordering::Less,
            (Card::Eight, Card::Eight) => Ordering::Equal,
            (Card::Eight, Card::Seven) => Ordering::Greater,
            (Card::Eight, Card::Six) => Ordering::Greater,
            (Card::Eight, Card::Five) => Ordering::Greater,
            (Card::Eight, Card::Four) => Ordering::Greater,
            (Card::Eight, Card::Three) => Ordering::Greater,
            (Card::Eight, Card::Two) => Ordering::Greater,
            (Card::Seven, Card::A) => Ordering::Less,
            (Card::Seven, Card::K) => Ordering::Less,
            (Card::Seven, Card::Q) => Ordering::Less,
            //(Card::Seven, Card::J) => Ordering::Less,
            (Card::Seven, Card::J) => Ordering::Greater,
            (Card::Seven, Card::T) => Ordering::Less,
            (Card::Seven, Card::Nine) => Ordering::Less,
            (Card::Seven, Card::Eight) => Ordering::Less,
            (Card::Seven, Card::Seven) => Ordering::Equal,
            (Card::Seven, Card::Six) => Ordering::Greater,
            (Card::Seven, Card::Five) => Ordering::Greater,
            (Card::Seven, Card::Four) => Ordering::Greater,
            (Card::Seven, Card::Three) => Ordering::Greater,
            (Card::Seven, Card::Two) => Ordering::Greater,
            (Card::Six, Card::A) => Ordering::Less,
            (Card::Six, Card::K) => Ordering::Less,
            (Card::Six, Card::Q) => Ordering::Less,
            //(Card::Six, Card::J) => Ordering::Less,
            (Card::Six, Card::J) => Ordering::Greater,
            (Card::Six, Card::T) => Ordering::Less,
            (Card::Six, Card::Nine) => Ordering::Less,
            (Card::Six, Card::Eight) => Ordering::Less,
            (Card::Six, Card::Seven) => Ordering::Less,
            (Card::Six, Card::Six) => Ordering::Equal,
            (Card::Six, Card::Five) => Ordering::Greater,
            (Card::Six, Card::Four) => Ordering::Greater,
            (Card::Six, Card::Three) => Ordering::Greater,
            (Card::Six, Card::Two) => Ordering::Greater,
            (Card::Five, Card::A) => Ordering::Less,
            (Card::Five, Card::K) => Ordering::Less,
            (Card::Five, Card::Q) => Ordering::Less,
            //(Card::Five, Card::J) => Ordering::Less,
            (Card::Five, Card::J) => Ordering::Greater,
            (Card::Five, Card::T) => Ordering::Less,
            (Card::Five, Card::Nine) => Ordering::Less,
            (Card::Five, Card::Eight) => Ordering::Less,
            (Card::Five, Card::Seven) => Ordering::Less,
            (Card::Five, Card::Six) => Ordering::Less,
            (Card::Five, Card::Five) => Ordering::Equal,
            (Card::Five, Card::Four) => Ordering::Greater,
            (Card::Five, Card::Three) => Ordering::Greater,
            (Card::Five, Card::Two) => Ordering::Greater,
            (Card::Four, Card::A) => Ordering::Less,
            (Card::Four, Card::K) => Ordering::Less,
            (Card::Four, Card::Q) => Ordering::Less,
            //(Card::Four, Card::J) => Ordering::Less,
            (Card::Four, Card::J) => Ordering::Greater,
            (Card::Four, Card::T) => Ordering::Less,
            (Card::Four, Card::Nine) => Ordering::Less,
            (Card::Four, Card::Eight) => Ordering::Less,
            (Card::Four, Card::Seven) => Ordering::Less,
            (Card::Four, Card::Six) => Ordering::Less,
            (Card::Four, Card::Five) => Ordering::Less,
            (Card::Four, Card::Four) => Ordering::Equal,
            (Card::Four, Card::Three) => Ordering::Greater,
            (Card::Four, Card::Two) => Ordering::Greater,
            (Card::Three, Card::A) => Ordering::Less,
            (Card::Three, Card::K) => Ordering::Less,
            (Card::Three, Card::Q) => Ordering::Less,
            //(Card::Three, Card::J) => Ordering::Less,
            (Card::Three, Card::J) => Ordering::Greater,
            (Card::Three, Card::T) => Ordering::Less,
            (Card::Three, Card::Nine) => Ordering::Less,
            (Card::Three, Card::Eight) => Ordering::Less,
            (Card::Three, Card::Seven) => Ordering::Less,
            (Card::Three, Card::Six) => Ordering::Less,
            (Card::Three, Card::Five) => Ordering::Less,
            (Card::Three, Card::Four) => Ordering::Less,
            (Card::Three, Card::Three) => Ordering::Equal,
            (Card::Three, Card::Two) => Ordering::Greater,
            (Card::Two, Card::A) => Ordering::Less,
            (Card::Two, Card::K) => Ordering::Less,
            (Card::Two, Card::Q) => Ordering::Less,
            //(Card::Two, Card::J) => Ordering::Less,
            (Card::Two, Card::J) => Ordering::Greater,
            (Card::Two, Card::T) => Ordering::Less,
            (Card::Two, Card::Nine) => Ordering::Less,
            (Card::Two, Card::Eight) => Ordering::Less,
            (Card::Two, Card::Seven) => Ordering::Less,
            (Card::Two, Card::Six) => Ordering::Less,
            (Card::Two, Card::Five) => Ordering::Less,
            (Card::Two, Card::Four) => Ordering::Less,
            (Card::Two, Card::Three) => Ordering::Less,
            (Card::Two, Card::Two) => Ordering::Equal,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
    pub bid: u64,
}

impl Hand {
    pub fn from(cards: Vec<Card>, bid: u64) -> Result<Hand, &'static str> {
        Ok(Hand {
            hand_type: HandType::get_hand_type(&cards).unwrap(),
            cards: cards,
            bid: bid,
        })
    }

    pub fn from2(cards: Vec<Card>, bid: u64) -> Result<Hand, &'static str> {
        Ok(Hand {
            hand_type: HandType::apply_joker_rule(&cards).unwrap(),
            cards: cards,
            bid: bid,
        })
    }

    pub fn from_string(input: &str) -> Result<Hand, &'static str> {
        let (hand, bid) = input.split_once(' ').unwrap();
        let bid = u64::from_str_radix(bid, 10).unwrap();
        let mut cards: Vec<Card> = vec![];
        for c in hand.chars() {
            let card = Card::parse(c).unwrap();
            cards.push(card);
        }
        if cards.len() != 5 {
            return Err("Hand contained invalid number of cards");
        }
        Hand::from(cards, bid)
    }

    pub fn from_string2(input: &str) -> Result<Hand, &'static str> {
        let (hand, bid) = input.split_once(' ').unwrap();
        let bid = u64::from_str_radix(bid, 10).unwrap();
        let mut cards: Vec<Card> = vec![];
        for c in hand.chars() {
            let card = Card::parse(c).unwrap();
            cards.push(card);
        }
        if cards.len() != 5 {
            return Err("Hand contained invalid number of cards");
        }
        Hand::from2(cards, bid)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {
                for (a, b) in zip(&self.cards, &other.cards) {
                    match a.cmp(&b) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                    }
                }
            }
        }
        Ordering::Equal
    }
}

pub fn rank_hands(input: &str) -> BTreeSet<Hand> {
    let mut hands: BTreeSet<Hand> = BTreeSet::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(l) = line {
                hands.insert(Hand::from_string(&l).unwrap());
            }
        }
    }
    hands
}

pub fn rank_hands2(input: &str) -> BTreeSet<Hand> {
    let mut hands: BTreeSet<Hand> = BTreeSet::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(l) = line {
                hands.insert(Hand::from_string2(&l).unwrap());
            }
        }
    }
    hands
}

pub fn calculate_total_winnings(hands: &BTreeSet<Hand>) -> u64 {
    let mut total_winnings: u64 = 0;
    for (i, h) in hands.iter().enumerate() {
        total_winnings += (i as u64 + 1) * h.bid;
    }
    total_winnings
}

// shamelessly stolen from Rust By Example
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        let input: Vec<&str> = vec![
            "AAAAA 345",
            "44474 1420",
            "QQQJA 483",
            "9332T 10",
            "88KKK 99",
            "7T25Q 1",
            "3AA66 27",
        ];
        assert_eq!(
            Ok(Hand {
                cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A],
                hand_type: HandType::FiveOfAKind,
                bid: 345,
            }),
            Hand::from_string(input[0])
        );
        assert_eq!(
            Ok(Hand {
                cards: vec![Card::Four, Card::Four, Card::Four, Card::Seven, Card::Four],
                hand_type: HandType::FourOfAKind,
                bid: 1420,
            }),
            Hand::from_string(input[1])
        );
        assert_eq!(
            Ok(Hand {
                cards: vec![Card::Q, Card::Q, Card::Q, Card::J, Card::A],
                hand_type: HandType::ThreeOfAKind,
                bid: 483,
            }),
            Hand::from_string(input[2])
        );
        assert_eq!(
            Ok(Hand {
                cards: vec![Card::Nine, Card::Three, Card::Three, Card::Two, Card::T],
                hand_type: HandType::OnePair,
                bid: 10,
            }),
            Hand::from_string(input[3])
        );
        assert_eq!(
            Ok(Hand {
                cards: vec![Card::Eight, Card::Eight, Card::K, Card::K, Card::K],
                hand_type: HandType::FullHouse,
                bid: 99,
            }),
            Hand::from_string(input[4])
        );
        assert_eq!(
            Ok(Hand {
                cards: vec![Card::Seven, Card::T, Card::Two, Card::Five, Card::Q],
                hand_type: HandType::HighCard,
                bid: 1,
            }),
            Hand::from_string(input[5])
        );
        assert_eq!(
            Ok(Hand {
                cards: vec![Card::Three, Card::A, Card::A, Card::Six, Card::Six],
                hand_type: HandType::TwoPair,
                bid: 27,
            }),
            Hand::from_string(input[6])
        );
    }

    #[test]
    fn test_apply_joker_rule() {
        // joker rule turns this hand from AJ333 -> A3333 to make FourOfAKind
        let input1 = vec![Card::A, Card::J, Card::Three, Card::Three, Card::Three];
        // JJJJA -> AAAAA
        let input2 = vec![Card::J, Card::J, Card::J, Card::J, Card::A];
        // TQTJ2 -> TQTT2
        let input3 = vec![Card::T, Card::Q, Card::T, Card::J, Card::Two];
        // QQJTT -> QQQTT
        let input4 = vec![Card::Q, Card::Q, Card::J, Card::T, Card::T];
        // 4K8AT -> 4K8AT no jokers so no change
        let input5 = vec![Card::Four, Card::K, Card::Eight, Card::A, Card::T];
        // 4K8JT -> 4K8KT
        let input6 = vec![Card::Four, Card::K, Card::Eight, Card::J, Card::T];
        // 22447 -> 22447
        let input7 = vec![Card::Two, Card::Two, Card::Four, Card::Four, Card::Seven];
        // QJQJ4 -> QQQQ4
        let input8 = vec![Card::Q, Card::J, Card::Q, Card::J, Card::Four];
        // JJJJJ -> JJJJJ no change
        let input9 = vec![Card::J, Card::J, Card::J, Card::J, Card::J];
        assert_eq!(
            Ok(HandType::FourOfAKind),
            HandType::apply_joker_rule(&input1)
        );
        assert_eq!(
            Ok(HandType::FiveOfAKind),
            HandType::apply_joker_rule(&input2)
        );
        assert_eq!(
            Ok(HandType::ThreeOfAKind),
            HandType::apply_joker_rule(&input3)
        );
        assert_eq!(Ok(HandType::FullHouse), HandType::apply_joker_rule(&input4));
        assert_eq!(Ok(HandType::HighCard), HandType::apply_joker_rule(&input5));
        assert_eq!(Ok(HandType::OnePair), HandType::apply_joker_rule(&input6));
        assert_eq!(Ok(HandType::TwoPair), HandType::apply_joker_rule(&input7));
        assert_eq!(
            Ok(HandType::FourOfAKind),
            HandType::apply_joker_rule(&input8)
        );
        assert_eq!(
            Ok(HandType::FiveOfAKind),
            HandType::apply_joker_rule(&input9)
        );
    }

    #[test]
    fn test_rank_hands() {
        let input = "test.txt";

        let mut expected: BTreeSet<Hand> = BTreeSet::new();
        let hands = vec![
            Hand {
                cards: vec![Card::Three, Card::Two, Card::T, Card::Three, Card::K],
                hand_type: HandType::OnePair,
                bid: 765,
            },
            Hand {
                cards: vec![Card::K, Card::T, Card::J, Card::J, Card::T],
                hand_type: HandType::TwoPair,
                bid: 220,
            },
            Hand {
                cards: vec![Card::K, Card::K, Card::Six, Card::Seven, Card::Seven],
                hand_type: HandType::TwoPair,
                bid: 28,
            },
            Hand {
                cards: vec![Card::T, Card::Five, Card::Five, Card::J, Card::Five],
                hand_type: HandType::ThreeOfAKind,
                bid: 684,
            },
            Hand {
                cards: vec![Card::Q, Card::Q, Card::Q, Card::J, Card::A],
                hand_type: HandType::ThreeOfAKind,
                bid: 483,
            },
        ];
        for h in hands {
            expected.insert(h);
        }
        assert_eq!(expected, rank_hands(input));
    }

    #[test]
    fn test_calculate_total_winnings() {
        let mut input: BTreeSet<Hand> = BTreeSet::new();
        let hands = vec![
            Hand {
                cards: vec![Card::Three, Card::Two, Card::T, Card::Three, Card::K],
                hand_type: HandType::OnePair,
                bid: 765,
            },
            Hand {
                cards: vec![Card::K, Card::T, Card::J, Card::J, Card::T],
                hand_type: HandType::TwoPair,
                bid: 220,
            },
            Hand {
                cards: vec![Card::K, Card::K, Card::Six, Card::Seven, Card::Seven],
                hand_type: HandType::TwoPair,
                bid: 28,
            },
            Hand {
                cards: vec![Card::Q, Card::Q, Card::Q, Card::J, Card::A],
                hand_type: HandType::ThreeOfAKind,
                bid: 483,
            },
            Hand {
                cards: vec![Card::T, Card::Five, Card::Five, Card::J, Card::Five],
                hand_type: HandType::ThreeOfAKind,
                bid: 684,
            },
        ];
        for h in hands {
            input.insert(h);
        }
        assert_eq!(6440, calculate_total_winnings(&input));
    }
}
