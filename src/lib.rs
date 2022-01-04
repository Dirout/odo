use std::io::Write;

use ansi_term::Style;


/// Handle a subcommand's script
pub fn handle(script: String, args: liquid::Object) {
	let rendered = render(script, args);
	let process = run_script::spawn_script!(rendered).unwrap();
	let output = process.wait_with_output().expect("Process did not start.");

	println!("▶ {}", Style::new().bold().paint(output.status.to_string()));

	println!("▶ {}: ", Style::new().bold().paint("output"));
	std::io::stdout().write_all(&output.stdout);

	println!("▶ {}: ", Style::new().bold().paint("errors"));
	std::io::stdout().write_all(&output.stderr);
}

/// Render a subcommand's script
pub fn render(script: String, args: liquid::Object) -> String {
	let template = create_liquid_parser().parse(&script).unwrap();
	template.render(&args).unwrap()
}

/// Creates a Liquid parser
pub fn create_liquid_parser() -> liquid::Parser {
	liquid::ParserBuilder::with_stdlib()
		.tag(liquid_lib::jekyll::IncludeTag)
		.filter(liquid_lib::jekyll::ArrayToSentenceString)
		.filter(liquid_lib::jekyll::Pop)
		.filter(liquid_lib::jekyll::Push)
		.filter(liquid_lib::jekyll::Shift)
		.filter(liquid_lib::jekyll::Slugify)
		.filter(liquid_lib::jekyll::Unshift)
		.filter(liquid_lib::shopify::Pluralize)
		.filter(liquid_lib::extra::DateInTz)
		.build()
		.unwrap()
}
