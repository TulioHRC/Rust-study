enum BookStatus {
    Available,
    CheckedOut(String),
    Reserved(String)
}

struct Book {
    title: String,
    author: String,
    status: BookStatus
}

impl Book {
    fn new(title: &str, author: &str) -> Self{
        Book {
            title: title.to_string(),
            author: author.to_string(),
            status: BookStatus::Available
        }
    }

    fn change_status(&mut self, new_status: BookStatus){
        self.status = new_status;
    }

    fn display(&self){
        let status = match &self.status {
            BookStatus::Available => "available".to_string(),
            BookStatus::CheckedOut(borrower_name) => format!("checked out by {borrower_name}"),
            BookStatus::Reserved(borrower_name) => format!("reserved by {borrower_name}")
        };
        println!("{0}, {1} is {2}", self.title, self.author, status);
    }
}

fn main() {
    let mut book1 = Book::new("TESTE", "Oilut");
    book1.display();

    book1.change_status(BookStatus::CheckedOut("Josh".to_string()));
    book1.display();

    book1.change_status(BookStatus::Reserved("Luke".to_string()));
    book1.display();
}
