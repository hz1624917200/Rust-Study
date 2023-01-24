// trait test
pub trait Summary {
	fn summarize(&self) -> String {
		String::from("Read more...")
	}
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
	pub date: String,
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} on {} ({})", self.headline, self.author, self.date, self.location)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

fn notify<T: Summary>(item: T) {
	println!("New news! Summary of the news: {}", item.summarize())
}

fn main () {
	let news_a = NewsArticle {
		headline: String::from("Zheng Huang's paper got published"),
		location: String::from("China"),
		author: String::from("Unknown"),
		content: String::from("This is content of the news"),
		date: String::from("1-24-2023"),
	};
	notify(news_a);
}