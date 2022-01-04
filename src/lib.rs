/// Handle a subcommand's script
pub fn handle(script: String, args: liquid::Object) {
	let rendered = render(script, args);
	let mut process = run_script::spawn_script!(rendered).unwrap();
	println!(
		"Exit status: {}",
		process.wait().expect("Process did not start.")
	);
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
