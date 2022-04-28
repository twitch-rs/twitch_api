//! Convenience functions for [HelixClient]

use crate::helix::{self, ClientRequestError, HelixClient};
use crate::types;
use twitch_oauth2::TwitchToken;

type ClientError<'a, C> = ClientRequestError<<C as crate::HttpClient<'a>>::Error>;

// TODO: Consider moving these into the specific modules where the request is defined. Preferably backed by a macro

impl<'a, C: crate::HttpClient<'a> + Sync> HelixClient<'a, C> {
    /// Get [User](helix::users::User) from user login
    pub async fn get_user_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(
            helix::users::GetUsersRequest::builder()
                .login(vec![login.into()])
                .build(),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Get [User](helix::users::User) from user id
    pub async fn get_user_from_id<T>(
        &'a self,
        id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(
            helix::users::GetUsersRequest::builder()
                .id(vec![id.into()])
                .build(),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters login
    pub async fn get_channel_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(login, token).await? {
            self.get_channel_from_id(user.id, token).await
        } else {
            Ok(None)
        }
    }

    /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters id
    pub async fn get_channel_from_id<T>(
        &'a self,
        id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(
            helix::channels::GetChannelInformationRequest::builder()
                .broadcaster_id(id.into())
                .build(),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Search [Categories](helix::search::Category)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api2::helix;
    /// use futures::TryStreamExt;
    ///
    /// let categories: Vec<helix::search::Category> = client.search_categories("Fortnite", &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn search_categories<T>(
        &'a self,
        query: impl Into<String>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::search::Category, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::search::SearchCategoriesRequest::builder()
            .query(query.into())
            .build();
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Search [Channels](helix::search::Channel) via channel name or description
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api2::helix;
    /// use futures::TryStreamExt;
    ///
    /// let channel: Vec<helix::search::Channel> = client.search_channels("twitchdev", false, &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn search_channels<T>(
        &'a self,
        query: impl Into<String>,
        live_only: bool,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::search::Channel, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::search::SearchChannelsRequest::builder()
            .query(query.into())
            .live_only(live_only)
            .build();
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
    /// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api2::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// // Get the followers of channel "1234"
    /// let followers: Vec<helix::users::FollowRelationship> = client.get_follow_relationships(types::UserId::new("1234"), None, &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_follow_relationships<T>(
        &'a self,
        to_id: impl Into<Option<types::UserId>>,
        from_id: impl Into<Option<types::UserId>>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<helix::users::FollowRelationship, ClientError<'a, C>>>
                + Send
                + 'a,
        >,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::users::GetUsersFollowsRequest::builder()
            .to_id(to_id)
            .from_id(from_id)
            .first(100)
            .build();
        make_stream(req, token, self, |s| {
            std::collections::VecDeque::from(s.follow_relationships)
        })
    }

    /// Get authenticated users' followed [streams](helix::streams::Stream)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api2::helix;
    /// use futures::TryStreamExt;
    ///
    /// let channels: Vec<helix::streams::Stream> = client.get_followed_streams(&token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_followed_streams<T>(
        &'a self,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::streams::Stream, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        use futures::StreamExt;

        let user_id = match token
            .user_id()
            .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
        {
            Ok(t) => t,
            Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
        };
        let req = helix::streams::GetFollowedStreamsRequest::builder()
            .user_id(user_id)
            .build();
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get authenticated broadcasters' [subscribers](helix::subscriptions::BroadcasterSubscription)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api2::helix;
    /// use futures::TryStreamExt;
    ///
    /// let subs: Vec<helix::subscriptions::BroadcasterSubscription> = client.get_broadcaster_subscriptions(&token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_broadcaster_subscriptions<T>(
        &'a self,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<
            dyn futures::Stream<
                    Item = Result<
                        helix::subscriptions::BroadcasterSubscription,
                        ClientError<'a, C>,
                    >,
                > + 'a,
        >,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        use futures::StreamExt;

        let user_id = match token
            .user_id()
            .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
        {
            Ok(t) => t,
            Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
        };
        let req = helix::subscriptions::GetBroadcasterSubscriptionsRequest::builder()
            .broadcaster_id(user_id)
            .build();
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get all moderators in a channel [Get Moderators](helix::moderation::GetModeratorsRequest)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api2::helix;
    /// use futures::TryStreamExt;
    ///
    /// let moderators: Vec<helix::moderation::Moderator> = client.get_moderators_in_channel_from_id("twitchdev", &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_moderators_in_channel_from_id<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<helix::moderation::Moderator, ClientError<'a, C>>>
                + 'a,
        >,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::GetModeratorsRequest::builder()
            .broadcaster_id(broadcaster_id)
            .build();

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get a users, with login, follow count
    pub async fn get_total_followers_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<i64>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(login, token).await? {
            self.get_total_followers_from_id(user.id, token)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get a users, with id, follow count
    ///
    /// # Notes
    ///
    /// This returns zero if the user doesn't exist
    pub async fn get_total_followers_from_id<T>(
        &'a self,
        to_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<i64, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let resp = self
            .req_get(
                helix::users::GetUsersFollowsRequest::builder()
                    .from_id(Some(to_id.into()))
                    .build(),
                token,
            )
            .await?;

        Ok(resp.data.total)
    }

    /// Get games by ID. Can only be at max 100 ids.
    pub async fn get_games_by_id<T>(
        &'a self,
        ids: &[types::CategoryId],
        token: &T,
    ) -> Result<std::collections::HashMap<types::CategoryId, helix::games::Game>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        if ids.len() > 100 {
            return Err(ClientRequestError::Custom("too many IDs, max 100".into()));
        }

        let resp = self
            .req_get(
                helix::games::GetGamesRequest::builder()
                    .id(ids.to_vec())
                    .build(),
                token,
            )
            .await?;

        Ok(resp
            .data
            .into_iter()
            .map(|g: helix::games::Game| (g.id.clone(), g))
            .collect())
    }

    /// Block a user
    pub async fn block_user<T>(
        &'a self,
        target_user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::users::BlockUser, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        Ok(self
            .req_put(
                helix::users::BlockUserRequest::builder()
                    .target_user_id(target_user_id)
                    .build(),
                helix::EmptyBody,
                token,
            )
            .await?
            .data)
    }

    /// Unblock a user
    pub async fn unblock_user<T>(
        &'a self,
        target_user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::users::UnblockUser, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        Ok(self
            .req_delete(
                helix::users::UnblockUserRequest::builder()
                    .target_user_id(target_user_id)
                    .build(),
                token,
            )
            .await?
            .data)
    }

    // FIXME: Example should use https://github.com/Emilgardis/twitch_api2/issues/162
    /// Get all scheduled streams in a channel.
    ///
    /// # Notes
    ///
    /// Make sure to limit the data here using [`try_take_while`](futures::stream::TryStreamExt::try_take_while), otherwise this will never end on recurring scheduled streams.
    ///
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api2::helix;
    /// use futures::TryStreamExt;
    ///
    /// let schedule: Vec<helix::schedule::Segment> = client
    ///     .get_channel_schedule("twitchdev", &token)
    ///     .try_take_while(|s| {
    ///         futures::future::ready(Ok(!s.start_time.as_str().starts_with("2021-10")))
    ///     })
    ///     .try_collect()
    ///     .await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_channel_schedule<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::schedule::Segment, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::schedule::GetChannelStreamScheduleRequest::builder()
            .broadcaster_id(broadcaster_id)
            .build();

        make_stream(req, token, self, |broadcasts| broadcasts.segments.into())
    }

    /// Get all global emotes
    pub async fn get_global_emotes<T>(
        &'a self,
        token: &T,
    ) -> Result<Vec<helix::chat::GlobalEmote>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetGlobalEmotesRequest::builder().build();
        Ok(self.req_get(req, token).await?.data)
    }

    /// Get channel emotes in channel with user id
    pub async fn get_channel_emotes_from_id<T>(
        &'a self,
        user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Vec<helix::chat::ChannelEmote>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetChannelEmotesRequest::builder()
            .broadcaster_id(user_id)
            .build();
        Ok(self.req_get(req, token).await?.data)
    }

    /// Get channel emotes in channel with user login
    pub async fn get_channel_emotes_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<Vec<helix::chat::ChannelEmote>>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(login, token).await? {
            self.get_channel_emotes_from_id(user.id, token)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get emotes in emote set
    pub async fn get_emote_sets<T>(
        &'a self,
        emote_sets: &[types::EmoteSetId],
        token: &T,
    ) -> Result<Vec<helix::chat::get_emote_sets::Emote>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetEmoteSetsRequest::builder()
            .emote_set_id(emote_sets.to_owned())
            .build();
        Ok(self.req_get(req, token).await?.data)
    }
}

/// Make a paginate-able request into a stream
///
/// # Examples
///
/// ```rust, no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, twitch_api2::client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
/// use twitch_api2::helix;
/// use futures::TryStreamExt;
///
/// let req = helix::moderation::GetModeratorsRequest::builder()
/// .broadcaster_id("1234")
/// .build();
///
/// helix::make_stream(req, &token, &client, std::collections::VecDeque::from).try_collect::<Vec<_>>().await?
/// # ;
/// # Ok(())
/// # }
/// ```
pub fn make_stream<
    'a,
    C: crate::HttpClient<'a> + Send + Sync,
    T: TwitchToken + ?Sized + Send + Sync,
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
) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<Item, ClientError<'a, C>>> + 'a + Send>>
where
    // FIXME: This clone is bad. I want to be able to return the data, but not in a way that limits the response to be Default
    // I also want to keep allocations low, so std::mem::take is perfect, but that makes get_next not work optimally.
    <Req as super::Request>::Response: Send + Sync + std::fmt::Debug + Clone,
{
    use futures::StreamExt;
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
        C: crate::HttpClient<'a>,
        T: TwitchToken + ?Sized,
        Req: super::Request + super::RequestGet,
        Item,
    > {
        mode: StateMode<Req, Item>,
        client: &'a HelixClient<'a, C>,
        token: &'a T,
    }

    impl<
            'a,
            C: crate::HttpClient<'a>,
            T: TwitchToken + ?Sized,
            Req: super::Request + super::RequestGet + super::Paginated,
            Item,
        > State<'a, C, T, Req, Item>
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
