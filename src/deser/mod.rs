mod error;
mod format;
mod xml;

pub mod deserialize;
pub mod serialize;

use std::marker::PhantomData;

pub use deserialize::{Deserialize, Deserializer, SubsonicDeserializer};
pub use error::Error;
pub use format::Format;
pub use serialize::{Serialize, Serializer, SubsonicSerializer};

pub struct Impossible<Ok, Error> {
    ok: PhantomData<Ok>,
    error: PhantomData<Error>,
}
