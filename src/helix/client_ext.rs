//! Convenience functions for [HelixClient]

use crate::helix::{self, ClientRequestError, HelixClient};
use crate::types;
use twitch_oauth2::TwitchToken;

type ClientError<'a, C> = ClientRequestError<<C as crate::HttpClient<'a>>::Error>;

// TODO: Consider moving these into the specific modules where the request is defined. Preferably backed by a macro

impl<'a, C: crate::HttpClient<'a>> HelixClient<'a, C> {
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
    pub async fn search_categories<T>(
        &'a self,
        query: impl Into<String>,
        token: &T,
    ) -> Result<Vec<helix::search::Category>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let mut result = vec![];

        let mut resp = self
            .req_get(
                helix::search::SearchCategoriesRequest::builder()
                    .query(query.into())
                    .build(),
                token,
            )
            .await?;
        result.extend(std::mem::take(&mut resp.data));
        while let Some(resp_new) = resp.get_next(&self, token).await? {
            resp = resp_new;
            result.extend(std::mem::take(&mut resp.data));
        }

        Ok(result)
    }

    /// Search [Channels](helix::search::Channel)
    pub async fn search_channels<T>(
        &'a self,
        query: impl Into<String>,
        live_only: bool,
        token: &T,
    ) -> Result<Vec<helix::search::Channel>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let mut result = vec![];

        let mut resp = self
            .req_get(
                helix::search::SearchChannelsRequest::builder()
                    .query(query.into())
                    .live_only(live_only)
                    .build(),
                token,
            )
            .await?;
        result.extend(std::mem::take(&mut resp.data));
        while let Some(resp_new) = resp.get_next(&self, token).await? {
            resp = resp_new;
            result.extend(std::mem::take(&mut resp.data));
        }

        Ok(result)
    }

    /// Get authenticated users followed [streams](helix::streams::Stream)
    pub async fn get_followed_streams<T>(
        &'a self,
        token: &twitch_oauth2::UserToken,
    ) -> Result<Vec<helix::streams::Stream>, ClientError<'a, C>>
    {
        let mut result = vec![];

        let mut resp = self
            .req_get(
                helix::streams::GetFollowedStreamsRequest::builder()
                .user_id(token.user_id.clone())
                    .build(),
                token,
            )
            .await?;
        result.extend(std::mem::take(&mut resp.data));
        while let Some(resp_new) = resp.get_next(&self, token).await? {
            resp = resp_new;
            result.extend(std::mem::take(&mut resp.data));
        }

        Ok(result)
    }

    /// Get all moderators in a channel [Channels](helix::search::Channel)
    pub async fn get_moderators_in_channel_from_id<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Vec<helix::moderation::Moderator>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let mut result = vec![];

        let mut resp = self
            .req_get(
                helix::moderation::GetModeratorsRequest::builder()
                    .broadcaster_id(broadcaster_id)
                    .build(),
                token,
            )
            .await?;
        result.extend(std::mem::take(&mut resp.data));
        while let Some(resp_new) = resp.get_next(&self, token).await? {
            resp = resp_new;
            result.extend(std::mem::take(&mut resp.data));
        }

        Ok(result)
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

    /// Block a user
    pub async fn block_user<T>(
        &'a self,
        target_user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::users::BlockUser, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_put(
            helix::users::BlockUserRequest::builder()
                .target_user_id(target_user_id)
                .build(),
            helix::EmptyBody,
            token,
        )
        .await
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
        self.req_delete(
            helix::users::UnblockUserRequest::builder()
                .target_user_id(target_user_id)
                .build(),
            token,
        )
        .await
    }
}

/*
    pub async fn get_xxxx<T>(&'a self, _:_, token: &T) -> Result<_, ClientError<'a, C>> where T: TwitchToken + ?Sized {todo!()}
*/
