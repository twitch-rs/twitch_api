use super::*;
use std::convert::TryInto;

/// Custom response retrieved from endpoint, used for specializing responses
#[non_exhaustive]
pub struct CustomResponse<'d, R, D>
where
    R: Request,
    D: 'd, {
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    pub pagination: Option<Cursor>,
    /// The request that was sent, used for [pagination](Paginated).
    pub request: Option<R>,
    /// Response would return this many results if fully paginated. Sometimes this is not emmitted or correct for this purpose, in those cases, this value will be `None`.
    pub total: Option<i64>,
    /// Other fields that are part of the response, but unknown.
    ///
    /// Unfortunately, this [can't be borrowed](https://github.com/serde-rs/json/issues/599).
    pub other: serde_json::Map<String, serde_json::Value>,
    /// The owned data. Use [`CustomResponse::data()`] to deserialize.
    pub raw_data: Box<serde_json::value::RawValue>,
    pd: std::marker::PhantomData<&'d D>,
}

impl<'d, R, D> CustomResponse<'d, R, D>
where
    R: Request,
    D: 'd + serde::Deserialize<'d>,
{
    /// Deserialize the data
    pub fn data(&'d self) -> Result<D, serde_json::Error> {
        serde_json::from_str(self.raw_data.get())
    }
}

impl<'a, C: crate::HttpClient<'a>> HelixClient<'a, C> {
    /// Request on a valid [`RequestGet`] endpoint, with the ability to return borrowed data and specific fields.
    pub async fn req_get_custom<'d, R, D, T>(
        &'a self,
        request: R,
        token: &T,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestGet,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?
            .into_response_bytes()
            .await?;
        {
            let request = Some(request);
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestGetError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            //eprintln!("\n\nmessage is ------------ {} ------------", text);
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestGetError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                }
                .into());
            }
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestGetError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request,
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestPost`] endpoint, with the ability to return borrowed data and specific fields.
    pub async fn req_post_custom<'d, R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestPost + RequestPost<Body = B>,
        B: HelixRequestBody,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?
            .into_response_bytes()
            .await?;
        {
            let request = Some(request);
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestPostError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            //eprintln!("\n\nmessage is ------------ {} ------------", text);
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestPostError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPostError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request,
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestPatch`] endpoint, with the ability to return borrowed data and specific fields.
    ///
    /// # Notes
    ///
    /// This is probably not useful, as `PATCH` endpoints do not usually return json
    pub async fn req_patch_custom<'d, R, B, D, T, F>(
        &'a self,
        request: R,
        body: B,
        token: &T,
        function: F,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestPatch + RequestPatch<Body = B>,
        B: HelixRequestBody,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
        F: Fn(&R, &http::Uri, &str, http::StatusCode) -> Result<(), HelixRequestPatchError>,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?
            .into_response_bytes()
            .await?;
        {
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestPatchError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestPatchError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            function(&request, uri, text, response.status())?;
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPatchError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request: Some(request),
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestDelete`] endpoint, with the ability to return borrowed data and specific fields.
    ///
    /// # Notes
    ///
    /// This is probably not useful, as `DELETE` endpoints do not usually return json
    pub async fn req_delete_custom<'d, R, D, T, F>(
        &'a self,
        request: R,
        token: &T,
        function: F,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestDelete,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
        F: Fn(&R, &http::Uri, &str, http::StatusCode) -> Result<(), HelixRequestDeleteError>,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?
            .into_response_bytes()
            .await?;
        {
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestDeleteError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestDeleteError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            function(&request, uri, text, response.status())?;
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPatchError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request: Some(request),
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestPut`] endpoint, with the ability to return borrowed data and specific fields.
    ///
    /// # Notes
    ///
    /// This is probably not useful, as `PUT` endpoints do not usually return json
    pub async fn req_put_custom<'d, R, B, D, T, F>(
        &'a self,
        request: R,
        body: B,
        token: &T,
        function: F,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestPut + RequestPut<Body = B>,
        B: HelixRequestBody,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
        F: Fn(&R, &http::Uri, &str, http::StatusCode) -> Result<(), HelixRequestDeleteError>,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?
            .into_response_bytes()
            .await?;
        {
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestPutError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestPutError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            function(&request, uri, text, response.status())?;
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPatchError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request: Some(request),
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }
}
