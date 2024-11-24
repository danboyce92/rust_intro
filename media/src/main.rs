#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    fn decription(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie { title, director } => {
                format!("Movie {} {}", title, director)
            },
            Media::Audiobook { title } => {
                format!("Audiobook {}", title)
            },
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            },
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
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

    match catalog.items.get(0) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index");
        }
    }


}
