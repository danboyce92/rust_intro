#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}



fn main() {
    let suits = ["hearts", "spades", "diamonds", "clubs"];
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];
    let cards = vec![];
    
    let deck = Deck { cards: vec![] };

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    println!("Here's your deck: {:?}", deck);
}
