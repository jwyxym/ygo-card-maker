use actix_multipart::Multipart;
use futures::StreamExt;
use actix_cors::Cors;
use actix_web::{
	HttpServer,
	HttpResponse,
	Error,
	App,
	post,
	get,
	error::{
		ErrorNotFound
	}
};
use std::{
	fs::{
		File,
		create_dir_all,
		read_to_string
	},
	io::{
		Write
	}
};

#[post("/api/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
	let mut create_file: bool = false;
	let path: &str = "pics";
	while let Some(item) = payload.next().await {
		if let Ok(mut field) = item {
			if let Some(content_disposition) = field.content_disposition() {
				if let Some(filename) = content_disposition.get_filename() {
					println!("/api/upload/{}", filename);
					create_dir_all(path)?;
					let filepath: String = format!("./{}/{}", path, filename);
					let mut file: File = File::create(&filepath)?;
					while let Some(chunk) = field.next().await {
						if let Ok(data) = chunk {
							file.write_all(&data)?;
						}
					}
					create_file = true;
				}
			}
		}
	}
	if create_file {
		Ok(HttpResponse::Ok().body("Ok"))
	} else {
		Err(ErrorNotFound("File Error".to_string()))
	}
}

#[get("/api/get")]
async fn get() -> Result<HttpResponse, Error> {
	let pics: String = read_to_string("pics.json")?;
	let files: Vec<i64> = serde_json::from_str(&pics)?;
	Ok(HttpResponse::Ok().json(files))
}

#[actix_web::main]
async fn main() -> () {
	let port: i32 = 5174;
	let server = HttpServer::new(move || {
		App::new()
			.wrap(Cors::permissive())
			.service(upload)
			.service(get)
	})
	.bind(format!("127.0.0.1:{}", port));

	if let Ok(server) = server {
		let _ = server.run().await;
	}
}
