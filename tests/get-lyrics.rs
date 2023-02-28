use subsonic_types::{
    common::Version,
    response::{Lyrics, Response, ResponseBody},
};

#[test]
fn get_lyrics() {
    let response = Response::from_json(include_str!("get-lyrics.json")).unwrap();
    let expected = Response::ok(Version::V1_16_1, ResponseBody::Lyrics(Lyrics::default()));
    assert_eq!(response, expected);
}
