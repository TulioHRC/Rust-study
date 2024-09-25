use std::collections::HashMap;

struct Book {
    id: usize,
    name: String,
    author_name: String
}

struct LibrarySystem {
    loaned_books: HashMap<usize, bool>,
    books: Vec<Book>,
}

impl LibrarySystem {
    pub fn new() -> Self{
        return LibrarySystem {
            loaned_books: HashMap::new(),
            books: Vec::new()
        }
    } 

    pub fn insert_book(&mut self, name: String, author_name: String){
        self.books.push(Book{
            id: self.books.len(),
            name,
            author_name
        })
    }

    pub fn print_library_report(&self){
        for book in &self.books {
            println!("Book {0} {1}, {2} - is it available? {3}", book.id, book.name, book.author_name, match self.loaned_books.get(&book.id) {
                Some(_book) => String::from("No"),
                None => String::from("Yes")
            })
        }
    }

    pub fn lend_book(&mut self, book_id: usize){
        match self.get_available_book(book_id) {
            Some(_book) => {
                self.loaned_books.insert(book_id, true);
            },
            None => {println!("No book available with id ({book_id}).");}
        }
        
    }

    fn get_available_book(&self, book_id: usize) -> Option<&Book> {
        for book in &self.books {
            if book.id == book_id {
                match self.loaned_books.get(&book_id){
                    Some(_book) => { println!("Book already loaned!"); },
                    None => { return Some(book); }
                }
            }
        }
        return None;
    }
}

fn main() {
    let mut library_system = LibrarySystem::new();
    library_system.insert_book(String::from("Caio's life"), String::from("Caio"));
    library_system.insert_book(String::from("Tulio's life"), String::from("Tulio"));
    library_system.insert_book(String::from("Pedrin's life"), String::from("Criminoso"));
    library_system.insert_book(String::from("Jack"), String::from("Killer"));
    library_system.lend_book(3);
    library_system.lend_book(4);

    library_system.print_library_report();

}