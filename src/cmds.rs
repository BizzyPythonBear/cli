pub fn simple(name: String, readonly: bool) -> anyhow::Result<()> {
	println!("Hello, {}, Readonly: {}", name, readonly);
	Ok(())
}

pub fn fat(name: String, michael: bool) -> anyhow::Result<()> {
	if michael {
		println!("{} is fats...", name);
	} else {
		println!("{} is not that fat...", name);
	}
	Ok(())
}