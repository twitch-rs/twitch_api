//! Convenience functions for [HelixClient]
#![warn(clippy::future_not_send)]
use futures::{StreamExt, TryStreamExt};
use std::borrow::Cow;

use crate::helix::{self, ClientRequestError, HelixClient};
use crate::types;
use twitch_oauth2::TwitchToken;

type ClientError<C> = ClientRequestError<<C as crate::HttpClient>::Error>;

// TODO: Consider moving these into the specific modules where the request is defined. Preferably backed by a macro

impl<'client, C: crate::HttpClient + Sync + 'client> HelixClient<'client, C> {
    /// Get [User](helix::users::User) from user login
    pub async fn get_user_from_login<T>(
        &'client self,
        login: impl Into<&types::UserNameRef> + Send,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<C>>
    where
        T: TwitchToken + Sync + Send + ?Sized,
    {
        self.req_get(
            helix::users::GetUsersRequest::logins(&[login.into()][..]),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Get [User](helix::users::User) from user id
    pub async fn get_user_from_id<T>(
        &'client self,
        id: impl Into<&types::UserIdRef> + Send,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        self.req_get(helix::users::GetUsersRequest::ids(&[id.into()][..]), token)
            .await
            .map(|response| response.first())
    }

    /// Get multiple [User](helix::users::User)s from user ids.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{helix, types};
    /// use futures::TryStreamExt;
    ///
    /// let users: Vec<helix::users::User> = client
    ///     .get_users_from_ids(&["1234", "4321"][..].into(), &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_users_from_ids<T>(
        &'client self,
        ids: &'client types::Collection<'client, types::UserId>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::users::User, ClientError<C>>> + Send + Unpin + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        futures::stream::iter(ids.chunks(100).collect::<Vec<_>>())
            .map(move |c| {
                let req = helix::users::GetUsersRequest::ids(c);
                futures::stream::once(self.req_get(req, token)).boxed()
            })
            .flatten_unordered(None)
            .map_ok(|resp| futures::stream::iter(resp.data.into_iter().map(Ok)))
            .try_flatten_unordered(None)
    }

    /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters login
    pub async fn get_channel_from_login<T>(
        &'client self,
        login: impl Into<&types::UserNameRef> + Send,
        token: &T,
    ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(login.into(), token).await? {
            self.get_channel_from_id(&user.id, token).await
        } else {
            Ok(None)
        }
    }

    /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters id
    pub async fn get_channel_from_id<T>(
        &'client self,
        id: impl Into<&types::UserIdRef> + Send,
        token: &T,
    ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let ids: &[_] = &[id.into()];
        self.req_get(
            helix::channels::GetChannelInformationRequest::broadcaster_ids(ids),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Get multiple [ChannelInformation](helix::channels::ChannelInformation) from broadcasters ids
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{helix, types};
    /// use futures::TryStreamExt;
    ///
    /// let chatters: Vec<helix::channels::ChannelInformation> = client
    ///     .get_channels_from_ids(&["1234", "4321"][..].into(), &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_channels_from_ids<T>(
        &'client self,
        ids: &'client types::Collection<'client, types::UserId>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::channels::ChannelInformation, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        futures::stream::iter(ids.chunks(100).collect::<Vec<_>>())
            .map(move |c| {
                let req = helix::channels::GetChannelInformationRequest::broadcaster_ids(c);
                futures::stream::once(self.req_get(req, token)).boxed()
            })
            .flatten_unordered(None)
            .map_ok(|resp| futures::stream::iter(resp.data.into_iter().map(Ok)))
            .try_flatten_unordered(None)
    }

    /// Get multiple [Stream](helix::streams::Stream)s from user ids.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// let live: Vec<helix::streams::Stream> = client
    ///     .get_streams_from_ids(&["123456", "987654"][..].into(), &token)
    ///     .try_collect()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn get_streams_from_ids<T>(
        &'client self,
        ids: &'client types::Collection<'client, types::UserId>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::streams::Stream, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let ids = ids.chunks(100).collect::<Vec<_>>();
        futures::stream::iter(ids.into_iter().map(move |c| {
            let req = helix::streams::GetStreamsRequest::user_ids(c).first(100);
            make_stream(req, token, self, std::collections::VecDeque::from)
        }))
        .flatten_unordered(None)
    }

    /// Get multiple [Stream](helix::streams::Stream)s from user logins.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// let live: Vec<helix::streams::Stream> = client
    ///     .get_streams_from_logins(&["twitchdev", "justinfan"][..].into(), &token)
    ///     .try_collect()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn get_streams_from_logins<T>(
        &'client self,
        logins: &'client types::Collection<'client, types::UserName>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::streams::Stream, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let logins = logins.chunks(100).collect::<Vec<_>>();
        futures::stream::iter(logins.into_iter().map(move |c| {
            let req = helix::streams::GetStreamsRequest::user_logins(c).first(100);
            make_stream(req, token, self, std::collections::VecDeque::from)
        }))
        .flatten_unordered(None)
    }

    /// Gets the channel’s stream key.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use twitch_oauth2::TwitchToken;
    ///
    /// // use the associated user-id  with the user-access token
    /// let user_id = token.user_id().expect("no user-id set in token");
    /// let key: twitch_types::StreamKey = client.get_stream_key(user_id, &token).await?;
    /// # Ok(()) }
    /// ```
    pub async fn get_stream_key<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b + Send,
        token: &T,
    ) -> Result<types::StreamKey, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        self.req_get(
            helix::streams::GetStreamKeyRequest::broadcaster_id(broadcaster_id),
            token,
        )
        .await
        .map(|res| res.data.stream_key)
    }

    /// Adds a marker to a live stream.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use twitch_oauth2::TwitchToken;
    ///
    /// // use the associated user-id  with the user-access token
    /// let user_id = token.user_id().expect("no user-id set in token");
    /// let marker: helix::streams::CreatedStreamMarker = client.create_stream_marker(user_id, "my description", &token).await?;
    /// # Ok(()) }
    /// ```
    pub async fn create_stream_marker<'b, T>(
        &'client self,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b + Send,
        description: impl Into<Cow<'b, str>> + Send,
        token: &T,
    ) -> Result<helix::streams::CreatedStreamMarker, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        self.req_post(
            helix::streams::CreateStreamMarkerRequest::new(),
            helix::streams::CreateStreamMarkerBody::new(user_id, description),
            token,
        )
        .await
        .map(|res| res.data)
    }

    /// Get chatters in a stream [Chatter][helix::chat::Chatter]
    ///
    /// `batch_size` sets the amount of chatters to retrieve per api call, max 1000, defaults to 100.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let chatters: Vec<helix::chat::Chatter> = client
    ///    .get_chatters("1234", "4321", 1000, &token)
    ///    .try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_chatters<T>(
        &'client self,
        broadcaster_id: impl Into<&'client types::UserIdRef>,
        moderator_id: impl Into<&'client types::UserIdRef>,
        batch_size: impl Into<Option<usize>>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::chat::Chatter, ClientError<C>>> + Send + Unpin + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::chat::GetChattersRequest {
            first: batch_size.into(),
            ..helix::chat::GetChattersRequest::new(broadcaster_id.into(), moderator_id.into())
        };

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Search [Categories](helix::search::Category)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let categories: Vec<helix::search::Category> = client
    ///     .search_categories("Fortnite", &token)
    ///     .try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn search_categories<T>(
        &'client self,
        query: impl Into<&'client str>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::search::Category, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::search::SearchCategoriesRequest::query(query.into()).first(100);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Search [Channels](helix::search::Channel) via channel name or description
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let channel: Vec<helix::search::Channel> = client
    ///     .search_channels("twitchdev", false, &token)
    ///     .try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn search_channels<'b, T>(
        &'client self,
        query: impl Into<&'b str>,
        live_only: bool,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::search::Channel, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
        'b: 'client,
    {
        let req = helix::search::SearchChannelsRequest::query(query.into()).live_only(live_only);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get information on a [follow relationship](helix::users::FollowRelationship)
    ///
    /// Can be used to see if X follows Y
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// // Get the followers of channel "1234"
    /// let followers: Vec<helix::users::FollowRelationship> = client
    ///     .get_follow_relationships(Some("1234".into()), None, &token)
    ///     .try_collect().await?;
    /// # Ok(()) }
    /// ```
    #[deprecated(
        note = "this method will not work anymore on 3 august, see https://discuss.dev.twitch.tv/t/follows-endpoints-and-eventsub-subscription-type-are-now-available-in-open-beta/43322"
    )]
    #[allow(deprecated)]
    #[doc(hidden)]
    pub fn get_follow_relationships<'b, T>(
        &'client self,
        to_id: impl Into<Option<&'b types::UserIdRef>>,
        from_id: impl Into<Option<&'b types::UserIdRef>>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::users::FollowRelationship, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
        'b: 'client,
    {
        let mut req = helix::users::GetUsersFollowsRequest::empty();
        req.to_id = to_id.into().map(Cow::Borrowed);
        req.from_id = from_id.into().map(Cow::Borrowed);

        make_stream(req, token, self, |s| {
            std::collections::VecDeque::from(s.follow_relationships)
        })
    }

    /// Get authenticated users' followed [streams](helix::streams::Stream)
    ///
    /// Requires token with scope [`user:read:follows`](twitch_oauth2::Scope::UserReadFollows).
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let channels: Vec<helix::streams::Stream> = client
    ///     .get_followed_streams(&token)
    ///     .try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_followed_streams<T>(
        &'client self,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::streams::Stream, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let user_id = match token
            .user_id()
            .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
        {
            Ok(t) => t,
            Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
        };
        let req = helix::streams::GetFollowedStreamsRequest::user_id(user_id);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get authenticated broadcasters' [subscribers](helix::subscriptions::BroadcasterSubscription)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let subs: Vec<helix::subscriptions::BroadcasterSubscription> = client
    ///     .get_broadcaster_subscriptions(&token)
    ///     .try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_broadcaster_subscriptions<T>(
        &'client self,
        token: &'client T,
    ) -> impl futures::Stream<
        Item = Result<helix::subscriptions::BroadcasterSubscription, ClientError<C>>,
    > + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let user_id = match token
            .user_id()
            .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
        {
            Ok(t) => t,
            Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
        };
        // If this fails to compile due to missing implementation, make sure this crate and `twitch_oauth2` use the same version of `twitch_types`
        let req = helix::subscriptions::GetBroadcasterSubscriptionsRequest::broadcaster_id(user_id);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get all moderators in a channel [Get Moderators](helix::moderation::GetModeratorsRequest)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let moderators: Vec<helix::moderation::Moderator> = client
    ///     .get_moderators_in_channel_from_id("twitchdev", &token)
    ///     .try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_moderators_in_channel_from_id<'b: 'client, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::moderation::Moderator, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::GetModeratorsRequest::broadcaster_id(broadcaster_id);

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get all banned users in a channel [Get Banned Users](helix::moderation::GetBannedUsersRequest)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let moderators: Vec<helix::moderation::BannedUser> = client.get_banned_users_in_channel_from_id("twitchdev", &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_banned_users_in_channel_from_id<'b: 'client, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::moderation::BannedUser, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::GetBannedUsersRequest::broadcaster_id(broadcaster_id);

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Gets a list of unban requests for a broadcaster’s channel. [Get Unban Requests](helix::moderation::GetUnbanRequestsRequest)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let requests: Vec<helix::moderation::UnbanRequest> = client.get_unban_requests("1234", "5678", helix::moderation::UnbanRequestStatus::Pending, &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_unban_requests<'b: 'client, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        status: helix::moderation::UnbanRequestStatus,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::moderation::UnbanRequest, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req =
            helix::moderation::GetUnbanRequestsRequest::new(broadcaster_id, moderator_id, status);

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Gets a list of channels that the specified user has moderator privileges in. [Get Moderated Channels](helix::moderation::GetModeratedChannelsRequest)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use twitch_oauth2::TwitchToken;
    /// use futures::TryStreamExt;
    ///
    /// // use the associated user-id  with the user-access token
    /// let user_id = token.user_id().expect("no user-id set in token");
    /// let requests: Vec<helix::moderation::ModeratedChannel> = client.get_moderated_channels(user_id, &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_moderated_channels<'b: 'client, T>(
        &'client self,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::moderation::ModeratedChannel, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::GetModeratedChannelsRequest::user_id(user_id);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get a users, with login, follow count
    #[deprecated(
        note = "this method will not work anymore on 3 august, see https://discuss.dev.twitch.tv/t/follows-endpoints-and-eventsub-subscription-type-are-now-available-in-open-beta/43322"
    )]
    #[allow(deprecated)]
    #[doc(hidden)]
    pub async fn get_total_followers_from_login<'b, T>(
        &'client self,
        login: impl types::IntoCow<'b, types::UserNameRef> + Send + 'b,
        token: &T,
    ) -> Result<Option<i64>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(&*login.into_cow(), token).await? {
            self.get_total_followers_from_id(&user.id, token)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get a broadcasters follow count
    ///
    /// # Notes
    ///
    /// You need to have the scope `moderator:read:followers` and be a moderator of the channel if the token is not the broadcasters own token
    pub async fn get_total_channel_followers<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<i64, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let resp = self
            .req_get(
                helix::channels::GetChannelFollowersRequest::broadcaster_id(broadcaster_id),
                token,
            )
            .await?;

        Ok(resp.total.unwrap_or(0))
    }

    /// Get users followed channels
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let schedule: Vec<helix::channels::FollowedBroadcaster> = client
    ///     .get_followed_channels("1234", &token)
    ///     .try_collect()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn get_followed_channels<'b: 'client, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::channels::FollowedBroadcaster, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req =
            helix::channels::get_followed_channels::GetFollowedChannels::user_id(broadcaster_id);
        make_stream(req, token, self, |broadcasts| broadcasts.into())
    }

    /// Get a users, with id, follow count
    ///
    /// # Notes
    ///
    /// This returns zero if the user doesn't exist
    #[allow(deprecated)]
    #[doc(hidden)]
    #[deprecated(
        note = "this method will not work anymore on 3 august, see https://discuss.dev.twitch.tv/t/follows-endpoints-and-eventsub-subscription-type-are-now-available-in-open-beta/43322"
    )]
    pub async fn get_total_followers_from_id<'b, T>(
        &'client self,
        to_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<i64, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let resp = self
            .req_get(
                helix::users::GetUsersFollowsRequest::followers(to_id),
                token,
            )
            .await?;

        Ok(resp.data.total)
    }

    /// Get games by ID.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// let games: Vec<helix::games::Game> = client
    ///     .get_games_by_id(&["509658", "32982", "27471"][..].into(), &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_games_by_id<T>(
        &'client self,
        ids: &'client types::Collection<'client, types::CategoryId>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::games::Game, ClientError<C>>> + Send + Unpin + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        futures::stream::iter(ids.chunks(100).collect::<Vec<_>>())
            .map(move |c| {
                let req = helix::games::GetGamesRequest::ids(c);
                futures::stream::once(self.req_get(req, token)).boxed()
            })
            .flatten_unordered(None)
            .map_ok(|resp| futures::stream::iter(resp.data.into_iter().map(Ok)))
            .try_flatten_unordered(None)
    }

    /// Block a user
    pub async fn block_user<'b, T>(
        &'client self,
        target_user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::users::BlockUser, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_put(
                helix::users::BlockUserRequest::block_user(target_user_id),
                helix::EmptyBody,
                token,
            )
            .await?
            .data)
    }

    /// Unblock a user
    pub async fn unblock_user<'b, T>(
        &'client self,
        target_user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::users::UnblockUser, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_delete(
                helix::users::UnblockUserRequest::unblock_user(target_user_id),
                token,
            )
            .await?
            .data)
    }

    /// Ban a user
    pub async fn ban_user<'b, T>(
        &'client self,
        target_user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        reason: impl Into<&'b str> + Send,
        duration: impl Into<Option<u32>> + Send,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::moderation::BanUser, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_post(
                helix::moderation::BanUserRequest::new(broadcaster_id, moderator_id),
                helix::moderation::BanUserBody::new(target_user_id, reason.into(), duration),
                token,
            )
            .await?
            .data)
    }

    /// Unban a user
    pub async fn unban_user<'b, T>(
        &'client self,
        target_user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::moderation::UnbanUserResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_delete(
                helix::moderation::UnbanUserRequest::new(
                    broadcaster_id,
                    moderator_id,
                    target_user_id,
                ),
                token,
            )
            .await?
            .data)
    }

    #[cfg(feature = "beta")]
    /// Warn a user
    pub async fn warn_chat_user<'b, T>(
        &'client self,
        target_user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        reason: impl Into<&'b str> + Send,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::moderation::WarnChatUser, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_post(
                helix::moderation::WarnChatUserRequest::new(broadcaster_id, moderator_id),
                helix::moderation::WarnChatUserBody::new(target_user_id, reason.into()),
                token,
            )
            .await?
            .data)
    }

    // FIXME: Example should use https://github.com/twitch-rs/twitch_api/issues/162
    /// Get all scheduled streams in a channel.
    ///
    /// # Notes
    ///
    /// Make sure to limit the data here using [`try_take_while`](futures::stream::TryStreamExt::try_take_while), otherwise this will never end on recurring scheduled streams.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let schedule: Vec<helix::schedule::Segment> = client
    ///     .get_channel_schedule("twitchdev", &token)
    ///     .try_take_while(|s| {
    ///         futures::future::ready(Ok(!s.start_time.as_str().starts_with("2021-10")))
    ///     })
    ///     .try_collect()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn get_channel_schedule<'b: 'client, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::schedule::Segment, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::schedule::GetChannelStreamScheduleRequest::broadcaster_id(broadcaster_id);

        make_stream(req, token, self, |broadcasts| broadcasts.segments.into())
    }

    /// Get all global emotes
    pub async fn get_global_emotes<T>(
        &'client self,
        token: &T,
    ) -> Result<Vec<helix::chat::GlobalEmote>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::chat::GetGlobalEmotesRequest::new();
        Ok(self.req_get(req, token).await?.data)
    }

    /// Get channel emotes in channel with user id
    pub async fn get_channel_emotes_from_id<'b, T>(
        &'client self,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<Vec<helix::chat::ChannelEmote>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::chat::GetChannelEmotesRequest::broadcaster_id(user_id);
        Ok(self.req_get(req, token).await?.data)
    }

    /// Get channel emotes in channel with user login
    pub async fn get_channel_emotes_from_login<T>(
        &'client self,
        login: impl types::IntoCow<'client, types::UserNameRef> + Send + 'client,
        token: &T,
    ) -> Result<Option<Vec<helix::chat::ChannelEmote>>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        if let Some(user) = self
            .get_user_from_login(login.into_cow().as_ref(), token)
            .await?
        {
            self.get_channel_emotes_from_id(&user.id, token)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get all emotes accessible to the user in all chats.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use twitch_oauth2::TwitchToken;
    /// use futures::TryStreamExt;
    ///
    /// // use the associated user-id with the user-access token
    /// let user_id = token.user_id().expect("no user-id set in token");
    /// let requests: Vec<helix::chat::UserEmote> = client.get_user_emotes(user_id, &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_user_emotes<'b: 'client, T>(
        &'client self,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::chat::UserEmote, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::chat::GetUserEmotesRequest::user_id(user_id);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get all emotes accessible to the user in a channel.
    ///
    /// This will include "follow" emotes if the user isn't subscribed but following.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use twitch_oauth2::TwitchToken;
    /// use futures::TryStreamExt;
    ///
    /// // use the associated user-id with the user-access token
    /// let user_id = token.user_id().expect("no user-id set in token");
    /// let requests: Vec<helix::chat::UserEmote> = client.get_user_emotes_in_channel(user_id, "1234", &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_user_emotes_in_channel<'b: 'client, 'c: 'client, T>(
        &'client self,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        channel_id: impl types::IntoCow<'c, types::UserIdRef> + 'c,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::chat::UserEmote, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let mut req = helix::chat::GetUserEmotesRequest::user_id(user_id);
        req.broadcaster_id = Some(twitch_types::IntoCow::into_cow(channel_id));
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get emotes in emote sets
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// let emotes: Vec<helix::chat::get_emote_sets::Emote> = client
    ///     .get_emote_sets(&["0"][..].into(), &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_emote_sets<T>(
        &'client self,
        emote_sets: &'client types::Collection<'client, types::EmoteSetId>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::chat::get_emote_sets::Emote, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        futures::stream::iter(emote_sets.chunks(25).collect::<Vec<_>>())
            .map(move |c| {
                let req = helix::chat::GetEmoteSetsRequest::emote_set_ids(c);
                futures::stream::once(self.req_get(req, token)).boxed()
            })
            .flatten_unordered(None)
            .map_ok(|r| futures::stream::iter(r.data.into_iter().map(Ok)))
            .try_flatten_unordered(None)
    }

    /// Get a broadcaster's chat settings
    pub async fn get_chat_settings<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl Into<Option<&'b types::UserIdRef>> + Send + 'b,
        token: &T,
    ) -> Result<helix::chat::ChatSettings, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let mut req = helix::chat::GetChatSettingsRequest::broadcaster_id(broadcaster_id);
        if let Some(moderator_id) = moderator_id.into() {
            req = req.moderator_id(moderator_id);
        }
        Ok(self.req_get(req, token).await?.data)
    }

    /// Send a chat announcement
    pub async fn send_chat_announcement<'b, T, E>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        message: impl Into<&'b str> + Send,
        color: impl std::convert::TryInto<helix::chat::AnnouncementColor, Error = E> + Send,
        token: &T,
    ) -> Result<helix::chat::SendChatAnnouncementResponse, ClientExtError<C, E>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::chat::SendChatAnnouncementRequest::new(broadcaster_id, moderator_id);
        let body = helix::chat::SendChatAnnouncementBody::new(message.into(), color)?;
        Ok(self
            .req_post(req, body, token)
            .await
            .map_err(ClientExtError::ClientError)?
            .data)
    }

    /// Delete a specific chat message
    pub async fn delete_chat_message<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        message_id: impl types::IntoCow<'b, types::MsgIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::moderation::DeleteChatMessagesResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::DeleteChatMessagesRequest::new(broadcaster_id, moderator_id)
            .message_id(message_id);
        Ok(self.req_delete(req, token).await?.data)
    }

    /// Delete all chat messages in a broadcasters chat room
    pub async fn delete_all_chat_message<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::moderation::DeleteChatMessagesResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::DeleteChatMessagesRequest::new(broadcaster_id, moderator_id);
        Ok(self.req_delete(req, token).await?.data)
    }

    /// Start a raid
    pub async fn start_a_raid<'b, T>(
        &'client self,
        from_broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        to_broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::raids::StartARaidResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::raids::StartARaidRequest::new(from_broadcaster_id, to_broadcaster_id);
        Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    }

    /// Cancel a raid
    pub async fn cancel_a_raid<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::raids::CancelARaidResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::raids::CancelARaidRequest::broadcaster_id(broadcaster_id);
        Ok(self.req_delete(req, token).await?.data)
    }

    /// Update a user's chat color
    pub async fn update_user_chat_color<'b, T>(
        &'client self,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        color: impl Into<types::NamedUserColor<'b>> + Send + 'b,
        token: &T,
    ) -> Result<helix::chat::UpdateUserChatColorResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::chat::UpdateUserChatColorRequest {
            user_id: user_id.into_cow(),
            color: color.into(),
        };

        Ok(self.req_put(req, helix::EmptyBody, token).await?.data)
    }

    /// Get a user's chat color
    ///
    /// [`None`](Option::None) is returned if the user never set their color in the settings.
    pub async fn get_user_chat_color<T>(
        &'client self,
        user_id: impl Into<&types::UserIdRef> + Send,
        token: &T,
    ) -> Result<Option<helix::chat::UserChatColor>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_get(
                helix::chat::GetUserChatColorRequest::user_ids(&user_id.into()),
                token,
            )
            .await?
            .first())
    }

    /// Get multiple users' chat colors
    ///
    /// Users that never set their color in the settings are not returned.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// let colors: Vec<helix::chat::UserChatColor> = client
    ///     .get_users_chat_colors(&["1234"][..].into(), &token).try_collect().await?;
    /// # Ok(()) }
    /// ```
    pub fn get_users_chat_colors<T>(
        &'client self,
        user_ids: &'client types::Collection<'client, types::UserId>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::chat::UserChatColor, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        futures::stream::iter(user_ids.chunks(100))
            .map(move |c| {
                let req = helix::chat::GetUserChatColorRequest::user_ids(c);
                futures::stream::once(self.req_get(req, token)).boxed()
            })
            .flatten_unordered(None)
            .map_ok(move |o| futures::stream::iter(o.data.into_iter().map(Ok)))
            .try_flatten_unordered(None)
    }

    /// Retrieves the active shared chat session for a channel
    ///
    /// [`None`] is returned if no shared chat session is active.
    pub async fn get_shared_chat_session<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<Option<helix::chat::SharedChatSession>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_get(
                helix::chat::GetSharedChatSessionRequest::broadcaster_id(broadcaster_id),
                token,
            )
            .await?
            .first())
    }

    /// Add a channel moderator
    pub async fn add_channel_moderator<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::moderation::AddChannelModeratorResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::AddChannelModeratorRequest {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
        };

        Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    }

    /// Remove a channel moderator
    pub async fn remove_channel_moderator<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        moderator_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::moderation::RemoveChannelModeratorResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::RemoveChannelModeratorRequest {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
        };

        Ok(self.req_delete(req, token).await?.data)
    }

    /// Get channel VIPs
    pub fn get_vips_in_channel<'b: 'client, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + 'b,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::channels::Vip, ClientError<C>>> + Send + Unpin + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::channels::GetVipsRequest::broadcaster_id(broadcaster_id);

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Add a channel vip
    pub async fn add_channel_vip<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::channels::AddChannelVipResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::channels::AddChannelVipRequest {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: user_id.into_cow(),
        };

        Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    }

    /// Remove a channel vip
    pub async fn remove_channel_vip<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        user_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::channels::RemoveChannelVipResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::channels::RemoveChannelVipRequest {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: user_id.into_cow(),
        };

        Ok(self.req_delete(req, token).await?.data)
    }

    /// Send a whisper
    pub async fn send_whisper<'b, T>(
        &'client self,
        from: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        to: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        message: impl Into<&'b str> + Send,
        token: &T,
    ) -> Result<helix::whispers::SendWhisperResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_post(
                helix::whispers::SendWhisperRequest::new(from, to),
                helix::whispers::SendWhisperBody::new(message.into()),
                token,
            )
            .await?
            .data)
    }

    /// Get all custom rewards
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    ///
    /// let rewards: Vec<helix::points::CustomReward> = client
    ///     .get_all_custom_rewards("1234", true, &token)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub async fn get_all_custom_rewards<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        only_managable_rewards: bool,
        token: &T,
    ) -> Result<Vec<helix::points::CustomReward>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        self.get_custom_rewards(
            broadcaster_id,
            only_managable_rewards,
            &types::Collection::EMPTY,
            token,
        )
        .await
    }

    /// Get specific custom rewards, see [`get_all_custom_rewards`](HelixClient::get_all_custom_rewards) to get all rewards
    ///
    /// # Notes
    ///
    /// Takes a max of 50 ids
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    ///
    /// let rewards: Vec<helix::points::CustomReward> = client
    ///     .get_custom_rewards("1234", true, &["8969ec47-55b6-4559-a8fe-3f1fc4e6fe58"][..].into(), &token)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    // XXX: This function is useless as a stream, since you can never have more than 50 rewards on a channel
    pub async fn get_custom_rewards<'b, T>(
        &'client self,
        broadcaster_id: impl types::IntoCow<'b, types::UserIdRef> + Send + 'b,
        only_managable_rewards: bool,
        ids: &'b types::Collection<'b, types::RewardId>,
        token: &'client T,
    ) -> Result<Vec<helix::points::CustomReward>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_get(
                helix::points::GetCustomRewardRequest::broadcaster_id(broadcaster_id)
                    .only_manageable_rewards(only_managable_rewards)
                    .ids(ids.clone()),
                token,
            )
            .await?
            .data)
    }

    #[cfg(feature = "eventsub")]
    /// Create an [EventSub](crate::eventsub) subscription
    pub async fn create_eventsub_subscription<T, E: crate::eventsub::EventSubscription + Send>(
        &'client self,
        subscription: E,
        transport: crate::eventsub::Transport,
        token: &T,
    ) -> Result<helix::eventsub::CreateEventSubSubscription<E>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_post(
                helix::eventsub::CreateEventSubSubscriptionRequest::new(),
                helix::eventsub::CreateEventSubSubscriptionBody::new(subscription, transport),
                token,
            )
            .await?
            .data)
    }

    #[cfg(feature = "eventsub")]
    /// Delete an [EventSub](crate::eventsub) subscription
    pub async fn delete_eventsub_subscription<'b, T>(
        &'client self,
        id: impl types::IntoCow<'b, types::EventSubIdRef> + Send + 'b,
        token: &T,
    ) -> Result<helix::eventsub::DeleteEventSubSubscription, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        Ok(self
            .req_delete(
                helix::eventsub::DeleteEventSubSubscriptionRequest::id(id),
                token,
            )
            .await?
            .data)
    }

    #[cfg(feature = "eventsub")]
    /// Get all [EventSub](crate::eventsub) subscriptions for this [Client](twitch_oauth2::TwitchToken)
    ///
    /// # Notes
    ///
    /// The return item is a struct [`EventSubSubscriptions`](helix::eventsub::EventSubSubscriptions)
    /// which contains a field with all the subscriptions.
    /// See the example for collecting all _specific_ subscriptions
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{helix, eventsub};
    /// use futures::{TryStreamExt, stream};
    ///
    /// let mut total_cost = None;
    ///
    /// let chatters: Vec<eventsub::EventSubSubscription> = client
    ///     .get_eventsub_subscriptions(None, None, None, &token)
    ///     .map_ok(|r| {
    ///         total_cost = Some(r.total_cost);
    ///         stream::iter(
    ///             r.subscriptions
    ///                 .into_iter()
    ///                 .map(Ok::<_, twitch_api::helix::ClientRequestError<_>>),
    ///         )
    ///     })
    ///     .try_flatten()
    ///     .try_collect()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn get_eventsub_subscriptions<'b: 'client, T>(
        &'client self,
        status: impl Into<Option<crate::eventsub::Status>>,
        event_type: impl Into<Option<crate::eventsub::EventType>>,
        // FIXME: IntoOptionCow?
        user_id: Option<&'b types::UserIdRef>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<helix::eventsub::EventSubSubscriptions, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::eventsub::GetEventSubSubscriptionsRequest {
            status: status.into(),
            type_: event_type.into(),
            user_id: user_id.map(|c| c.as_cow()),
            after: None,
            first: None,
        };

        make_stream(req, token, self, |r| {
            let mut vec = std::collections::VecDeque::new();
            vec.push_front(r);
            vec
        })
    }

    #[cfg(feature = "eventsub")]
    /// Get all [Conduits](crate::eventsub::Conduit) for the Twitch Developer Application
    /// associated with this token
    ///
    /// # Notes
    ///
    /// The token must be an App Access Token
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let client_id = twitch_oauth2::types::ClientId::from_static("your_client_id");
    /// # let client_secret = twitch_oauth2::types::ClientSecret::from_static("your_client_id");
    /// # let token = twitch_oauth2::AppAccessToken::get_app_access_token(&client, client_id, client_secret, vec![]).await?;
    /// use twitch_api::{helix, eventsub};
    ///
    /// let conduits = client.get_conduits(&token).await?;
    ///
    /// # Ok(()) }
    /// ```
    pub async fn get_conduits<'b: 'client, T>(
        &'client self,
        token: &'client T,
    ) -> Result<Vec<crate::eventsub::Conduit>, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        self.req_get(helix::eventsub::GetConduitsRequest {}, token)
            .await
            .map(|response| response.data)
    }

    #[cfg(feature = "eventsub")]
    /// Create a [Conduit](crate::eventsub) for the Twitch Developer Application
    /// associated with this token
    ///
    /// # Notes
    ///
    /// The token must be an App Access Token
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let client_id = twitch_oauth2::types::ClientId::from_static("your_client_id");
    /// # let client_secret = twitch_oauth2::types::ClientSecret::from_static("your_client_id");
    /// # let token = twitch_oauth2::AppAccessToken::get_app_access_token(&client, client_id, client_secret, vec![]).await?;
    /// use twitch_api::{helix, eventsub};
    ///
    /// let shard_count = 5;
    /// let created_conduit = client.create_conduit(shard_count, &token).await?;
    ///
    /// # Ok(()) }
    /// ```
    pub async fn create_conduit<'b: 'client, T>(
        &'client self,
        shard_count: usize,
        token: &'client T,
    ) -> Result<crate::eventsub::Conduit, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::eventsub::CreateConduitRequest {};
        let body = helix::eventsub::CreateConduitBody::new(shard_count);

        self.req_post(req, body, token)
            .await
            .map(|response| response.data)
    }

    #[cfg(feature = "eventsub")]
    /// Gets a list of all shards for a conduit.
    ///
    /// # Notes
    ///
    /// The token must be an App Access Token
    ///
    /// # Examples
    ///
    /// ## Get all shards from the given conduit
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let client_id = twitch_oauth2::types::ClientId::from_static("your_client_id");
    /// # let client_secret = twitch_oauth2::types::ClientSecret::from_static("your_client_id");
    /// # let token = twitch_oauth2::AppAccessToken::get_app_access_token(&client, client_id, client_secret, vec![]).await?;
    /// use twitch_api::{helix, eventsub};
    /// use futures::TryStreamExt;
    ///
    /// let conduit_id = "26b1c993-bfcf-44d9-b876-379dacafe75a";
    /// let status = None;
    /// let all_shards: Vec<eventsub::ShardResponse> = client
    ///     .get_conduit_shards(conduit_id, status, &token)
    ///     .try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    ///
    /// ## Get all enabled shards from the given conduit
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let client_id = twitch_oauth2::types::ClientId::from_static("your_client_id");
    /// # let client_secret = twitch_oauth2::types::ClientSecret::from_static("your_client_id");
    /// # let token = twitch_oauth2::AppAccessToken::get_app_access_token(&client, client_id, client_secret, vec![]).await?;
    /// use twitch_api::{helix, eventsub};
    /// use futures::TryStreamExt;
    ///
    /// let conduit_id = "26b1c993-bfcf-44d9-b876-379dacafe75a";
    /// let status = eventsub::ShardStatus::Enabled;
    /// let enabled_shards: Vec<eventsub::ShardResponse> = client
    ///     .get_conduit_shards(conduit_id, status, &token)
    ///     .try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_conduit_shards<'b: 'client, T>(
        &'client self,
        conduit_id: impl Into<Cow<'b, str>>,
        status: impl Into<Option<crate::eventsub::ShardStatus>>,
        token: &'client T,
    ) -> impl futures::Stream<Item = Result<crate::eventsub::ShardResponse, ClientError<C>>>
           + Send
           + Unpin
           + 'client
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::eventsub::GetConduitShardsRequest {
            conduit_id: conduit_id.into(),
            status: status.into(),
            after: None,
        };

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    #[cfg(feature = "eventsub")]
    /// Updates the [Shard](crate::eventsub) for the given [Conduit](crate::eventsub).
    ///
    /// This is used to connect a Webhook or Websocket transport to a conduit, which you can read
    /// more about [here](https://dev.twitch.tv/docs/eventsub/handling-conduit-events/)
    ///
    /// # Notes
    ///
    /// Shard IDs are indexed starting at 0, so a conduit with a shard_count of 5 will have shards with IDs 0 through 4.
    ///
    /// The token must be an App Access Token
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let client_id = twitch_oauth2::types::ClientId::from_static("your_client_id");
    /// # let client_secret = twitch_oauth2::types::ClientSecret::from_static("your_client_id");
    /// # let token = twitch_oauth2::AppAccessToken::get_app_access_token(&client, client_id, client_secret, vec![]).await?;
    /// use twitch_api::{helix, eventsub};
    ///
    /// // The conduit ID of a previously created Conduit
    /// let conduit_id = "bb7a1803-eb03-41ef-a1ab-e9242e72053e";
    /// // The session ID of your WebSocket EventSub connection
    /// let session_id = "AgoQMpdhHZ-dSoyv7NLALgOGHhIGY2VsbC1j";
    /// let shard = twitch_api::eventsub::Shard::new(
    ///     "0",
    ///     twitch_api::eventsub::Transport::websocket(session_id),
    /// );
    ///
    /// let response = client
    ///     .update_conduit_shards(conduit_id, vec![shard], &token)
    ///     .await;
    ///
    /// # Ok(()) }
    /// ```
    pub async fn update_conduit_shards<'b: 'client, T>(
        &'client self,
        conduit_id: impl Into<String> + Send,
        shards: Vec<crate::eventsub::Shard>,
        token: &'client T,
    ) -> Result<helix::eventsub::UpdateConduitShardsResponse, ClientError<C>>
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::eventsub::UpdateConduitShardsRequest {};
        let body = helix::eventsub::UpdateConduitShardsBody::new(conduit_id.into(), shards);

        self.req_patch(req, body, token)
            .await
            .map(|response| response.data)
    }
}

/// Error type to combine a http client error with a other error
#[derive(Debug, thiserror::Error)]
pub enum ClientExtError<C: crate::HttpClient, E> {
    /// Http client error
    #[error(transparent)]
    ClientError(ClientError<C>),
    /// Other error
    #[error(transparent)]
    Other(#[from] E),
}

/// Make a paginate-able request into a stream
///
/// # Examples
///
/// ```rust, no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
/// use twitch_api::helix;
/// use futures::TryStreamExt;
///
/// let req = helix::moderation::GetModeratorsRequest::broadcaster_id("1234");
///
/// helix::make_stream(req, &token, &client, std::collections::VecDeque::from).try_collect::<Vec<_>>().await?
/// # ;
/// # Ok(())
/// # }
/// ```
pub fn make_stream<
    'a,
    C: crate::HttpClient + Send + Sync,
    T: TwitchToken + Send + Sync + ?Sized + Send + Sync,
    // FIXME: Why does this have to be clone and debug?
    Req: super::Request
        + super::RequestGet
        + super::Paginated
        + Clone
        + std::fmt::Debug
        + Send
        + Sync
        + 'a,
    // FIXME: this 'a seems suspicious
    Item: Send + 'a,
>(
    req: Req,
    token: &'a T,
    client: &'a super::HelixClient<'a, C>,
    fun: impl Fn(<Req as super::Request>::Response) -> std::collections::VecDeque<Item>
        + Send
        + Sync
        + Copy
        + 'static,
) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<Item, ClientError<C>>> + 'a + Send>>
where
    // FIXME: This clone is bad. I want to be able to return the data, but not in a way that limits the response to be Default
    // I also want to keep allocations low, so std::mem::take is perfect, but that makes get_next not work optimally.
    <Req as super::Request>::Response: Send + Sync + std::fmt::Debug + Clone,
{
    enum StateMode<Req: super::Request + super::RequestGet, Item> {
        /// A request needs to be done.
        Req(Option<Req>),
        /// We have made a request, now working through the data
        Cont(
            super::Response<Req, <Req as super::Request>::Response>,
            std::collections::VecDeque<Item>,
        ),
        Next(Option<super::Response<Req, <Req as super::Request>::Response>>),
        /// The operation failed, allowing no further processing
        Failed,
    }

    impl<Req: super::Request + super::RequestGet, Item> StateMode<Req, Item> {
        fn take_initial(&mut self) -> Req {
            match self {
                StateMode::Req(ref mut r) if r.is_some() => std::mem::take(r).expect("oops"),
                _ => todo!("hmmm"),
            }
        }

        fn take_next(&mut self) -> super::Response<Req, <Req as super::Request>::Response> {
            match self {
                StateMode::Next(ref mut r) if r.is_some() => std::mem::take(r).expect("oops"),
                _ => todo!("hmmm"),
            }
        }
    }

    struct State<
        'a,
        C: crate::HttpClient,
        T: TwitchToken + Send + Sync + ?Sized,
        Req: super::Request + super::RequestGet,
        Item,
    > {
        mode: StateMode<Req, Item>,
        client: &'a HelixClient<'a, C>,
        token: &'a T,
    }

    impl<
            C: crate::HttpClient,
            T: TwitchToken + Send + Sync + ?Sized,
            Req: super::Request + super::RequestGet + super::Paginated,
            Item,
        > State<'_, C, T, Req, Item>
    {
        /// Process a request, with a given deq
        fn process(
            mut self,
            r: super::Response<Req, <Req as super::Request>::Response>,
            d: std::collections::VecDeque<Item>,
        ) -> Self {
            self.mode = StateMode::Cont(r, d);
            self
        }

        fn failed(mut self) -> Self {
            self.mode = StateMode::Failed;
            self
        }

        /// get the next
        fn get_next(mut self) -> Self {
            match self.mode {
                StateMode::Cont(r, d) => {
                    assert!(d.is_empty());
                    self.mode = StateMode::Next(Some(r));
                    self
                }
                _ => panic!("oops"),
            }
        }
    }
    let statemode = StateMode::Req(Some(req));
    let state = State {
        mode: statemode,
        client,
        token,
    };
    futures::stream::unfold(state, move |mut state: State<_, _, _, _>| async move {
        match state.mode {
            StateMode::Req(Some(_)) => {
                let req = state.mode.take_initial();
                let f = state.client.req_get(req, state.token);
                let resp = match f.await {
                    Ok(resp) => resp,
                    Err(e) => return Some((Err(e), state.failed())),
                };
                let mut deq = fun(resp.data.clone());
                deq.pop_front().map(|d| (Ok(d), state.process(resp, deq)))
            }
            StateMode::Cont(_, ref mut deq) => {
                if let Some(d) = deq.pop_front() {
                    if deq.is_empty() {
                        Some((Ok(d), state.get_next()))
                    } else {
                        Some((Ok(d), state))
                    }
                } else {
                    // New request returned empty.
                    None
                }
            }
            StateMode::Next(Some(_)) => {
                let resp = state.mode.take_next();
                let f = resp.get_next(state.client, state.token);
                let resp = match f.await {
                    Ok(Some(resp)) => resp,
                    Ok(None) => return None,
                    Err(e) => return Some((Err(e), state.failed())),
                };
                let mut deq = fun(resp.data.clone());
                deq.pop_front().map(|d| (Ok(d), state.process(resp, deq)))
            }
            _ => todo!("failed to process request"),
        }
    })
    .boxed()
}
