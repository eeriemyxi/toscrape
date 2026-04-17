use toscrape::toscrape;

fn main() {
    dbg!(toscrape::fetch_book(
        "https://books.toscrape.com/catalogue/a-light-in-the-attic_1000/index.html"
    ))
    .unwrap();

    dbg!(
        toscrape::paginate_category(
            "https://books.toscrape.com/catalogue/category/books/historical_42/index.html"
        )
        .collect::<Vec<_>>()
    );

    for category in toscrape::fetch_categories().unwrap() {
        dbg!(&category);
        for cards in category.pages() {
            let cards = cards.unwrap();
            for book in cards {
                dbg!(&book);
                dbg!(&book.full());
            }
        }
    }
}
