/**
 * Value of a hand of cards
 * Calculate the value of a Blackjack hand
 * implement the value method for Hand struct
 */
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Card {
    Ace,
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
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut running_total: usize = 0;
        let ace_count = self.cards.iter().filter(|&n| *n == Card::Ace).count();

        for card in &self.cards {
            // what to add
            let to_add: usize = match card {
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Ten | Card::Jack | Card::Queen | Card::King => 10,
                Card::Ace => 11,
                // _ => unreachable!()
            };

            running_total += to_add
        }

        // we have to handle the aces here
        if running_total > 21 && ace_count > 1 {
            println!("{}", running_total);
            for _element in 2..ace_count {
                if running_total - 10 > 21 {
                    running_total -= 10
                }
            }
        }

        // // have to handle ace_count here
        // if running_total + to_add > 21 && ace_count > 0 {
        //     let mut score = running_total;

        //     // filter all non-aces
        //     // then process the aces

        //     // for _element in 1..=ace_count {
        //     //     if running_total + 11 > 21 {
        //     //         running_total -= 10
        //     //     }
        //     // }
        //     // std::cmp::min(T, T)
        // } else {
        //     running_total += to_add
        // }
        // // loop through number of aces
        // // total + 11 > 21 -> decrement
        // // else add 1

        running_total
    }

    fn is_losing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::Ace);
    hand.add(Card::Five);
    hand.add(Card::Ace);
    // hand.add(Card::Ace);
    // hand.add(Card::Five);
    // hand.add(Card::Seven);
    if hand.is_losing_hand() {
        println!("Loser: {}", hand.value());
    } else {
        println!("The hand value is {}", hand.value());
    }
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_losing_hand());
    assert_eq!(hand.value(), 22);
}

#[test]
fn ace_first() {
    let mut hand: Hand = Hand::new();
    hand.add(Card::Ace);
    hand.add(Card::Five);
    hand.add(Card::Seven);
    assert_eq!(hand.value(), 13);
}
