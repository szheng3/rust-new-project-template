# Weekly Rust Progress Report
Build a useful API server in the domain of data engineering or machine learning engineering.

## Week 13 Progress

This week, I added the upload function in the rust server. The server can now upload a file to the server and store it in the `uploads` folder. The server will also return the file name and the file size in the response.
```

async fn save_file(mut field: Field) -> Result<String, std::io::Error> {
    let mut upload_dir = PathBuf::from(std::env::current_dir().unwrap());
    upload_dir.push("upload");

    if !upload_dir.exists() {
        match fs::create_dir(&upload_dir) {
            Ok(_) => println!("Created directory: {:?}", upload_dir),
            Err(e) => panic!("Failed to create directory {:?}: {}", upload_dir, e),
        }
    }
    let mut file_name = None;
    let content_disposition = field.content_disposition();
    if let Some(name) = content_disposition.get_filename() {
        let upload_dir_string = upload_dir.to_string_lossy().to_string();

        file_name = Some(format!("{}/{}", upload_dir_string, name));
    }
    let file_path = file_name.unwrap();

    println!("{}", file_path);

    let mut file = std::fs::File::create(file_path.clone())?;

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        file.write_all(&data)?;
    }

    Ok(file_path)
}


```



### Usage
> Run `make run` in the terminal



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [server](https://codevoweb.com/build-a-simple-api-with-rust-and-actix-web/)