mod content;

use content::media::Media;
use content::catalog::Catalog;

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

fn main() {
    let audiobook = Media::Audiobook { 
        title: String::from("Audiobook") 
    };

    let good_movie = Media::Movie { 
        title: String::from("Good Title"), 
        director: String::from("Good Director") 
    };

    let bad_book = Media::Book { 
        title: String::from("Bad Book"), 
        author: String::from("Bad Author") 
    };
    let podcast = Media::Podcast(12);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.decription());
    // println!("{}", good_movie.decription());
    // println!("{}", bad_book.decription());
    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // match catalog.items.get(0) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     None => {
    //         println!("Nothing at that index");
    //     }
    // }

    match catalog.get_by_index(100) {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("Item: {:#?}", value);
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No value here!");
        }
    }

}
