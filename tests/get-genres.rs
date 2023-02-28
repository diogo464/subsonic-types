use subsonic_types::{
    common::Version,
    response::{Genres, Response, ResponseBody},
};

#[test]
fn get_genres() {
    let response = Response::from_json(include_str!("get-genres.json")).unwrap();
    let expected = Response::ok(Version::V1_16_1, ResponseBody::Genres(Genres::default()));
    assert_eq!(response, expected);
}
