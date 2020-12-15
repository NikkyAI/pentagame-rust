use derive_more::{Display, Error};
use serde::Serialize;

/*
UserError:
    displays user when something is not e.g. available as html for GET Requests
    returns template error.html

    Errors:
    ValidationError: Only returned for non-auth queries as everything else is API (POST) based
    InternalError: Something went really, really wrong

    codes:
        1: not found
        2: LOL
*/
// pub type WebResult<R> = Result<R, WebsocketError>;

#[derive(Debug, Error, Display)]
pub enum WebsocketError {
    #[display(fmt = "Internal Error")]
    InternalError {},
    #[display(fmt = "Failed to load message from JSON")]
    MessageFormatError {},
    #[display(fmt = "Unimplemented feature")]
    UnimplementedError {},
}

#[derive(Serialize)]
pub struct ErrorMessage<'a> {
    /*
    This represents

    0: InternalError
    1: MessageformatError
    u8::MAX: UnimplementedError
    */
    code: u8,
    message: &'a str,
}

impl<'a> ErrorMessage<'a> {
    pub fn text<'x>(error: WebsocketError) -> String {
        let (code, message) = match error {
            WebsocketError::InternalError { .. } => {
                (0, "Internal Error: Sorry for any caused inconvenience")
            }
            WebsocketError::MessageFormatError { .. } => {
                (1, "MessageError: Seems like your message couldn't be loaded from JSON")
            }
            WebsocketError::UnimplementedError { .. } => {
                (u8::MAX, "Unimplemented: The action you tried to use is either implemented/ supported at them moment")
            }
        };

        return serde_json::to_string(&ErrorMessage { code, message })
            .expect("The creation of websocket error messages failed");
    }
}

impl From<serde_json::Error> for WebsocketError {
    fn from(_: serde_json::Error) -> Self {
        return WebsocketError::MessageFormatError {};
    }
}

impl From<diesel::result::Error> for WebsocketError {
    fn from(why: diesel::result::Error) -> Self {
        eprintln!("Diesel Execution failed: {:?}", why);
        return WebsocketError::InternalError {};
    }
}

// Errors with 'static' outcomes that don't feature failure specific fields may be cached here
lazy_static! {
    pub static ref INTERNAL_ERROR_MESSAGE: String =
        ErrorMessage::text(WebsocketError::InternalError {});
    pub static ref MESSAGE_FORMAT_ERROR: String =
        ErrorMessage::text(WebsocketError::MessageFormatError {});
    pub static ref UNIMPLEMENTED_ERROR: String =
        ErrorMessage::text(WebsocketError::UnimplementedError {});
}
