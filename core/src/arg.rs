use clap::Parser;

fn banner() -> String {
	r#"RQM Project."#.to_string()
}

#[derive(Parser, Debug)]
#[command(version, about = banner(), long_about = banner())]
pub struct Args {
	#[arg(long, short, default_value = "en")]
	pub language: String,
}
