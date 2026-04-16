use toscrape::toscrape;

fn main() {
    for category in toscrape::fetch_categories().unwrap() {
        dbg!(&category);
        for cards in category.pages() {
            for book in cards {
                dbg!(book.full());
            }
        }
    }
}
