fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license = subsonic_types::License {
        valid: true,
        ..Default::default()
    };
    let response = subsonic_types::Response {
        status: subsonic_types::ResponseStatus::Ok,
        version: subsonic_types::Version::new(1, 12, 0),
        body: subsonic_types::ResponseBody::License(license),
    };

    println!("{}", subsonic_types::to_json(&response)?);
    println!("{}", subsonic_types::to_xml(&response)?);

    let folders = subsonic_types::MusicFolders {
        music_folder: vec![
            subsonic_types::MusicFolder {
                id: 1,
                name: Some("Music".to_string()),
            },
            subsonic_types::MusicFolder {
                id: 2,
                name: Some("Podcasts".to_string()),
            },
        ],
    };
    let response = subsonic_types::Response {
        status: subsonic_types::ResponseStatus::Ok,
        version: subsonic_types::Version::new(1, 12, 0),
        body: subsonic_types::ResponseBody::MusicFolders(folders),
    };
    println!("{}", subsonic_types::to_json(&response)?);
    println!("{}", subsonic_types::to_xml(&response)?);

    Ok(())
}
