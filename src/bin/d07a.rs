use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::env;
use std::str::FromStr;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let bids = input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple::<(_, _)>())
        .map(|bid| {
            bid.ok_or(()).and_then(|(hand, bid)| {
                Ok((hand.parse::<Hand>()?, bid.parse::<usize>().map_err(|_| ())?))
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "could not parse bid")?;

    let result: usize = bids
        .iter()
        .sorted_unstable_by_key(|(hand, _)| hand)
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(idx, bid)| (idx + 1) * bid)
        .sum();

    println!("{result}");
    Ok(())
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = s.trim().chars().map(Card::try_from);
        let cards = [
            cards.next().ok_or(())??,
            cards.next().ok_or(())??,
            cards.next().ok_or(())??,
            cards.next().ok_or(())??,
            cards.next().ok_or(())??,
        ];
        Ok(Self {
            cards,
            hand_type: HandType::from(cards.as_slice()),
        })
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card]> for HandType {
    fn from(hand: &[Card]) -> Self {
        fn n_of_a_kind(cards: &[Card], n: usize) -> bool {
            cards.windows(n).any(|n| n.iter().all_equal())
        }

        let hand = hand.iter().cloned().sorted_unstable().collect_vec();

        if n_of_a_kind(&hand, 5) {
            Self::FiveOfAKind
        } else if n_of_a_kind(&hand, 4) {
            Self::FourOfAKind
        } else if (0..hand.len()).any(|pivot| {
            let (left, right) = hand.split_at(pivot);
            n_of_a_kind(left, 2) && n_of_a_kind(right, 3)
                || n_of_a_kind(left, 3) && n_of_a_kind(right, 2)
        }) {
            Self::FullHouse
        } else if n_of_a_kind(&hand, 3) {
            Self::ThreeOfAKind
        } else if (0..hand.len()).any(|pivot| {
            let (left, right) = hand.split_at(pivot);
            n_of_a_kind(left, 2) && n_of_a_kind(right, 2)
        }) {
            Self::TwoPair
        } else if n_of_a_kind(&hand, 2) {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value.to_ascii_uppercase() {
            | '2' => Self::Two,
            | '3' => Self::Three,
            | '4' => Self::Four,
            | '5' => Self::Five,
            | '6' => Self::Six,
            | '7' => Self::Seven,
            | '8' => Self::Eight,
            | '9' => Self::Nine,
            | 'T' => Self::Ten,
            | 'J' => Self::Joker,
            | 'Q' => Self::Queen,
            | 'K' => Self::King,
            | 'A' => Self::Ace,
            | _ => Err(())?,
        })
    }
}
