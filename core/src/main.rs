use rust_i18n::t;
use clap::Parser;

mod arg;

rust_i18n::i18n!("locales", fallback = "en");

pub fn t(key: &str) -> String {
    t!(key).to_string()
}

fn main() {
	let opt = arg::Args::parse();

	rust_i18n::set_locale(&opt.language);

	println!("{}", t("hello world"));
}
