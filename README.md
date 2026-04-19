# Toscrape

This is a dummy module that scrapes the contents of
<https://books.toscrape.com>. This library was created as result of me trying to
learn Rust.

I've tried experimenting with traits, structs, enums, etc., with this project.

The documentation is generated and deployed automatically at <https://eeriemyxi.github.io/toscrape/>.

If you, for whatever startling reason, desire to use this library, 

```shell
cargo add --git https://github.com/eeriemyxi/toscrape.git
```

... You could add it as a dependency as shown above.

## Examples
#### Fetch a specific book
```rust
use toscrape::toscrape;

fn main() {
    dbg!(toscrape::fetch_book(
        "https://books.toscrape.com/catalogue/a-light-in-the-attic_1000/index.html"
    ))
    .unwrap();
}
```

#### Paginate through a particular category
```rust
use toscrape::toscrape;

fn main() {
     dbg!(
        toscrape::paginate_category(
            "https://books.toscrape.com/catalogue/category/books/historical_42/index.html"
        )
        .unwrap()
        .collect::<Vec<_>>()
    );
}
```
You could do:
```rust
for ... in toscrape::paginate_category("...").page(2) { ... }
```

To paginate from a particular page number.

#### Fetch and iterate all categories, then iterate every page, and then iterate every card in each page
```rust
use toscrape::toscrape;

fn main() {
    for category in toscrape::fetch_categories().unwrap() {
        dbg!(&category);
        for book in category.paginate().unwrap().flatten() {
            dbg!(&book);
            dbg!(&book.full());
        }
    }
}
```

#### Thread fetching book cards
This example uses threads (excl. main thread) to fetch the results faster.

> [!NOTE]
> This is limited to `BookCard` right now. Extending it to `BookDetails`
> is yet to be done.


```rust
use toscrape::toscrape;

fn main() {
    for category in toscrape::fetch_categories().unwrap() {
        dbg!(&category);
        for book in category.paginate().unwrap().thread_ahead(5).flatten() {
            dbg!(&book);
            dbg!(&book.full());
        }
    }
}
```
