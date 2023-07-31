struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    fn new() -> Library {
        Library {books: Vec::new()} // Creates a new empty vector
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn print_books(&self) {
        for books in &self.books{
            println!("{} ({})", books.title, books.year)
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year) // Checks the year of the book
    }
}

fn main() {
    let mut library = Library::new();

    println!("The library is empty:\t\t library.is_empty() -> {}", library.is_empty());
    
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
    println!("The library is no longer empty:\t library.is_empty() -> {}", library.is_empty());
    
    print!("\n");
    library.print_books();
    
    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }
    
    println!("\nThe library has {} books:", library.len());
    library.print_books();
}