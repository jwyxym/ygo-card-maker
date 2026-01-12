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
		read_dir
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
    let mut files: Vec<i64> = Vec::new();
    
    for entry in read_dir("pics")? {
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
