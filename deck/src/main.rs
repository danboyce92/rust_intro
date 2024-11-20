use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

//Called an Inherent Implementation
impl Deck {
    fn new() -> Self {
        let suits = ["hearts", "spades", "diamonds"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];
    
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
    
        Deck { cards }
        
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Here's your deck: {:#?}", deck);
}
