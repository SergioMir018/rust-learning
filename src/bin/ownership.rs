struct Book {
    pages: i32,
    rating: i32
}

fn display_page_count(book: &Book) {
    println!("pages: {:?}", book.pages)
}

fn display_rating(book: &Book) {
    println!("rating: {:?}", book.rating)
}

fn main(){
    let my_book = Book {
        pages: 380,
        rating: 100
   };

    display_page_count(&my_book);
    display_rating(&my_book);
}
