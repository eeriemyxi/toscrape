fn main() {
    dbg!(toscrape::fetch_book(
        "https://books.toscrape.com/catalogue/a-light-in-the-attic_1000/index.html"
    ))
    .unwrap();

    dbg!(
        toscrape::paginate_category(
            "https://books.toscrape.com/catalogue/category/books/historical_42/index.html"
        )
        .unwrap()
        .collect::<Vec<_>>()
    );

    for category in toscrape::fetch_categories().unwrap() {
        dbg!(&category);
        for book in category.paginate().unwrap().thread_ahead(3).flatten() {
            dbg!(&book);
            dbg!(&book.full());
        }
    }
}
