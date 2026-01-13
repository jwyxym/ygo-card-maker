use fantoccini::{
	Client, Locator
};
use std::{
	thread::sleep,
	time::Duration,
	fs::read_dir,
	fs::File,
	io::Write
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	let client: Client = Client::new("http://localhost:9515").await?;
	
	client.goto("http://localhost:5173/").await?;
	loop {
		if client.find(Locator::Css(".fixed")).await.is_ok() {
			break;
		}
		sleep(Duration::from_secs(1));
	}
	client.close().await?;

	let mut files: Vec<i64> = Vec::new();
	
	for entry in read_dir("../back/pics")? {
		let entry = entry?;
		let path = entry.path();
		
		if path.is_file() {
			if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
				if let Ok(name) = name.to_string().parse::<i64>() {
					files.push(name);
				}
			}
		}
	}
	let json_string: String = serde_json::to_string(&files)?;
	let mut file = File::create("pics.json")?;
	file.write_all(json_string.as_bytes())?;
	Ok(())
}
