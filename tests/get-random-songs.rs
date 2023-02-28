use subsonic_types::{
    common::{MediaType, Version},
    response::{Child, Response, ResponseBody, Songs},
};

/*
{
    "subsonic-response": {
        "status": "ok",
        "version": "1.16.1",
        "type": "navidrome",
        "serverVersion": "0.49.3 (8b93962f)",
        "randomSongs": {
            "song": [
                {
                    "id": "e1bf816902f0deea6d7b1b3a088fa55c",
                    "isDir": false,
                    "title": "Crystal Clear",
                    "created": "2023-02-19T14:06:49.481619267Z",
                    "albumId": "042ad3a563dbe70861d1e7f1eb35e7fa",
                    "artistId": "57fd6b271014d8b0a2fcb9124b143b53",
                    "type": "music",
                    "isVideo": false
                }
            ]
        }
    }
}
*/

#[test]
fn get_random_songs() {
    let response = Response::from_json(include_str!("get-random-songs.json")).unwrap();
    let expected = Response::ok(
        Version::V1_16_1,
        ResponseBody::RandomSongs(Songs {
            song: vec![Child {
                id: "e1bf816902f0deea6d7b1b3a088fa55c".into(),
                is_dir: false,
                title: "Crystal Clear".into(),
                created: Some("2023-02-19T14:06:49.481619267Z".parse().unwrap()),
                album_id: Some("042ad3a563dbe70861d1e7f1eb35e7fa".into()),
                artist_id: Some("57fd6b271014d8b0a2fcb9124b143b53".into()),
                media_type: Some(MediaType::Music),
                is_video: Some(false),
                ..Default::default()
            }],
        }),
    );
    assert_eq!(response, expected);
}
