fn main() {
    let article = Article {
        headline: String::from("Breaking News!"),
    };
    println!("Article Summary: {}", article.summary());
}

trait Summarizable {
    fn summary<'a>(&'a self) -> &'a str;
}

struct Article {
    headline: String,
}

impl Summarizable for Article {
    fn summary(&self) -> &str {
        &self.headline
    }
}