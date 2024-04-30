use std::boxed::Box;


#[derive(Debug, Clone, Display, Default)]
struct RespError {
  kind: ErrorKind,
  source: Box<dyn Error>,
  message: Option<String>,
}

#[derive(Debug)]
// struct RespCreate

impl Error for RespError {}

#[derive(Debug, Clone, Default)]
pub enum ErrorKind {
  Forbidden,
  UnknownToken,
  BadJSON,
  NotJSON,
  NotFound,
  LimitExceeded,
  UserInUse,
  InvalidUsername,
  RoomInUse,
  BadPagination,
  ThreepidInUse,
  ThreepidNotFound,
  ServerNotTrusted

}