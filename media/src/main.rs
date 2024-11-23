#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
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
            }
        }
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

    println!("{}", audiobook.decription());
    println!("{}", good_movie.decription());
    println!("{}", bad_book.decription());


}
