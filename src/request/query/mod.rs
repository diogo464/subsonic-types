mod de;
mod ser;

type Result<T, E = QueryError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct QueryError {
    message: String,
}

impl QueryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for QueryError {}

impl serde::ser::Error for QueryError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::new(msg.to_string())
    }
}

impl serde::de::Error for QueryError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::new(msg.to_string())
    }
}

pub use de::from_query;
pub use ser::to_query;

mod prototype {
    use std::borrow::Cow;

    pub type Result<T, E = QueryParseError> = std::result::Result<T, E>;

    pub type QueryKey<'a> = Cow<'a, str>;
    pub type QueryValue<'a> = Option<Cow<'a, str>>;

    #[derive(Debug)]
    pub struct QueryValueParseError(String);

    impl std::error::Error for QueryValueParseError {}

    impl std::fmt::Display for QueryValueParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl QueryValueParseError {
        pub fn message(msg: impl ToString) -> Self {
            Self(msg.to_string())
        }

        pub fn duplicate_value() -> Self {
            Self::message("duplicate value")
        }

        pub fn empty_value() -> Self {
            Self::message("empty value")
        }
    }

    #[derive(Debug)]
    pub enum QueryParseError {
        InvalidValue {
            key: String,
            error: QueryValueParseError,
        },
        UnknownKey {
            key: String,
        },
        InvalidQueryString {
            query: String,
            message: String,
        },
    }

    impl QueryParseError {
        pub fn invalid_value(key: impl ToString, error: QueryValueParseError) -> Self {
            Self::InvalidValue {
                key: key.to_string(),
                error,
            }
        }

        pub fn unknown_key(key: impl ToString) -> Self {
            Self::UnknownKey {
                key: key.to_string(),
            }
        }

        pub fn invalid_query_string(query: impl ToString, message: impl ToString) -> Self {
            Self::InvalidQueryString {
                query: query.to_string(),
                message: message.to_string(),
            }
        }
    }

    pub struct QueryPair<'a> {
        key: QueryKey<'a>,
        value: QueryValue<'a>,
    }

    pub enum ConsumeStatus<'a> {
        Consumed,
        Ignored(QueryPair<'a>),
    }

    pub trait QueryAccumulator: Default {
        type Output: Sized;

        fn consume<'a>(&mut self, pair: QueryPair<'a>) -> Result<ConsumeStatus<'a>>;
        fn finish(self) -> Result<Self::Output>;
    }

    pub trait QueryValueAccumulator: Default {
        type Output: Sized;

        fn consume<'a>(&mut self, value: QueryValue<'a>) -> Result<(), QueryValueParseError>;
        fn finish(self) -> Result<Self::Output, QueryValueParseError>;
    }

    pub trait FromQuery: Sized {
        type QueryAccumulator: QueryAccumulator<Output = Self>;
    }

    pub trait FromQueryValue: Sized {
        type QueryValueAccumulator: QueryValueAccumulator<Output = Self>;
    }

    // Implements a basic query iterator
    mod basic {
        use std::borrow::Cow;

        use super::{QueryPair, QueryParseError, QueryValue, Result};

        pub struct QueryIter<'a> {
            iter: std::str::Split<'a, char>,
        }

        impl<'a> QueryIter<'a> {
            fn new(query: &'a str) -> Self {
                Self {
                    iter: query.split('&'),
                }
            }
        }

        fn segment_to_query_pair<'a>(segment: &'a str) -> Result<QueryPair<'a>> {
            let decode_str = |s: &'a str| -> Cow<'a, str> {
                percent_encoding::percent_decode_str(s).decode_utf8_lossy()
            };
            let (key, value) = match segment.split_once('=') {
                Some((key, value)) => (key, Some(value)),
                None => (segment, None),
            };
            Ok(QueryPair {
                key: decode_str(key),
                value: value.map(decode_str),
            })
        }

        impl<'a> Iterator for QueryIter<'a> {
            type Item = Result<QueryPair<'a>>;

            fn next(&mut self) -> Option<Self::Item> {
                let segment = self.iter.next()?;
                Some(segment_to_query_pair(segment))
            }
        }

        pub fn parse_query<'a>(query: &'a str) -> impl Iterator<Item = Result<QueryPair<'a>>> + 'a {
            QueryIter::new(query)
        }
    }

    pub fn from_query<'a, T>(query: &str) -> Result<T>
    where
        T: FromQuery,
    {
        let mut accumulator = T::QueryAccumulator::default();
        for pair in basic::parse_query(query) {
            let pair = pair?;
            match accumulator.consume(pair)? {
                ConsumeStatus::Consumed => {}
                ConsumeStatus::Ignored(pair) => {
                    return Err(QueryParseError::UnknownKey {
                        key: pair.key.into_owned(),
                    })
                }
            }
        }
        accumulator.finish()
    }

    pub struct QueryValueAccumulatorFromStr<T> {
        value: Option<T>,
    }

    impl<T> Default for QueryValueAccumulatorFromStr<T> {
        fn default() -> Self {
            Self { value: None }
        }
    }

    impl<T, E> QueryValueAccumulator for QueryValueAccumulatorFromStr<T>
    where
        T: std::str::FromStr<Err = E>,
        E: std::fmt::Display,
    {
        type Output = T;

        fn consume<'a>(&mut self, value: QueryValue<'a>) -> Result<(), QueryValueParseError> {
            if self.value.is_some() {
                return Err(QueryValueParseError::duplicate_value());
            }
            let value = value.ok_or_else(QueryValueParseError::empty_value)?;
            let value = value.parse().map_err(QueryValueParseError::message)?;
            self.value = Some(value);
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, QueryValueParseError> {
            match self.value {
                Some(value) => Ok(value),
                None => Err(QueryValueParseError::empty_value()),
            }
        }
    }

    pub struct QueryValueAccumulatorOption<T>
    where
        T: FromQueryValue,
    {
        value: Option<<T as FromQueryValue>::QueryValueAccumulator>,
    }

    impl<T> Default for QueryValueAccumulatorOption<T>
    where
        T: FromQueryValue,
    {
        fn default() -> Self {
            Self { value: None }
        }
    }

    impl<T> QueryValueAccumulator for QueryValueAccumulatorOption<T>
    where
        T: FromQueryValue,
    {
        type Output = Option<T>;

        fn consume<'a>(&mut self, value: QueryValue<'a>) -> Result<(), QueryValueParseError> {
            let accumulator = self
                .value
                .get_or_insert_with(<T as FromQueryValue>::QueryValueAccumulator::default);
            accumulator.consume(value)?;
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, QueryValueParseError> {
            match self.value {
                Some(accumulator) => accumulator.finish().map(Some),
                None => Ok(None),
            }
        }
    }

    impl<T> FromQueryValue for Option<T>
    where
        T: FromQueryValue,
    {
        type QueryValueAccumulator = QueryValueAccumulatorOption<T>;
    }

    pub struct QueryValueAccumulatorVec<T> {
        values: Vec<T>,
    }

    impl<T> Default for QueryValueAccumulatorVec<T> {
        fn default() -> Self {
            Self { values: Vec::new() }
        }
    }

    impl<T> QueryValueAccumulator for QueryValueAccumulatorVec<T>
    where
        T: FromQueryValue,
    {
        type Output = Vec<T>;

        fn consume<'a>(&mut self, value: QueryValue<'a>) -> Result<(), QueryValueParseError> {
            let mut accumulator = <T as FromQueryValue>::QueryValueAccumulator::default();
            accumulator.consume(value)?;
            let value = accumulator.finish()?;
            self.values.push(value);
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, QueryValueParseError> {
            Ok(self.values)
        }
    }

    impl<T> FromQueryValue for Vec<T>
    where
        T: FromQueryValue,
    {
        type QueryValueAccumulator = QueryValueAccumulatorVec<T>;
    }

    pub struct QueryValueAccumulatorBool {
        value: Option<bool>,
    }

    impl Default for QueryValueAccumulatorBool {
        fn default() -> Self {
            Self { value: None }
        }
    }

    impl QueryValueAccumulator for QueryValueAccumulatorBool {
        type Output = bool;

        fn consume<'a>(&mut self, value: QueryValue<'a>) -> Result<(), QueryValueParseError> {
            if self.value.is_some() {
                return Err(QueryValueParseError::duplicate_value());
            }
            let value = match value {
                Some(value) => match value.as_ref() {
                    "true" | "1" => true,
                    "false" | "0" => false,
                    _ => return Err(QueryValueParseError::message("invalid boolean value")),
                },
                None => false,
            };
            self.value = Some(value);
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, QueryValueParseError> {
            match self.value {
                Some(value) => Ok(value),
                None => Err(QueryValueParseError::empty_value()),
            }
        }
    }

    macro_rules! impl_from_query_value_for_parse {
        ($($t:ty),*) => {
            $(
                impl FromQueryValue for $t {
                    type QueryValueAccumulator = QueryValueAccumulatorFromStr<Self>;
                }
            )*
        };
    }

    impl_from_query_value_for_parse!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, String
    );

    struct Nested {
        field_d: u32,
        field_e: Option<String>,
    }

    #[derive(Default)]
    struct NestedAccumulator {
        field_d: <u32 as FromQueryValue>::QueryValueAccumulator,
        field_e: <Option<String> as FromQueryValue>::QueryValueAccumulator,
    }

    impl QueryAccumulator for NestedAccumulator {
        type Output = Nested;

        fn consume<'a>(&mut self, pair: QueryPair<'a>) -> Result<ConsumeStatus<'a>> {
            match pair.key.as_ref() {
                "field_d" => {
                    self.field_d
                        .consume(pair.value)
                        .map_err(|e| QueryParseError::invalid_value("field_b", e))?;
                    Ok(ConsumeStatus::Consumed)
                }
                "field_e" => {
                    self.field_e
                        .consume(pair.value)
                        .map_err(|e| QueryParseError::invalid_value("field_e", e))?;
                    Ok(ConsumeStatus::Consumed)
                }
                _ => Ok(ConsumeStatus::Ignored(pair)),
            }
        }

        fn finish(self) -> Result<Self::Output> {
            let field_d = self
                .field_d
                .finish()
                .map_err(|e| QueryParseError::invalid_value("field_d", e))?;
            let field_e = self
                .field_e
                .finish()
                .map_err(|e| QueryParseError::invalid_value("field_e", e))?;
            Ok(Nested { field_d, field_e })
        }
    }

    impl FromQuery for Nested {
        type QueryAccumulator = NestedAccumulator;
    }

    struct Test {
        field_a: u32,
        field_b: String,
        field_c: Nested,
    }

    #[test]
    fn test_nested() {
        let query = "field_d=3&field_e=4";
        let test: Nested = from_query(query).unwrap();
        assert_eq!(test.field_d, 3);
        assert_eq!(test.field_e, Some("4".to_string()));

        let query = "field_d=3";
        let test: Nested = from_query(query).unwrap();
        assert_eq!(test.field_d, 3);
        assert_eq!(test.field_e, None);

        let query = "field_d=3&field_e=";
        let test: Nested = from_query(query).unwrap();
        assert_eq!(test.field_d, 3);
        assert_eq!(test.field_e, Some("".to_string()));
    }
}
