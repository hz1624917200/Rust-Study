struct ImportantExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
	fn foo(&self, s: &str) -> &str {
		self.part
	}
}

fn main() {

}