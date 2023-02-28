use subsonic_types::{
    common::Version,
    response::{ArtistID3, ArtistsID3, IndexID3, Response, ResponseBody},
};

#[test]
fn get_artists() {
    let response = Response::from_json(include_str!("get-artists.json")).unwrap();
    let expected = Response::ok(Version::V1_16_1, ResponseBody::Artists(
        ArtistsID3{
            index: vec![
                IndexID3{
                    name: "H".to_string(),
                    artist: vec![
                        ArtistID3{
                            id: "57fd6b271014d8b0a2fcb9124b143b53".to_string(),
                            name: "Haywyre".to_string(),
                            album_count: 20,
                            cover_art: Some("ar-57fd6b271014d8b0a2fcb9124b143b53_0".to_string()),
                            artist_image_url: Some("http://localhost:4533/share/img/eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6ImFyLTU3ZmQ2YjI3MTAxNGQ4YjBhMmZjYjkxMjRiMTQzYjUzXzAiLCJpc3MiOiJORCJ9.-HInypoa_50Fg-F7mlycP-L-aBSFI_y0F7OeYItLI5A?size=600".to_string()),
                            starred:None
                        }
                    ]
                }
            ],
            ignored_articles: "The El La Los Las Le Les Os As O A".to_string(),
        }
    ));
    assert_eq!(response, expected);
}
