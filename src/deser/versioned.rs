use crate::common::Version;

use super::{Format, MaybeSerialize, SubsonicSerialize};

#[derive(Default)]
pub struct Versioned<T, const VERSION: u32>(std::marker::PhantomData<T>);

impl<'s, T, const VERSION: u32> SubsonicSerialize<'s> for Versioned<T, VERSION>
where
    T: SubsonicSerialize<'s> + 's,
{
    type Input = T::Input;
    type Output = MaybeSerialize<T::Output>;

    fn prepare(input: Self::Input, format: Format, version: Version) -> Self::Output {
        if version.as_u32() >= VERSION {
            MaybeSerialize::serialize(T::prepare(input, format, version))
        } else {
            MaybeSerialize::none()
        }
    }
}
