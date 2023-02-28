// Example proxy using subsonic-types
// Check main function for default configuration

use std::net::SocketAddr;

use async_trait::async_trait;
use axum::{body::Bytes, Server};
use subsonic::SubsonicService;
use subsonic_types::{
    request::{system, Request, SubsonicRequest},
    response::Response,
};

mod subsonic {
    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::{
        extract::{FromRequestParts, State},
        response::IntoResponse,
        routing::get,
        Router,
    };
    use bytes::Bytes;
    use hyper::StatusCode;
    use subsonic_types::{
        request::{
            annotation, bookmark, browsing, chat, jukebox, lists, playlists, podcast, radio,
            retrieval, scan, search, sharing, system, user, Request, SubsonicRequest,
        },
        response::Response,
    };

    pub type Result<T, E = Error> = std::result::Result<T, E>;

    #[derive(Debug)]
    pub struct Error;

    impl IntoResponse for Error {
        fn into_response(self) -> axum::response::Response {
            axum::response::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(axum::body::boxed(axum::body::Empty::default()))
                .unwrap()
        }
    }

    #[async_trait]
    #[allow(unused_variables)]
    pub trait SubsonicService: Send + Sync + 'static {
        // Annotation
        async fn star(&self, request: Request<annotation::Star>) -> Result<Response> {
            Err(Error)
        }
        async fn unstar(&self, request: Request<annotation::Unstar>) -> Result<Response> {
            Err(Error)
        }
        async fn set_rating(&self, request: Request<annotation::SetRating>) -> Result<Response> {
            Err(Error)
        }
        async fn scrobble(&self, request: Request<annotation::Scrobble>) -> Result<Response> {
            Err(Error)
        }

        // Bookmarks
        async fn get_bookmarks(
            &self,
            request: Request<bookmark::GetBookmarks>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn create_bookmark(
            &self,
            request: Request<bookmark::CreateBookmark>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn delete_bookmark(
            &self,
            request: Request<bookmark::DeleteBookmark>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_play_queue(
            &self,
            request: Request<bookmark::GetPlayQueue>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn save_play_queue(
            &self,
            request: Request<bookmark::SavePlayQueue>,
        ) -> Result<Response> {
            Err(Error)
        }

        // Browsing
        async fn get_music_folders(
            &self,
            request: Request<browsing::GetMusicFolders>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_indexes(&self, request: Request<browsing::GetIndexes>) -> Result<Response> {
            Err(Error)
        }
        async fn get_music_directory(
            &self,
            request: Request<browsing::GetMusicDirectory>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_genres(&self, request: Request<browsing::GetGenres>) -> Result<Response> {
            Err(Error)
        }
        async fn get_artists(&self, request: Request<browsing::GetArtists>) -> Result<Response> {
            Err(Error)
        }
        async fn get_artist(&self, request: Request<browsing::GetArtist>) -> Result<Response> {
            Err(Error)
        }
        async fn get_album(&self, request: Request<browsing::GetAlbum>) -> Result<Response> {
            Err(Error)
        }
        async fn get_song(&self, request: Request<browsing::GetSong>) -> Result<Response> {
            Err(Error)
        }
        async fn get_videos(&self, request: Request<browsing::GetVideos>) -> Result<Response> {
            Err(Error)
        }
        async fn get_video_info(
            &self,
            request: Request<browsing::GetVideoInfo>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_artist_info(
            &self,
            request: Request<browsing::GetArtistInfo>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_artist_info2(
            &self,
            request: Request<browsing::GetArtistInfo2>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_album_info(
            &self,
            request: Request<browsing::GetAlbumInfo>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_album_info2(
            &self,
            request: Request<browsing::GetAlbumInfo2>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_similar_songs(
            &self,
            request: Request<browsing::GetSimilarSongs>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_similar_songs2(
            &self,
            request: Request<browsing::GetSimilarSongs2>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_top_songs(&self, request: Request<browsing::GetTopSongs>) -> Result<Response> {
            Err(Error)
        }

        // Chat
        async fn get_chat_messages(
            &self,
            request: Request<chat::GetChatMessages>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn add_chat_message(
            &self,
            request: Request<chat::AddChatMessage>,
        ) -> Result<Response> {
            Err(Error)
        }

        // Jukebox
        async fn jukebox_control(
            &self,
            request: Request<jukebox::JukeboxControl>,
        ) -> Result<Response> {
            Err(Error)
        }

        // Lists
        async fn get_album_list(&self, request: Request<lists::GetAlbumList>) -> Result<Response> {
            Err(Error)
        }
        async fn get_album_list2(
            &self,
            request: Request<lists::GetAlbumList2>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_random_songs(
            &self,
            request: Request<lists::GetRandomSongs>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_songs_by_genre(
            &self,
            request: Request<lists::GetSongsByGenre>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_now_playing(
            &self,
            request: Request<lists::GetNowPlaying>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_starred(&self, request: Request<lists::GetStarred>) -> Result<Response> {
            Err(Error)
        }
        async fn get_starred2(&self, request: Request<lists::GetStarred2>) -> Result<Response> {
            Err(Error)
        }

        // Playlists
        async fn get_playlists(
            &self,
            request: Request<playlists::GetPlaylists>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn get_playlist(&self, request: Request<playlists::GetPlaylist>) -> Result<Response> {
            Err(Error)
        }
        async fn create_playlist(
            &self,
            request: Request<playlists::CreatePlaylist>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn update_playlist(
            &self,
            request: Request<playlists::UpdatePlaylist>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn delete_playlist(
            &self,
            request: Request<playlists::DeletePlaylist>,
        ) -> Result<Response> {
            Err(Error)
        }

        // Podcasts
        async fn get_podcasts(&self, request: Request<podcast::GetPodcasts>) -> Result<Response> {
            Err(Error)
        }
        async fn get_newest_podcasts(
            &self,
            request: Request<podcast::GetNewestPodcasts>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn refresh_podcasts(
            &self,
            request: Request<podcast::RefreshPodcasts>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn create_podcast_channel(
            &self,
            request: Request<podcast::CreatePodcastChannel>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn delete_podcast_channel(
            &self,
            request: Request<podcast::DeletePodcastChannel>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn delete_podcast_episode(
            &self,
            request: Request<podcast::DeletePodcastEpisode>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn download_podcast_episode(
            &self,
            request: Request<podcast::DownloadPodcastEpisode>,
        ) -> Result<Response> {
            Err(Error)
        }

        // Radio
        async fn get_internet_radio_stations(
            &self,
            request: Request<radio::GetInternetRadioStations>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn create_internet_radio_station(
            &self,
            request: Request<radio::CreateInternetRadioStation>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn update_internet_radio_station(
            &self,
            request: Request<radio::UpdateInternetRadioStation>,
        ) -> Result<Response> {
            Err(Error)
        }
        async fn delete_internet_radio_station(
            &self,
            request: Request<radio::DeleteInternetRadioStation>,
        ) -> Result<Response> {
            Err(Error)
        }

        // Retrieval
        async fn stream(&self, request: Request<retrieval::Stream>) -> Result<Bytes> {
            Err(Error)
        }
        async fn download(&self, request: Request<retrieval::Download>) -> Result<Bytes> {
            Err(Error)
        }
        async fn hls(&self, request: Request<retrieval::Hls>) -> Result<Response> {
            Err(Error)
        }
        async fn get_captions(&self, request: Request<retrieval::GetCaptions>) -> Result<Response> {
            Err(Error)
        }
        async fn get_cover_art(&self, request: Request<retrieval::GetCoverArt>) -> Result<Bytes> {
            Err(Error)
        }
        async fn get_lyrics(&self, request: Request<retrieval::GetLyrics>) -> Result<Response> {
            Err(Error)
        }
        async fn get_avatar(&self, request: Request<retrieval::GetAvatar>) -> Result<Bytes> {
            Err(Error)
        }

        // Scan
        async fn get_scan_status(&self, request: Request<scan::GetScanStatus>) -> Result<Response> {
            Err(Error)
        }
        async fn start_scan(&self, request: Request<scan::StartScan>) -> Result<Response> {
            Err(Error)
        }

        // Search
        async fn search(&self, request: Request<search::Search>) -> Result<Response> {
            Err(Error)
        }
        async fn search2(&self, request: Request<search::Search2>) -> Result<Response> {
            Err(Error)
        }
        async fn search3(&self, request: Request<search::Search3>) -> Result<Response> {
            Err(Error)
        }

        // Sharing
        async fn get_shares(&self, request: Request<sharing::GetShares>) -> Result<Response> {
            Err(Error)
        }
        async fn create_share(&self, request: Request<sharing::CreateShare>) -> Result<Response> {
            Err(Error)
        }
        async fn update_share(&self, request: Request<sharing::UpdateShare>) -> Result<Response> {
            Err(Error)
        }
        async fn delete_share(&self, request: Request<sharing::DeleteShare>) -> Result<Response> {
            Err(Error)
        }

        // System
        async fn ping(&self, request: Request<system::Ping>) -> Result<Response> {
            Err(Error)
        }
        async fn get_license(&self, request: Request<system::GetLicense>) -> Result<Response> {
            Err(Error)
        }

        // User
        async fn get_user(&self, request: Request<user::GetUser>) -> Result<Response> {
            Err(Error)
        }
        async fn get_users(&self, request: Request<user::GetUsers>) -> Result<Response> {
            Err(Error)
        }
        async fn create_user(&self, request: Request<user::CreateUser>) -> Result<Response> {
            Err(Error)
        }
        async fn update_user(&self, request: Request<user::UpdateUser>) -> Result<Response> {
            Err(Error)
        }
        async fn delete_user(&self, request: Request<user::DeleteUser>) -> Result<Response> {
            Err(Error)
        }
        async fn change_password(
            &self,
            request: Request<user::ChangePassword>,
        ) -> Result<Response> {
            Err(Error)
        }
    }

    pub fn router(service: impl SubsonicService) -> Router {
        macro_rules! make_router {
            ($($req:path => $handler:ident),*) => {
                Router::new()
                $(
                    .route($req, get($handler))
                    .route(&format!("{}.view", $req), get($handler))
                )*
            }
        }

        let service = Arc::new(service);
        make_router! {
            // Annotation
            annotation::Star::PATH => star,
            annotation::Unstar::PATH => unstar,
            annotation::SetRating::PATH => set_rating,
            annotation::Scrobble::PATH => scrobble,
            // Bookmarks
            bookmark::GetBookmarks::PATH => get_bookmarks,
            bookmark::CreateBookmark::PATH => create_bookmark,
            bookmark::DeleteBookmark::PATH => delete_bookmark,
            bookmark::GetPlayQueue::PATH => get_play_queue,
            bookmark::SavePlayQueue::PATH => save_play_queue,
            // Browsing
            browsing::GetMusicFolders::PATH => get_music_folders,
            browsing::GetIndexes::PATH => get_indexes,
            browsing::GetMusicDirectory::PATH => get_music_directory,
            browsing::GetGenres::PATH => get_genres,
            browsing::GetArtists::PATH => get_artists,
            browsing::GetArtist::PATH => get_artist,
            browsing::GetAlbum::PATH => get_album,
            browsing::GetSong::PATH => get_song,
            browsing::GetVideos::PATH => get_videos,
            browsing::GetVideoInfo::PATH => get_video_info,
            browsing::GetArtistInfo::PATH => get_artist_info,
            browsing::GetArtistInfo2::PATH => get_artist_info2,
            browsing::GetAlbumInfo::PATH => get_album_info,
            browsing::GetAlbumInfo2::PATH => get_album_info2,
            browsing::GetSimilarSongs::PATH => get_similar_songs,
            browsing::GetSimilarSongs2::PATH => get_similar_songs2,
            browsing::GetTopSongs::PATH => get_top_songs,
            // Chat
            chat::GetChatMessages::PATH => get_chat_messages,
            chat::AddChatMessage::PATH => add_chat_message,
            // Jukebox
            jukebox::JukeboxControl::PATH => jukebox_control,
            // Lists
            lists::GetAlbumList::PATH => get_album_list,
            lists::GetAlbumList2::PATH => get_album_list2,
            lists::GetRandomSongs::PATH => get_random_songs,
            lists::GetSongsByGenre::PATH => get_songs_by_genre,
            lists::GetNowPlaying::PATH => get_now_playing,
            lists::GetStarred::PATH => get_starred,
            lists::GetStarred2::PATH => get_starred2,
            // Playlists
            playlists::GetPlaylists::PATH => get_playlists,
            playlists::GetPlaylist::PATH => get_playlist,
            playlists::CreatePlaylist::PATH => create_playlist,
            playlists::UpdatePlaylist::PATH => update_playlist,
            playlists::DeletePlaylist::PATH => delete_playlist,
            // Podcasts
            podcast::GetPodcasts::PATH => get_podcasts,
            podcast::GetNewestPodcasts::PATH => get_newest_podcasts,
            podcast::RefreshPodcasts::PATH => refresh_podcasts,
            podcast::CreatePodcastChannel::PATH => create_podcast_channel,
            podcast::DeletePodcastChannel::PATH => delete_podcast_channel,
            podcast::DeletePodcastEpisode::PATH => delete_podcast_episode,
            podcast::DownloadPodcastEpisode::PATH => download_podcast_episode,
            // Radio
            radio::GetInternetRadioStations::PATH => get_internet_radio_stations,
            radio::CreateInternetRadioStation::PATH => create_internet_radio_station,
            radio::UpdateInternetRadioStation::PATH => update_internet_radio_station,
            radio::DeleteInternetRadioStation::PATH => delete_internet_radio_station,
            // Retrieval
            retrieval::Stream::PATH => stream,
            retrieval::Download::PATH => download,
            retrieval::Hls::PATH => hls,
            retrieval::GetCaptions::PATH => get_captions,
            retrieval::GetCoverArt::PATH => get_cover_art,
            retrieval::GetLyrics::PATH => get_lyrics,
            retrieval::GetAvatar::PATH => get_avatar,
            // Scan
            scan::GetScanStatus::PATH => get_scan_status,
            scan::StartScan::PATH => start_scan,
            // Search
            search::Search::PATH => search,
            search::Search2::PATH => search2,
            search::Search3::PATH => search3,
            // Sharing
            sharing::GetShares::PATH => get_shares,
            sharing::CreateShare::PATH => create_share,
            sharing::UpdateShare::PATH => update_share,
            sharing::DeleteShare::PATH => delete_share,
            // System
            system::Ping::PATH => ping,
            system::GetLicense::PATH => get_license,
            // User
            user::GetUser::PATH => get_user,
            user::GetUsers::PATH => get_users,
            user::CreateUser::PATH => create_user,
            user::UpdateUser::PATH => update_user,
            user::DeleteUser::PATH => delete_user,
            user::ChangePassword::PATH => change_password
        }
        .with_state(service)
    }

    struct FromRequest<R>(Request<R>)
    where
        R: SubsonicRequest;

    #[async_trait]
    impl<S, R> FromRequestParts<S> for FromRequest<R>
    where
        R: SubsonicRequest,
    {
        type Rejection = Error;

        async fn from_request_parts(
            parts: &mut axum::http::request::Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            println!("uri: {}", parts.uri);
            match parts.uri.query() {
                Some(query) => Request::<R>::from_query(query)
                    .map(FromRequest)
                    .map_err(|e| {
                        eprintln!("failed to parse request: {}", e);
                        Error
                    }),
                None => {
                    eprintln!("failed to parse request");
                    Err(Error)
                }
            }
        }
    }

    macro_rules! declare_handlers {
        ($(($name:ident $t:path)),*) => {
            $(
                async fn $name(
                    service: State<Arc<dyn SubsonicService>>,
                    request: FromRequest<$t>,
                ) -> axum::response::Response {
                    println!("yhea");
                    let request = request.0.clone();
                    let response = match service.$name(request.clone()).await {
                        Ok(response) => response,
                        Err(err) => return err.into_response(),
                    };
                    let response = match request.format.as_ref().map(|s| s.as_str()) {
                        Some("json") => response.to_json().unwrap(),
                        _ => response.to_xml().unwrap()
                    };
                    let response = axum::response::Response::builder()
                        .status(StatusCode::OK)
                        .body(axum::body::boxed(axum::body::Body::from(Bytes::from(response.into_bytes()))))
                        .unwrap();
                    response
                }
            )*
        }
    }

    macro_rules! declare_handlers_binary {
        ($(($name:ident $t:path)),*) => {
            $(
                async fn $name(
                    service: State<Arc<dyn SubsonicService>>,
                    request: FromRequest<$t>,
                ) -> axum::response::Response {
                    let request = request.0.clone();
                    let response = match service.$name(request.clone()).await {
                        Ok(response) => response,
                        Err(err) => return err.into_response(),
                    };
                    let response = axum::response::Response::builder()
                        .status(StatusCode::OK)
                        .body(axum::body::boxed(axum::body::Body::from(response)))
                        .unwrap();
                    response
                }
            )*
        }
    }

    declare_handlers!(
        // Annotation
        (star annotation::Star),
        (unstar annotation::Unstar),
        (set_rating annotation::SetRating),
        (scrobble annotation::Scrobble),

        // Bookmarks
        (get_bookmarks bookmark::GetBookmarks),
        (create_bookmark bookmark::CreateBookmark),
        (delete_bookmark bookmark::DeleteBookmark),
        (get_play_queue bookmark::GetPlayQueue),
        (save_play_queue bookmark::SavePlayQueue),

        // Browsing
        (get_music_folders browsing::GetMusicFolders),
        (get_indexes browsing::GetIndexes),
        (get_music_directory browsing::GetMusicDirectory),
        (get_genres browsing::GetGenres),
        (get_artists browsing::GetArtists),
        (get_artist browsing::GetArtist),
        (get_album browsing::GetAlbum),
        (get_song browsing::GetSong),
        (get_videos browsing::GetVideos),
        (get_video_info browsing::GetVideoInfo),
        (get_artist_info browsing::GetArtistInfo),
        (get_artist_info2 browsing::GetArtistInfo2),
        (get_album_info browsing::GetAlbumInfo),
        (get_album_info2 browsing::GetAlbumInfo2),
        (get_similar_songs browsing::GetSimilarSongs),
        (get_similar_songs2 browsing::GetSimilarSongs2),
        (get_top_songs browsing::GetTopSongs),

        // Chat
        (get_chat_messages chat::GetChatMessages),
        (add_chat_message chat::AddChatMessage),

        // Jukebox
        (jukebox_control jukebox::JukeboxControl),

        // Lists
        (get_album_list lists::GetAlbumList),
        (get_album_list2 lists::GetAlbumList2),
        (get_random_songs lists::GetRandomSongs),
        (get_songs_by_genre lists::GetSongsByGenre),
        (get_now_playing lists::GetNowPlaying),
        (get_starred lists::GetStarred),
        (get_starred2 lists::GetStarred2),

        // Playlists
        (get_playlists playlists::GetPlaylists),
        (get_playlist playlists::GetPlaylist),
        (create_playlist playlists::CreatePlaylist),
        (update_playlist playlists::UpdatePlaylist),
        (delete_playlist playlists::DeletePlaylist),

        // Podcasts
        (get_podcasts podcast::GetPodcasts),
        (get_newest_podcasts podcast::GetNewestPodcasts),
        (refresh_podcasts podcast::RefreshPodcasts),
        (create_podcast_channel podcast::CreatePodcastChannel),
        (delete_podcast_channel podcast::DeletePodcastChannel),
        (delete_podcast_episode podcast::DeletePodcastEpisode),
        (download_podcast_episode podcast::DownloadPodcastEpisode),

        // Radio
        (get_internet_radio_stations radio::GetInternetRadioStations),
        (create_internet_radio_station radio::CreateInternetRadioStation),
        (update_internet_radio_station radio::UpdateInternetRadioStation),
        (delete_internet_radio_station radio::DeleteInternetRadioStation),

        // Retrieval
        (hls retrieval::Hls),
        (get_captions retrieval::GetCaptions),
        (get_lyrics retrieval::GetLyrics),

        // Scan
        (get_scan_status scan::GetScanStatus),
        (start_scan scan::StartScan),

        // Search
        (search search::Search),
        (search2 search::Search2),
        (search3 search::Search3),

        // Sharing
        (get_shares sharing::GetShares),
        (create_share sharing::CreateShare),
        (update_share sharing::UpdateShare),
        (delete_share sharing::DeleteShare),

        // System
        (ping system::Ping),
        (get_license system::GetLicense),

        // User
        (get_user user::GetUser),
        (get_users user::GetUsers),
        (create_user user::CreateUser),
        (update_user user::UpdateUser),
        (delete_user user::DeleteUser),
        (change_password user::ChangePassword)
    );

    declare_handlers_binary! {
        // Retrieval
        (stream retrieval::Stream),
        (download retrieval::Download),
        (get_cover_art retrieval::GetCoverArt),
        (get_avatar retrieval::GetAvatar)
    }
}

struct Service {
    base_url: String,
}

impl Service {
    async fn forward<R>(&self, request: &Request<R>) -> subsonic::Result<Response>
    where
        R: SubsonicRequest,
    {
        println!("forwarding request: {:#?}", request);
        let url = format!("{}{}?{}", self.base_url, R::PATH, request.to_query());
        let response = reqwest::get(&url).await.unwrap();
        let is_success = response.status().is_success();
        let response_body = response.text().await.unwrap();
        println!("response body: {}", response_body);
        if !is_success {
            return Ok(Response::failed(
                request.version,
                subsonic_types::response::Error::with_message(
                    subsonic_types::response::ErrorCode::Generic,
                    response_body,
                ),
            ));
        }
        let response = match request.format.as_ref().map(|s| s.as_str()) {
            Some("json") => match Response::from_json(&response_body) {
                Ok(response) => response,
                Err(err) => {
                    println!("error: {}", err);
                    println!("response: {}", response_body);
                    panic!("{}", err);
                }
            },
            _ => Response::from_xml(&response_body).unwrap(),
        };
        Ok(response)
    }

    async fn forward_binary<R>(&self, request: &Request<R>) -> subsonic::Result<Bytes>
    where
        R: SubsonicRequest,
    {
        println!("forwarding request: {:#?}", request);
        let url = format!("{}{}?{}", self.base_url, R::PATH, request.to_query());
        let response = reqwest::get(&url).await.unwrap();
        let is_success = response.status().is_success();
        let response_body = response.bytes().await.unwrap();
        if !is_success {
            println!("Returining error");
            return Err(subsonic::Error);
        }
        println!("Returning {} bytes", response_body.len());
        Ok(response_body)
    }
}

use subsonic::Result;
use subsonic_types::request::*;

#[async_trait]
impl SubsonicService for Service {
    // Annotation
    async fn star(&self, request: Request<annotation::Star>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn unstar(&self, request: Request<annotation::Unstar>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn set_rating(&self, request: Request<annotation::SetRating>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn scrobble(&self, request: Request<annotation::Scrobble>) -> Result<Response> {
        self.forward(&request).await
    }

    // Bookmarks
    async fn get_bookmarks(&self, request: Request<bookmark::GetBookmarks>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn create_bookmark(
        &self,
        request: Request<bookmark::CreateBookmark>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn delete_bookmark(
        &self,
        request: Request<bookmark::DeleteBookmark>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_play_queue(&self, request: Request<bookmark::GetPlayQueue>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn save_play_queue(&self, request: Request<bookmark::SavePlayQueue>) -> Result<Response> {
        self.forward(&request).await
    }

    // Browsing
    async fn get_music_folders(
        &self,
        request: Request<browsing::GetMusicFolders>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_indexes(&self, request: Request<browsing::GetIndexes>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_music_directory(
        &self,
        request: Request<browsing::GetMusicDirectory>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_genres(&self, request: Request<browsing::GetGenres>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_artists(&self, request: Request<browsing::GetArtists>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_artist(&self, request: Request<browsing::GetArtist>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_album(&self, request: Request<browsing::GetAlbum>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_song(&self, request: Request<browsing::GetSong>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_videos(&self, request: Request<browsing::GetVideos>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_video_info(&self, request: Request<browsing::GetVideoInfo>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_artist_info(&self, request: Request<browsing::GetArtistInfo>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_artist_info2(
        &self,
        request: Request<browsing::GetArtistInfo2>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_album_info(&self, request: Request<browsing::GetAlbumInfo>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_album_info2(&self, request: Request<browsing::GetAlbumInfo2>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_similar_songs(
        &self,
        request: Request<browsing::GetSimilarSongs>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_similar_songs2(
        &self,
        request: Request<browsing::GetSimilarSongs2>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_top_songs(&self, request: Request<browsing::GetTopSongs>) -> Result<Response> {
        self.forward(&request).await
    }

    // Chat
    async fn get_chat_messages(&self, request: Request<chat::GetChatMessages>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn add_chat_message(&self, request: Request<chat::AddChatMessage>) -> Result<Response> {
        self.forward(&request).await
    }

    // Jukebox
    async fn jukebox_control(&self, request: Request<jukebox::JukeboxControl>) -> Result<Response> {
        self.forward(&request).await
    }

    // Lists
    async fn get_album_list(&self, request: Request<lists::GetAlbumList>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_album_list2(&self, request: Request<lists::GetAlbumList2>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_random_songs(&self, request: Request<lists::GetRandomSongs>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_songs_by_genre(
        &self,
        request: Request<lists::GetSongsByGenre>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_now_playing(&self, request: Request<lists::GetNowPlaying>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_starred(&self, request: Request<lists::GetStarred>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_starred2(&self, request: Request<lists::GetStarred2>) -> Result<Response> {
        self.forward(&request).await
    }

    // Playlists
    async fn get_playlists(&self, request: Request<playlists::GetPlaylists>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_playlist(&self, request: Request<playlists::GetPlaylist>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn create_playlist(
        &self,
        request: Request<playlists::CreatePlaylist>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn update_playlist(
        &self,
        request: Request<playlists::UpdatePlaylist>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn delete_playlist(
        &self,
        request: Request<playlists::DeletePlaylist>,
    ) -> Result<Response> {
        self.forward(&request).await
    }

    // Podcasts
    async fn get_podcasts(&self, request: Request<podcast::GetPodcasts>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_newest_podcasts(
        &self,
        request: Request<podcast::GetNewestPodcasts>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn refresh_podcasts(
        &self,
        request: Request<podcast::RefreshPodcasts>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn create_podcast_channel(
        &self,
        request: Request<podcast::CreatePodcastChannel>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn delete_podcast_channel(
        &self,
        request: Request<podcast::DeletePodcastChannel>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn delete_podcast_episode(
        &self,
        request: Request<podcast::DeletePodcastEpisode>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn download_podcast_episode(
        &self,
        request: Request<podcast::DownloadPodcastEpisode>,
    ) -> Result<Response> {
        self.forward(&request).await
    }

    // Radio
    async fn get_internet_radio_stations(
        &self,
        request: Request<radio::GetInternetRadioStations>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn create_internet_radio_station(
        &self,
        request: Request<radio::CreateInternetRadioStation>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn update_internet_radio_station(
        &self,
        request: Request<radio::UpdateInternetRadioStation>,
    ) -> Result<Response> {
        self.forward(&request).await
    }
    async fn delete_internet_radio_station(
        &self,
        request: Request<radio::DeleteInternetRadioStation>,
    ) -> Result<Response> {
        self.forward(&request).await
    }

    // Retrieval
    async fn stream(&self, request: Request<retrieval::Stream>) -> Result<Bytes> {
        self.forward_binary(&request).await
    }
    async fn download(&self, request: Request<retrieval::Download>) -> Result<Bytes> {
        self.forward_binary(&request).await
    }
    async fn hls(&self, request: Request<retrieval::Hls>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_captions(&self, request: Request<retrieval::GetCaptions>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_cover_art(&self, request: Request<retrieval::GetCoverArt>) -> Result<Bytes> {
        self.forward_binary(&request).await
    }
    async fn get_lyrics(&self, request: Request<retrieval::GetLyrics>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_avatar(&self, request: Request<retrieval::GetAvatar>) -> Result<Bytes> {
        self.forward_binary(&request).await
    }

    // Scan
    async fn get_scan_status(&self, request: Request<scan::GetScanStatus>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn start_scan(&self, request: Request<scan::StartScan>) -> Result<Response> {
        self.forward(&request).await
    }

    // Search
    async fn search(&self, request: Request<search::Search>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn search2(&self, request: Request<search::Search2>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn search3(&self, request: Request<search::Search3>) -> Result<Response> {
        self.forward(&request).await
    }

    // Sharing
    async fn get_shares(&self, request: Request<sharing::GetShares>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn create_share(&self, request: Request<sharing::CreateShare>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn update_share(&self, request: Request<sharing::UpdateShare>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn delete_share(&self, request: Request<sharing::DeleteShare>) -> Result<Response> {
        self.forward(&request).await
    }

    // System
    async fn ping(&self, request: Request<system::Ping>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_license(&self, request: Request<system::GetLicense>) -> Result<Response> {
        self.forward(&request).await
    }

    // User
    async fn get_user(&self, request: Request<user::GetUser>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn get_users(&self, request: Request<user::GetUsers>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn create_user(&self, request: Request<user::CreateUser>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn update_user(&self, request: Request<user::UpdateUser>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn delete_user(&self, request: Request<user::DeleteUser>) -> Result<Response> {
        self.forward(&request).await
    }
    async fn change_password(&self, request: Request<user::ChangePassword>) -> Result<Response> {
        self.forward(&request).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let router = subsonic::router(Service {
        // URL of the Subsonic server
        base_url: "http://localhost:4533".to_string(),
    });
    let server = Server::bind(&addr).serve(router.into_make_service());
    server.await?;
    Ok(())
}
