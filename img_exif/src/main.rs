use little_exif::metadata::Metadata;

fn main(){
    let image_path = std::path::Path::new("./assets/11533-rust-logo.png");

    let mut metadata = match Metadata::new_from_path(&image_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Erro ao ler metadados EXIF: {}", e);
            return;
        }
    };

    for (index, tag) in metadata.data().iter().enumerate() {
        println!("Tag {} : {:?}", index, tag);
    }

}
