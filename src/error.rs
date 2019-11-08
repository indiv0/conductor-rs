// Copyright 2018 Netflix, Inc.
// Copyright 2019 Nikita Pekin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::http::ErrorResponse;
use http::StatusCode;
use std::error::Error as StdError;
use std::fmt;

/// Wrapper around `std::Result`.
pub type Result<T> = std::result::Result<T, Error>;

type Source = Box<dyn StdError + Send + Sync>;

/// Represents errors that can occur while performing Conductor API operations.
pub struct Error {
    kind: Kind,
    source: Option<Source>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Kind {
    /// An `http::Error` that occurred while handling an HTTP connection.
    Http,
    /// A `hyper::Error` that occurred while handling an HTTP stream.
    Hyper,
    /// Invalid argument was provided to a function.
    InvalidArgument(InvalidArgument),
    /*
    /// An `io::Error` that occurred while handling IO.
    Io,
    */
    /// A `serde_json::Error` that occurred while (de)serializing JSON.
    Json,
    /// Conductor server returned an unexpected status code.
    Server(Server),
}

#[derive(Debug, PartialEq)]
pub(crate) enum InvalidArgument {
    /// User provided an empty `Vec<TaskDef>`.
    EmptyTaskDefList,
}

// TODO: do we really want `PartialEq` on this now that we implement ErrorResponse?
#[derive(Debug, PartialEq)]
pub(crate) enum Server {
    /// TODO: docs
    ErrorResponse(ErrorResponse),
    /// Server responded with an unexpected status code.
    UnexpectedStatus(StatusCode),
}

impl Error {
    fn new(kind: Kind) -> Self {
        Self { kind, source: None }
    }

    fn with<S: Into<Source>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
        self
    }

    #[allow(dead_code)]
    pub(crate) const fn kind(&self) -> &Kind {
        &self.kind
    }

    pub(crate) fn new_invalid_argument(invalid_argument: InvalidArgument) -> Self {
        Self::new(Kind::InvalidArgument(invalid_argument))
    }

    pub(crate) fn new_empty_task_def_list() -> Self {
        Self::new_invalid_argument(InvalidArgument::EmptyTaskDefList)
    }

    pub(crate) fn new_server(server: Server) -> Self {
        Self::new(Kind::Server(server))
    }

    pub(crate) fn new_error_response(error_response: ErrorResponse) -> Self {
        Self::new_server(Server::ErrorResponse(error_response))
    }

    pub(crate) fn new_unexpected_status(status_code: StatusCode) -> Self {
        Self::new_server(Server::UnexpectedStatus(status_code))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("Error");
        f.field(&self.kind);
        if let Some(ref source) = self.source {
            f.field(source);
        }
        f.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())?;

        match self.kind {
            Kind::Server(Server::UnexpectedStatus(ref code)) => write!(f, ", status = {}", code)?,
            Kind::Server(Server::ErrorResponse(ref err)) => write!(f, ", error response = {}", err)?,
            _ => {},
        }

        if let Some(ref source) = self.source {
            write!(f, ": {}", source)?
        }

        Ok(())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::Http => "HTTP connection error",
            Kind::Hyper => "HTTP stream error",
            Kind::InvalidArgument(InvalidArgument::EmptyTaskDefList) => {
                "Empty task def list provided"
            }
            /*
            Kind::Io => "IO error",
            */
            Kind::Json => "JSON error",
            Kind::Server(Server::UnexpectedStatus(_)) => "Unexpected status code from server",
            Kind::Server(Server::ErrorResponse(_)) => "Server returned an error",
        }
    }

    #[allow(trivial_casts)]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn StdError + 'static))
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Self {
        Self::new(Kind::Http).with(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self::new(Kind::Hyper).with(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::new(Kind::Json).with(err)
    }
}

#[doc(hidden)]
trait AssertSendSync: Send + Sync + 'static {}
#[doc(hidden)]
impl AssertSendSync for Error {}

#[cfg(test)]
mod tests {
    use crate::error::{Error, Kind};
    use matches::assert_matches;
    use std::error::Error as StdError;
    use std::io;

    #[test]
    fn when_new_error_then_kind_field_set() {
        let err = Error::new(Kind::Json);
        assert_matches!(err.kind, Kind::Json);
    }

    #[test]
    fn when_new_error_then_source_is_none() {
        let err = Error::new(Kind::Json);
        assert!(err.source.is_none());
    }

    #[test]
    fn given_error_with_kind_when_debug_fmt_then_print_tuple_with_kind() {
        let err = Error::new(Kind::Json);
        let string = format!("{:?}", err);
        assert_eq!(string, "Error(Json)");
    }

    #[test]
    fn given_error_with_kind_when_display_fmt_then_print_description() {
        let err = Error::new(Kind::Json);
        let string = format!("{}", err);
        assert_eq!(string, "JSON error");
    }

    #[test]
    fn given_error_with_source_when_debug_fmt_then_print_tuple_with_source() {
        let err = Error::new(Kind::Json).with(new_io_error());
        let string = format!("{:?}", err);
        assert_eq!(
            string,
            "Error(Json, Custom { kind: Other, error: \"oh no!\" })"
        );
    }

    #[test]
    fn given_error_with_source_when_display_fmt_then_print_description_with_source() {
        let err = Error::new(Kind::Json).with(new_io_error());
        let string = format!("{}", err);
        assert_eq!(string, "JSON error: oh no!");
    }

    #[test]
    fn given_error_with_no_source_when_get_source_then_return_none() {
        let err = Error::new(Kind::Json);
        assert_matches!(err.source(), None);
    }

    #[test]
    fn given_error_with_source_when_get_source_then_return_source() {
        let err = Error::new(Kind::Json).with(new_io_error());
        let source = err.source();
        assert_matches!(source, Some(source) if source.description() == "oh no!");
    }

    fn new_io_error() -> io::Error {
        io::Error::new(io::ErrorKind::Other, "oh no!")
    }
}
