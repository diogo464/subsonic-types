use subsonic_types::{
    common::{Seconds, Version},
    response::{Playlist, Playlists, Response, ResponseBody},
};

#[test]
fn get_playlists() {
    let response = Response::from_json(include_str!("get-playlists.json")).unwrap();
    let expected = Response::ok(
        Version::V1_16_1,
        ResponseBody::Playlists(Playlists {
            playlist: vec![
                Playlist {
                    id: "d830c0ff-74e5-4e4b-9b7a-6be3f7fc2697".into(),
                    name: "2023-02-27".into(),
                    comment: None,
                    owner: Some("admin".into()),
                    public: Some(false),
                    song_count: 2,
                    duration: Seconds::new(589),
                    created: "2023-02-27T15:59:10.991316377Z".parse().unwrap(),
                    changed: "2023-02-27T15:59:10Z".parse().unwrap(),
                    cover_art: Some("pl-d830c0ff-74e5-4e4b-9b7a-6be3f7fc2697_63fcd34e".into()),
                    allowed_user: vec![],
                },
                Playlist {
                    id: "2402328c-2c31-4475-ad3a-a698884edefb".into(),
                    name: "2023-02-27".into(),
                    comment: None,
                    owner: Some("admin".into()),
                    public: Some(false),
                    song_count: 2,
                    duration: Seconds::new(522),
                    created: "2023-02-27T16:00:43.404089728Z".parse().unwrap(),
                    changed: "2023-02-27T16:00:43Z".parse().unwrap(),
                    cover_art: Some("pl-2402328c-2c31-4475-ad3a-a698884edefb_63fcd3ab".into()),
                    allowed_user: vec![],
                },
            ],
        }),
    );
    assert_eq!(response, expected);
}
