//! Types used for paginated requests

use std::fmt::Debug;

use super::{Cursor, Request};

/// The state of a paginated request
#[derive(PartialEq, Eq, Debug)]
pub struct PaginationState<R: Request> {
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    pub cursor: Option<Cursor>,
    /// The request that was sent, used for [pagination](super::Paginated).
    pub request: Option<R>,
    /// Response would return this many results if fully paginated. Sometimes this is not emmitted or correct for this purpose, in those cases, this value will be `None`.
    pub total: Option<i64>,
}

/// Data used to keep track of the pagination for a request.
pub trait PaginationData<R: Request> {
    /// Create the state for a response and its request
    fn new(cursor: Option<Cursor>, request: Option<R>, total: Option<i64>) -> Self;

    /// Get the total number of items in the data
    fn total(&self) -> Option<i64>;
}

impl<R: Request> PaginationData<R> for () {
    fn new(_cursor: Option<Cursor>, _request: Option<R>, _total: Option<i64>) -> Self {}

    fn total(&self) -> Option<i64> { None }
}

impl<R: Request> PaginationData<R> for PaginationState<R> {
    fn new(cursor: Option<Cursor>, request: Option<R>, total: Option<i64>) -> Self {
        Self {
            cursor,
            request,
            total,
        }
    }

    fn total(&self) -> Option<i64> { self.total }
}
