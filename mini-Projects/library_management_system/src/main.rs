#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_available: bool,
}
#[derive(Debug)]
struct Library {
    name: String,
    address: String,
    book: Option<Book>,
}

#[derive(Debug)]
enum LibraryError {
    BookNotAvailable,
    BookNotFound,
    AlreadyBorrowed,
    BookAlreadyExists,
}

impl Book {
    fn borrow(&mut self) -> Result<&mut Book, LibraryError> {
        if self.is_available == false {
            self.is_available = true;
            Ok(self)
        } else {
            Err(LibraryError::AlreadyBorrowed)
        }
    }
    fn return_book(&mut self) {
        if self.is_available == true {
            self.is_available = false
        }
    }
}
impl Library {
    fn add_book(&mut self, book: Book) -> Result<(), LibraryError> {
        if self.book.is_none() {
            self.book = Some(book);
            Ok(())
        } else {
            Err(LibraryError::BookAlreadyExists)
        }
    }

    fn borrow_book(&mut self) -> Result<&mut Book, LibraryError> {
        if let Some(book) = self.book.as_mut() {
            match book.borrow() {
                Ok(borrowed_book) => {
                    println!("Books is borrowed");
                    Ok(borrowed_book)
                }
                Err(err) => {
                    println!("Book is already borrowed");
                    Err(err)
                }
            }
        } else {
            Err(LibraryError::BookNotFound)
        }
    }
    fn return_book(&mut self) {
        if let Some(book) = self.book.as_mut() {
            book.return_book();
        } else {
            println!("No book found in library.");
        }
    }
}

fn main() {
    let book = Book {
        title: "The Rust Book".to_string(),
        author: "SK".to_string(),
        is_available: true,
    };

    let mut library = Library {
        name: "City Library".to_string(),
        address: "123 Library Lane".to_string(),
        book: None,
    };

    match library.add_book(book) {
        Ok(_) => {
            println!("Book Added");
        }
        Err(err) => {
            println!("{:?}", err)
        }
    };

    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed Book: {:?}", result);
        }
        Err(err) => {
            println!("Borrowed Book: {:?}", err);
        }
    }
}
