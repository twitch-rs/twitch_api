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
        login: types::UserName,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(
            helix::users::GetUsersRequest::builder()
                .login(vec![login])
                .build(),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Get [User](helix::users::User) from user id
    pub async fn get_user_from_id<T>(
        &'a self,
        id: types::UserId,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(
            helix::users::GetUsersRequest::builder()
                .id(vec![id])
                .build(),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters login
    pub async fn get_channel_from_login<T>(
        &'a self,
        login: types::UserName,
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
        id: types::UserId,
        token: &T,
    ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(
            helix::channels::GetChannelInformationRequest::builder()
                .broadcaster_id(id)
                .build(),
            token,
        )
        .await
        .map(|response| response.first())
    }
}

/*
    pub async fn get_xxxx<T>(&'a self, _:_, token: &T) -> Result<_, ClientError<'a, C>> where T: TwitchToken + ?Sized {todo!()}
*/
