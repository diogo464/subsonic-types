use subsonic_types::response::Response;

#[test]
fn example_get_random_songs() {
    //let value: serde_json::Value =
    //    serde_json::from_str(include_str!("../get-random-songs.json")).unwrap();
    //eprintln!("{:#?}", value);

    let response = Response::from_json(include_str!("../get-random-songs.json")).unwrap();
    eprintln!("{:#?}", response);
}

#[test]
fn example_get_artists() {
    let response = Response::from_json(include_str!("../get-artists.json")).unwrap();
    eprintln!("{:#?}", response);
}

#[test]
fn example_get_genres() {
    let response = Response::from_json(include_str!("../get-genres.json")).unwrap();
    eprintln!("{:#?}", response);
}

#[test]
fn example_get_lyrics() {
    let response = Response::from_json(include_str!("../get-lyrics.json")).unwrap();
    eprintln!("{:#?}", response);
}

#[test]
fn example_get_playlists() {
    let response = Response::from_json(include_str!("../get-playlists.json")).unwrap();
    eprintln!("{:#?}", response);
}

#[test]
fn example_get_playlist() {
    let response = Response::from_json(include_str!("../get-playlist.json")).unwrap();
    eprintln!("{:#?}", response);
}
