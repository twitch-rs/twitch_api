//! Convenience functions for [HelixClient]
use crate::helix::{self, ClientRequestError, HelixClient};
use crate::types;
use twitch_oauth2::TwitchToken;

type ClientError<'a, C> = ClientRequestError<<C as crate::HttpClient<'a>>::Error>;

// TODO: Consider moving these into the specific modules where the request is defined. Preferably backed by a macro

impl<'client, C: crate::HttpClient<'client> + Sync> HelixClient<'client, C> {
    // /// Get [User](helix::users::User) from user login
    // pub async fn get_user_from_login<T>(
    //     &'client self,
    //     login: impl Into<&types::UserNameRef>,
    //     token: &T,
    // ) -> Result<Option<helix::users::User>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     self.req_get(
    //         helix::users::GetUsersRequest::logins(&[login.into()][..]),
    //         token,
    //     )
    //     .await
    //     .map(|response| response.first())
    // }

    /// Get [User](helix::users::User) from user id
    pub async fn get_user_from_id<T>(
        &'client self,
        id: impl Into<&types::UserIdRef>,
        token: &T,
    ) -> Result<Option<yoke::Yoke<helix::users::User<'static>, Vec<u8>>>, ClientError<'client, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(helix::users::GetUsersRequest::ids(&[id.into()][..]), token)
            .await
            .map(|response| response.first())
    }

    // /// Get multiple [User](helix::users::User)s from user ids.
    // pub async fn get_users_from_ids<T>(
    //     &'client self,
    //     ids: impl AsRef<[&types::UserIdRef]>,
    //     token: &T,
    // ) -> Result<Option<helix::users::User>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let ids = ids.as_ref();
    //     if ids.len() > 100 {
    //         return Err(ClientRequestError::Custom("too many IDs, max 100".into()));
    //     }
    //     self.req_get(helix::users::GetUsersRequest::ids(ids), token)
    //         .await
    //         .map(|response| response.first())
    // }

    // /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters login
    // pub async fn get_channel_from_login<T>(
    //     &'client self,
    //     login: impl Into<&types::UserNameRef>,
    //     token: &T,
    // ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     if let Some(user) = self.get_user_from_login(login.into(), token).await? {
    //         self.get_channel_from_id(&user.id, token).await
    //     } else {
    //         Ok(None)
    //     }
    // }

    // /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters id
    // pub async fn get_channel_from_id<T>(
    //     &'client self,
    //     id: impl Into<&types::UserIdRef>,
    //     token: &T,
    // ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let ids: &[_] = &[id.into()];
    //     self.req_get(
    //         helix::channels::GetChannelInformationRequest::broadcaster_ids(ids),
    //         token,
    //     )
    //     .await
    //     .map(|response| response.first())
    // }

    // /// Get multiple [ChannelInformation](helix::channels::ChannelInformation) from broadcasters ids
    // pub async fn get_channels_from_ids<'b, T>(
    //     &'client self,
    //     ids: impl AsRef<[&types::UserIdRef]>,
    //     token: &T,
    // ) -> Result<Vec<helix::channels::ChannelInformation>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let ids = ids.as_ref();
    //     if ids.len() > 100 {
    //         return Err(ClientRequestError::Custom("too many IDs, max 100".into()));
    //     }
    //     self.req_get(
    //         helix::channels::GetChannelInformationRequest::broadcaster_ids(ids),
    //         token,
    //     )
    //     .await
    //     .map(|response| response.data)
    // }

    // /// Get chatters in a stream [Chatter][helix::chat::Chatter]
    // ///
    // /// `batch_size` sets the amount of chatters to retrieve per api call, max 1000, defaults to 100.
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let chatters: Vec<helix::chat::Chatter> = client.get_chatters("1234", "4321", 1000, &token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // #[cfg(feature = "unsupported")]
    // pub fn get_chatters<T>(
    //     &'client self,
    //     broadcaster_id: impl Into<&'client types::UserIdRef>,
    //     moderator_id: impl Into<&'client types::UserIdRef>,
    //     batch_size: impl Into<Option<usize>>,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<Item = Result<helix::chat::Chatter, ClientError<'client, C>>>
    //             + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     let req = helix::chat::GetChattersRequest {
    //         first: batch_size.into(),
    //         ..helix::chat::GetChattersRequest::new(broadcaster_id.into(), moderator_id.into())
    //     };

    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Search [Categories](helix::search::Category)
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let categories: Vec<helix::search::Category> = client.search_categories("Fortnite", &token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn search_categories<T>(
    //     &'client self,
    //     query: impl Into<&'client str>,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<Item = Result<helix::search::Category, ClientError<'client, C>>>
    //             + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     let req = helix::search::SearchCategoriesRequest::query(query.into()).first(100);
    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Search [Channels](helix::search::Channel) via channel name or description
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let channel: Vec<helix::search::Channel> = client.search_channels("twitchdev", false, &token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn search_channels<'b, T>(
    //     &'client self,
    //     query: impl Into<&'b str>,
    //     live_only: bool,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<Item = Result<helix::search::Channel, ClientError<'client, C>>>
    //             + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    //     'b: 'client,
    // {
    //     let req = helix::search::SearchChannelsRequest::query(query.into()).live_only(live_only);
    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Get information on a [follow relationship](helix::users::FollowRelationship)
    // ///
    // /// Can be used to see if X follows Y
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::{types, helix};
    // /// use futures::TryStreamExt;
    // ///
    // /// // Get the followers of channel "1234"
    // /// let followers: Vec<helix::users::FollowRelationship> = client.get_follow_relationships(Some("1234".into()), None, &token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn get_follow_relationships<'b, T>(
    //     &'client self,
    //     to_id: impl Into<Option<&'b types::UserIdRef>>,
    //     from_id: impl Into<Option<&'b types::UserIdRef>>,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<
    //                 Item = Result<helix::users::FollowRelationship, ClientError<'client, C>>,
    //             > + Send
    //             + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    //     'b: 'client,
    // {
    //     let mut req = helix::users::GetUsersFollowsRequest::empty();
    //     req.to_id = to_id.into().map(Cow::Borrowed);
    //     req.from_id = from_id.into().map(Cow::Borrowed);

    //     make_stream(req, token, self, |s| {
    //         std::collections::VecDeque::from(s.follow_relationships)
    //     })
    // }

    // /// Get authenticated users' followed [streams](helix::streams::Stream)
    // ///
    // /// Requires token with scope [`user:read:follows`](twitch_oauth2::Scope::UserReadFollows).
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let channels: Vec<helix::streams::Stream> = client.get_followed_streams(&token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn get_followed_streams<T>(
    //     &'client self,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<Item = Result<helix::streams::Stream, ClientError<'client, C>>>
    //             + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     use futures::StreamExt;

    //     let user_id = match token
    //         .user_id()
    //         .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
    //     {
    //         Ok(t) => t,
    //         Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
    //     };
    //     let req = helix::streams::GetFollowedStreamsRequest::user_id(user_id);
    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Get authenticated broadcasters' [subscribers](helix::subscriptions::BroadcasterSubscription)
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let subs: Vec<helix::subscriptions::BroadcasterSubscription> = client.get_broadcaster_subscriptions(&token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn get_broadcaster_subscriptions<T>(
    //     &'client self,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<
    //                 Item = Result<
    //                     helix::subscriptions::BroadcasterSubscription,
    //                     ClientError<'client, C>,
    //                 >,
    //             > + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     use futures::StreamExt;

    //     let user_id = match token
    //         .user_id()
    //         .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
    //     {
    //         Ok(t) => t,
    //         Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
    //     };
    //     let req = helix::subscriptions::GetBroadcasterSubscriptionsRequest::broadcaster_id(user_id);
    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Get all moderators in a channel [Get Moderators](helix::moderation::GetModeratorsRequest)
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let moderators: Vec<helix::moderation::Moderator> = client.get_moderators_in_channel_from_id("twitchdev", &token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn get_moderators_in_channel_from_id<'b: 'client, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<
    //                 Item = Result<helix::moderation::Moderator, ClientError<'client, C>>,
    //             > + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     let req = helix::moderation::GetModeratorsRequest::broadcaster_id(broadcaster_id);

    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Get all banned users in a channel [Get Banned Users](helix::moderation::GetBannedUsersRequest)
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let moderators: Vec<helix::moderation::BannedUser> = client.get_banned_users_in_channel_from_id("twitchdev", &token).try_collect().await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn get_banned_users_in_channel_from_id<'b: 'client, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<
    //                 Item = Result<helix::moderation::BannedUser, ClientError<'client, C>>,
    //             > + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     let req = helix::moderation::GetBannedUsersRequest::broadcaster_id(broadcaster_id);

    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Get a users, with login, follow count
    // pub async fn get_total_followers_from_login<'b, T>(
    //     &'client self,
    //     login: impl types::IntoCow<'b, types::UserNameRef> + 'b,
    //     token: &T,
    // ) -> Result<Option<i64>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     if let Some(user) = self.get_user_from_login(&*login.to_cow(), token).await? {
    //         self.get_total_followers_from_id(&user.id, token)
    //             .await
    //             .map(Some)
    //     } else {
    //         Ok(None)
    //     }
    // }

    // /// Get a users, with id, follow count
    // ///
    // /// # Notes
    // ///
    // /// This returns zero if the user doesn't exist
    // pub async fn get_total_followers_from_id<'b, T>(
    //     &'client self,
    //     to_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<i64, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let resp = self
    //         .req_get(
    //             helix::users::GetUsersFollowsRequest::followers(to_id),
    //             token,
    //         )
    //         .await?;

    //     Ok(resp.data.total)
    // }

    // /// Get games by ID. Can only be at max 100 ids.
    // pub async fn get_games_by_id<T>(
    //     &'client self,
    //     ids: impl AsRef<[&'client types::CategoryIdRef]>,
    //     token: &T,
    // ) -> Result<
    //     std::collections::HashMap<types::CategoryId, helix::games::Game>,
    //     ClientError<'client, C>,
    // >
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let ids = ids.as_ref();
    //     if ids.len() > 100 {
    //         return Err(ClientRequestError::Custom("too many IDs, max 100".into()));
    //     }

    //     let resp = self
    //         .req_get(helix::games::GetGamesRequest::ids(ids), token)
    //         .await?;

    //     Ok(resp
    //         .data
    //         .into_iter()
    //         .map(|g: helix::games::Game| (g.id.clone(), g))
    //         .collect())
    // }

    // /// Block a user
    // pub async fn block_user<'b, T>(
    //     &'client self,
    //     target_user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::users::BlockUser, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_put(
    //             helix::users::BlockUserRequest::block_user(target_user_id),
    //             helix::EmptyBody,
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // /// Unblock a user
    // pub async fn unblock_user<'b, T>(
    //     &'client self,
    //     target_user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::users::UnblockUser, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_delete(
    //             helix::users::UnblockUserRequest::unblock_user(target_user_id),
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // /// Ban a user
    // pub async fn ban_user<'b, T>(
    //     &'client self,
    //     target_user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     reason: impl Into<&'b str>,
    //     duration: impl Into<Option<u32>>,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::moderation::BanUser, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_post(
    //             helix::moderation::BanUserRequest::new(broadcaster_id, moderator_id),
    //             helix::moderation::BanUserBody::new(target_user_id, reason.into(), duration),
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // /// Unban a user
    // pub async fn unban_user<'b, T>(
    //     &'client self,
    //     target_user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::moderation::UnbanUserResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_delete(
    //             helix::moderation::UnbanUserRequest::new(
    //                 broadcaster_id,
    //                 moderator_id,
    //                 target_user_id,
    //             ),
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // // FIXME: Example should use https://github.com/twitch-rs/twitch_api/issues/162
    // /// Get all scheduled streams in a channel.
    // ///
    // /// # Notes
    // ///
    // /// Make sure to limit the data here using [`try_take_while`](futures::stream::TryStreamExt::try_take_while), otherwise this will never end on recurring scheduled streams.
    // ///
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // /// use futures::TryStreamExt;
    // ///
    // /// let schedule: Vec<helix::schedule::Segment> = client
    // ///     .get_channel_schedule("twitchdev", &token)
    // ///     .try_take_while(|s| {
    // ///         futures::future::ready(Ok(!s.start_time.as_str().starts_with("2021-10")))
    // ///     })
    // ///     .try_collect()
    // ///     .await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn get_channel_schedule<'b: 'client, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<Item = Result<helix::schedule::Segment, ClientError<'client, C>>>
    //             + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     let req = helix::schedule::GetChannelStreamScheduleRequest::broadcaster_id(broadcaster_id);

    //     make_stream(req, token, self, |broadcasts| broadcasts.segments.into())
    // }

    // /// Get all global emotes
    // pub async fn get_global_emotes<T>(
    //     &'client self,
    //     token: &T,
    // ) -> Result<Vec<helix::chat::GlobalEmote>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::chat::GetGlobalEmotesRequest::new();
    //     Ok(self.req_get(req, token).await?.data)
    // }

    // /// Get channel emotes in channel with user id
    // pub async fn get_channel_emotes_from_id<'b, T>(
    //     &'client self,
    //     user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<Vec<helix::chat::ChannelEmote>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::chat::GetChannelEmotesRequest::broadcaster_id(user_id);
    //     Ok(self.req_get(req, token).await?.data)
    // }

    // /// Get channel emotes in channel with user login
    // pub async fn get_channel_emotes_from_login<T>(
    //     &'client self,
    //     login: impl types::IntoCow<'client, types::UserNameRef> + 'client,
    //     token: &T,
    // ) -> Result<Option<Vec<helix::chat::ChannelEmote>>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     if let Some(user) = self
    //         .get_user_from_login(login.to_cow().as_ref(), token)
    //         .await?
    //     {
    //         self.get_channel_emotes_from_id(&user.id, token)
    //             .await
    //             .map(Some)
    //     } else {
    //         Ok(None)
    //     }
    // }

    // /// Get emotes in emote set
    // pub async fn get_emote_sets<T>(
    //     &'client self,
    //     emote_sets: impl AsRef<[&types::EmoteSetIdRef]>,
    //     token: &T,
    // ) -> Result<Vec<helix::chat::get_emote_sets::Emote>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let emote_sets = emote_sets.as_ref();
    //     let req = helix::chat::GetEmoteSetsRequest::emote_set_ids(emote_sets);
    //     Ok(self.req_get(req, token).await?.data)
    // }

    // /// Get a broadcaster's chat settings
    // pub async fn get_chat_settings<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl Into<Option<&'b types::UserIdRef>> + 'b,
    //     token: &T,
    // ) -> Result<helix::chat::ChatSettings, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let mut req = helix::chat::GetChatSettingsRequest::broadcaster_id(broadcaster_id);
    //     if let Some(moderator_id) = moderator_id.into() {
    //         req = req.moderator_id(moderator_id);
    //     }
    //     Ok(self.req_get(req, token).await?.data)
    // }

    // /// Send a chat announcement
    // pub async fn send_chat_announcement<'b, T, E>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     message: impl Into<&'b str>,
    //     color: impl std::convert::TryInto<helix::chat::AnnouncementColor, Error = E>,
    //     token: &T,
    // ) -> Result<helix::chat::SendChatAnnouncementResponse, ClientExtError<'client, C, E>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::chat::SendChatAnnouncementRequest::new(broadcaster_id, moderator_id);
    //     let body = helix::chat::SendChatAnnouncementBody::new(message.into(), color)?;
    //     Ok(self
    //         .req_post(req, body, token)
    //         .await
    //         .map_err(ClientExtError::ClientError)?
    //         .data)
    // }

    // /// Delete a specific chat message
    // pub async fn delete_chat_message<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     message_id: impl types::IntoCow<'b, types::MsgIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::moderation::DeleteChatMessagesResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::moderation::DeleteChatMessagesRequest::new(broadcaster_id, moderator_id)
    //         .message_id(message_id);
    //     Ok(self.req_delete(req, token).await?.data)
    // }

    // /// Delete all chat messages in a broadcasters chat room
    // pub async fn delete_all_chat_message<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::moderation::DeleteChatMessagesResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::moderation::DeleteChatMessagesRequest::new(broadcaster_id, moderator_id);
    //     Ok(self.req_delete(req, token).await?.data)
    // }

    // /// Start a raid
    // pub async fn start_a_raid<'b, T>(
    //     &'client self,
    //     from_broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     to_broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::raids::StartARaidResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::raids::StartARaidRequest::new(from_broadcaster_id, to_broadcaster_id);
    //     Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    // }

    // /// Cancel a raid
    // pub async fn cancel_a_raid<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::raids::CancelARaidResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::raids::CancelARaidRequest::broadcaster_id(broadcaster_id);
    //     Ok(self.req_delete(req, token).await?.data)
    // }

    // /// Get a users chat color
    // pub async fn get_user_chat_color<T>(
    //     &'client self,
    //     user_id: impl Into<&types::UserIdRef>,
    //     token: &T,
    // ) -> Result<Option<helix::chat::UserChatColor>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_get(
    //             helix::chat::GetUserChatColorRequest::user_ids(&[user_id.into()][..]),
    //             token,
    //         )
    //         .await?
    //         .first())
    // }

    // /// Get a users chat color
    // pub async fn update_user_chat_color<'b, T>(
    //     &'client self,
    //     user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     color: impl Into<types::NamedUserColor<'b>> + 'b,
    //     token: &T,
    // ) -> Result<helix::chat::UpdateUserChatColorResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::chat::UpdateUserChatColorRequest {
    //         user_id: user_id.to_cow(),
    //         color: color.into(),
    //     };

    //     Ok(self.req_put(req, helix::EmptyBody, token).await?.data)
    // }

    // /// Get multiple users chat colors
    // pub async fn get_users_chat_colors<T>(
    //     &'client self,
    //     user_ids: impl AsRef<[&types::UserIdRef]>,
    //     token: &T,
    // ) -> Result<Vec<helix::chat::UserChatColor>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let user_ids = user_ids.as_ref();
    //     if user_ids.len() > 100 {
    //         return Err(ClientRequestError::Custom("too many IDs, max 100".into()));
    //     }
    //     let req = helix::chat::GetUserChatColorRequest::user_ids(user_ids);

    //     Ok(self.req_get(req, token).await?.data)
    // }

    // /// Add a channel moderator
    // pub async fn add_channel_moderator<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::moderation::AddChannelModeratorResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::moderation::AddChannelModeratorRequest {
    //         broadcaster_id: broadcaster_id.to_cow(),
    //         moderator_id: moderator_id.to_cow(),
    //     };

    //     Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    // }

    // /// Remove a channel moderator
    // pub async fn remove_channel_moderator<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::moderation::RemoveChannelModeratorResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::moderation::RemoveChannelModeratorRequest {
    //         broadcaster_id: broadcaster_id.to_cow(),
    //         moderator_id: moderator_id.to_cow(),
    //     };

    //     Ok(self.req_delete(req, token).await?.data)
    // }

    // /// Get channel VIPs
    // pub fn get_vips_in_channel<'b: 'client, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<Item = Result<helix::channels::Vip, ClientError<'client, C>>>
    //             + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     let req = helix::channels::GetVipsRequest::broadcaster_id(broadcaster_id);

    //     make_stream(req, token, self, std::collections::VecDeque::from)
    // }

    // /// Add a channel vip
    // pub async fn add_channel_vip<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::channels::AddChannelVipResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::channels::AddChannelVipRequest {
    //         broadcaster_id: broadcaster_id.to_cow(),
    //         user_id: user_id.to_cow(),
    //     };

    //     Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    // }

    // /// Remove a channel vip
    // pub async fn remove_channel_vip<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::channels::RemoveChannelVipResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = helix::channels::RemoveChannelVipRequest {
    //         broadcaster_id: broadcaster_id.to_cow(),
    //         user_id: user_id.to_cow(),
    //     };

    //     Ok(self.req_delete(req, token).await?.data)
    // }

    // /// Send a whisper
    // pub async fn send_whisper<'b, T>(
    //     &'client self,
    //     from: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     to: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     message: impl Into<&'b str>,
    //     token: &T,
    // ) -> Result<helix::whispers::SendWhisperResponse, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_post(
    //             helix::whispers::SendWhisperRequest::new(from, to),
    //             helix::whispers::SendWhisperBody::new(message.into()),
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // /// Get all custom rewards
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // ///
    // /// let rewards: Vec<helix::points::CustomReward> = client
    // ///     .get_all_custom_rewards("1234", true, &token)
    // ///     .await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub async fn get_all_custom_rewards<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     only_managable_rewards: bool,
    //     token: &T,
    // ) -> Result<Vec<helix::points::CustomReward>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     self.get_custom_rewards(broadcaster_id, only_managable_rewards, &[], token)
    //         .await
    // }

    // /// Get specific custom rewards, see [`get_all_custom_rewards`](HelixClient::get_all_custom_rewards) to get all rewards
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::helix;
    // ///
    // /// let rewards: Vec<helix::points::CustomReward> = client
    // ///     .get_custom_rewards("1234", true, &["8969ec47-55b6-4559-a8fe-3f1fc4e6fe58".into()], &token)
    // ///     .await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub async fn get_custom_rewards<'b, T>(
    //     &'client self,
    //     broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
    //     only_managable_rewards: bool,
    //     // FIXME: This should be `impl AsRef<[&'b T]> + 'b`
    //     ids: &'b [&'b types::RewardIdRef],
    //     token: &T,
    // ) -> Result<Vec<helix::points::CustomReward>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_get(
    //             helix::points::GetCustomRewardRequest::broadcaster_id(broadcaster_id)
    //                 .only_manageable_rewards(only_managable_rewards)
    //                 .ids(ids),
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // #[cfg(feature = "eventsub")]
    // /// Create an [EventSub](crate::eventsub) subscription
    // pub async fn create_eventsub_subscription<T, E: crate::eventsub::EventSubscription>(
    //     &'client self,
    //     subscription: E,
    //     transport: crate::eventsub::Transport,
    //     token: &T,
    // ) -> Result<helix::eventsub::CreateEventSubSubscription<E>, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_post(
    //             helix::eventsub::CreateEventSubSubscriptionRequest::new(),
    //             helix::eventsub::CreateEventSubSubscriptionBody::new(subscription, transport),
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // #[cfg(feature = "eventsub")]
    // /// Delete an [EventSub](crate::eventsub) subscription
    // pub async fn delete_eventsub_subscription<'b, T>(
    //     &'client self,
    //     id: impl types::IntoCow<'b, types::EventSubIdRef> + 'b,
    //     token: &T,
    // ) -> Result<helix::eventsub::DeleteEventSubSubscription, ClientError<'client, C>>
    // where
    //     T: TwitchToken + ?Sized,
    // {
    //     Ok(self
    //         .req_delete(
    //             helix::eventsub::DeleteEventSubSubscriptionRequest::id(id),
    //             token,
    //         )
    //         .await?
    //         .data)
    // }

    // #[cfg(feature = "eventsub")]
    // /// Get all [EventSub](crate::eventsub) subscriptions for this [Client](twitch_oauth2::TwitchToken)
    // ///
    // /// # Notes
    // ///
    // /// The return item is a struct [`EventSubSubscriptions`](helix::eventsub::EventSubSubscriptions) which contains the subscriptions.
    // /// See the example for getting only the subscriptions
    // ///
    // /// # Examples
    // ///
    // /// ```rust, no_run
    // /// # #[tokio::main]
    // /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    // /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    // /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    // /// use twitch_api::{helix, eventsub};
    // /// use futures::{TryStreamExt, stream};
    // ///
    // /// let mut total_cost = None;
    // ///
    // /// let chatters: Vec<eventsub::EventSubSubscription> = client
    // ///     .get_eventsub_subscriptions(None, None, None, &token)
    // ///     .map_ok(|r| {
    // ///         total_cost = Some(r.total_cost);
    // ///         stream::iter(
    // ///             r.subscriptions
    // ///                 .into_iter()
    // ///                 .map(Ok::<_, twitch_api::helix::ClientRequestError<_>>),
    // ///         )
    // ///     })
    // ///     .try_flatten()
    // ///     .try_collect()
    // ///     .await?;
    // ///
    // /// # Ok(()) }
    // /// ```
    // pub fn get_eventsub_subscriptions<'b: 'client, T>(
    //     &'client self,
    //     status: impl Into<Option<crate::eventsub::Status>>,
    //     event_type: impl Into<Option<crate::eventsub::EventType>>,
    //     // FIXME: IntoOptionCow?
    //     user_id: Option<&'b types::UserIdRef>,
    //     token: &'client T,
    // ) -> std::pin::Pin<
    //     Box<
    //         dyn futures::Stream<
    //                 Item = Result<helix::eventsub::EventSubSubscriptions, ClientError<'client, C>>,
    //             > + 'client,
    //     >,
    // >
    // where
    //     T: TwitchToken + Send + Sync + ?Sized,
    // {
    //     let req = helix::eventsub::GetEventSubSubscriptionsRequest {
    //         status: status.into(),
    //         type_: event_type.into(),
    //         user_id: user_id.map(|c| c.as_cow()),
    //         after: None,
    //         first: None,
    //     };

    //     make_stream(req, token, self, |r| {
    //         let mut vec = std::collections::VecDeque::new();
    //         vec.push_front(r);
    //         vec
    //     })
    // }
}

// #[derive(Debug, thiserror::Error)]
// pub enum ClientExtError<'a, C: crate::HttpClient<'a>, E> {
//     #[error(transparent)]
//     ClientError(ClientError<'a, C>),
//     #[error(transparent)]
//     Other(#[from] E),
// }

// /// Make a paginate-able request into a stream
// ///
// /// # Examples
// ///
// /// ```rust, no_run
// /// # #[tokio::main]
// /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
// /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
// /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
// /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
// /// use twitch_api::helix;
// /// use futures::TryStreamExt;
// ///
// /// let req = helix::moderation::GetModeratorsRequest::broadcaster_id("1234");
// ///
// /// helix::make_stream(req, &token, &client, std::collections::VecDeque::from).try_collect::<Vec<_>>().await?
// /// # ;
// /// # Ok(())
// /// # }
// /// ```
// pub fn make_stream<
//     'a,
//     C: crate::HttpClient<'a> + Send + Sync,
//     T: TwitchToken + ?Sized + Send + Sync,
//     // FIXME: Why does this have to be clone and debug?
//     Req: super::Request
//         + super::RequestGet
//         + super::Paginated
//         + Clone
//         + std::fmt::Debug
//         + Send
//         + Sync
//         + 'a,
//     // FIXME: this 'a seems suspicious
//     Item: Send + 'a,
// >(
//     req: Req,
//     token: &'a T,
//     client: &'a super::HelixClient<'a, C>,
//     fun: impl Fn(<Req as super::Request>::Response) -> std::collections::VecDeque<Item>
//         + Send
//         + Sync
//         + Copy
//         + 'static,
// ) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<Item, ClientError<'a, C>>> + 'a + Send>>
// where
//     // FIXME: This clone is bad. I want to be able to return the data, but not in a way that limits the response to be Default
//     // I also want to keep allocations low, so std::mem::take is perfect, but that makes get_next not work optimally.
//     <Req as super::Request>::Response: Send + Sync + std::fmt::Debug + Clone,
// {
//     use futures::StreamExt;
//     enum StateMode<Req: super::Request + super::RequestGet, Item> {
//         /// A request needs to be done.
//         Req(Option<Req>),
//         /// We have made a request, now working through the data
//         Cont(
//             super::Response<Req, <Req as super::Request>::Response>,
//             std::collections::VecDeque<Item>,
//         ),
//         Next(Option<super::Response<Req, <Req as super::Request>::Response>>),
//         /// The operation failed, allowing no further processing
//         Failed,
//     }

//     impl<Req: super::Request + super::RequestGet, Item> StateMode<Req, Item> {
//         fn take_initial(&mut self) -> Req {
//             match self {
//                 StateMode::Req(ref mut r) if r.is_some() => std::mem::take(r).expect("oops"),
//                 _ => todo!("hmmm"),
//             }
//         }

//         fn take_next(&mut self) -> super::Response<Req, <Req as super::Request>::Response> {
//             match self {
//                 StateMode::Next(ref mut r) if r.is_some() => std::mem::take(r).expect("oops"),
//                 _ => todo!("hmmm"),
//             }
//         }
//     }

//     struct State<
//         'a,
//         C: crate::HttpClient<'a>,
//         T: TwitchToken + ?Sized,
//         Req: super::Request + super::RequestGet,
//         Item,
//     > {
//         mode: StateMode<Req, Item>,
//         client: &'a HelixClient<'a, C>,
//         token: &'a T,
//     }

//     impl<
//             'a,
//             C: crate::HttpClient<'a>,
//             T: TwitchToken + ?Sized,
//             Req: super::Request + super::RequestGet + super::Paginated,
//             Item,
//         > State<'a, C, T, Req, Item>
//     {
//         /// Process a request, with a given deq
//         fn process(
//             mut self,
//             r: super::Response<Req, <Req as super::Request>::Response>,
//             d: std::collections::VecDeque<Item>,
//         ) -> Self {
//             self.mode = StateMode::Cont(r, d);
//             self
//         }

//         fn failed(mut self) -> Self {
//             self.mode = StateMode::Failed;
//             self
//         }

//         /// get the next
//         fn get_next(mut self) -> Self {
//             match self.mode {
//                 StateMode::Cont(r, d) => {
//                     assert!(d.is_empty());
//                     self.mode = StateMode::Next(Some(r));
//                     self
//                 }
//                 _ => panic!("oops"),
//             }
//         }
//     }
//     let statemode = StateMode::Req(Some(req));
//     let state = State {
//         mode: statemode,
//         client,
//         token,
//     };
//     futures::stream::unfold(state, move |mut state: State<_, _, _, _>| async move {
//         match state.mode {
//             StateMode::Req(Some(_)) => {
//                 let req = state.mode.take_initial();
//                 let f = state.client.req_get(req, state.token);
//                 let resp = match f.await {
//                     Ok(resp) => resp,
//                     Err(e) => return Some((Err(e), state.failed())),
//                 };
//                 let mut deq = fun(resp.data.clone());
//                 deq.pop_front().map(|d| (Ok(d), state.process(resp, deq)))
//             }
//             StateMode::Cont(_, ref mut deq) => {
//                 if let Some(d) = deq.pop_front() {
//                     if deq.is_empty() {
//                         Some((Ok(d), state.get_next()))
//                     } else {
//                         Some((Ok(d), state))
//                     }
//                 } else {
//                     // New request returned empty.
//                     None
//                 }
//             }
//             StateMode::Next(Some(_)) => {
//                 let resp = state.mode.take_next();
//                 let f = resp.get_next(state.client, state.token);
//                 let resp = match f.await {
//                     Ok(Some(resp)) => resp,
//                     Ok(None) => return None,
//                     Err(e) => return Some((Err(e), state.failed())),
//                 };
//                 let mut deq = fun(resp.data.clone());
//                 deq.pop_front().map(|d| (Ok(d), state.process(resp, deq)))
//             }
//             _ => todo!("failed to process request"),
//         }
//     })
//     .boxed()
// }
