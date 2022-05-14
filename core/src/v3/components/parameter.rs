use serde::{Deserialize, Serialize};

pub enum Parameter {
    Query(QueryParameter),
    Header(HeaderParameter),
    Path(PathParameter),
    Cookie(CookieParameter),
}

pub struct QueryParameter {}

pub struct HeaderParameter {}

pub struct PathParameter {}

pub struct CookieParameter {}
