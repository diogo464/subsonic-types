use std::{borrow::Cow, fmt::Write};

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

impl std::fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue { key, error } => {
                write!(f, "invalid value for key {}: {}", key, error)
            }
            Self::UnknownKey { key } => write!(f, "unknown key: {}", key),
            Self::InvalidQueryString { query, message } => {
                write!(f, "invalid query string {}: {}", query, message)
            }
        }
    }
}

impl std::error::Error for QueryParseError {}

pub struct QueryPair<'a> {
    pub key: QueryKey<'a>,
    pub value: QueryValue<'a>,
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

pub trait QueryBuilder {
    fn emit<K, V>(&mut self, key: K, value: Option<V>)
    where
        K: std::fmt::Display,
        V: std::fmt::Display;

    fn emit_key<K>(&mut self, key: K)
    where
        K: std::fmt::Display;

    fn emit_key_value<K, V>(&mut self, key: K, value: V)
    where
        K: std::fmt::Display,
        V: std::fmt::Display;
}

pub trait ToQuery {
    fn to_query_builder<B>(&self, builder: &mut B)
    where
        B: QueryBuilder;
}

pub trait ToQueryValue {
    fn to_query_builder<B>(&self, builder: &mut B, encode_as: &str)
    where
        B: QueryBuilder;
}

#[derive(Default)]
pub struct QueryBuilderString {
    query: String,
    buffer: String,
}

impl QueryBuilderString {
    /// <https://url.spec.whatwg.org/#query-percent-encode-set≥
    const QUERY_PERCENT_ENCODE_SET: percent_encoding::AsciiSet = percent_encoding::CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>');

    fn into_query(self) -> String {
        self.query
    }
}

impl QueryBuilder for QueryBuilderString {
    fn emit<K, V>(&mut self, key: K, value: Option<V>)
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        match value {
            Some(value) => self.emit_key_value(key, value),
            None => self.emit_key(key),
        }
    }

    fn emit_key<K>(&mut self, key: K)
    where
        K: std::fmt::Display,
    {
        if !self.query.is_empty() {
            self.query.push('&');
        }
        self.buffer.clear();
        write!(self.buffer, "{}", key).unwrap();
        self.query.extend(percent_encoding::utf8_percent_encode(
            self.buffer.as_str(),
            &Self::QUERY_PERCENT_ENCODE_SET,
        ));
    }

    fn emit_key_value<K, V>(&mut self, key: K, value: V)
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        self.emit_key(key);
        self.query.push('=');
        self.buffer.clear();
        write!(self.buffer, "{}", value).unwrap();
        self.query.extend(percent_encoding::utf8_percent_encode(
            self.buffer.as_str(),
            &Self::QUERY_PERCENT_ENCODE_SET,
        ));
    }
}

// Implements a basic query iterator
mod basic {
    use std::borrow::Cow;

    use super::{QueryPair, Result};

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
            if !segment.is_empty() {
                Some(segment_to_query_pair(segment))
            } else {
                None
            }
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

pub fn to_query<T>(value: &T) -> String
where
    T: ToQuery,
{
    let mut builder = QueryBuilderString::default();
    value.to_query_builder(&mut builder);
    builder.into_query()
}

impl<T> ToQueryValue for &T
where
    T: ToQueryValue,
{
    fn to_query_builder<B>(&self, builder: &mut B, encode_as: &str)
    where
        B: QueryBuilder,
    {
        <T as ToQueryValue>::to_query_builder(self, builder, encode_as);
    }
}

impl ToQueryValue for bool {
    fn to_query_builder<B>(&self, builder: &mut B, encode_as: &str)
    where
        B: QueryBuilder,
    {
        if *self {
            builder.emit_key(encode_as);
        }
    }
}

impl<T> ToQueryValue for Option<T>
where
    T: ToQueryValue,
{
    fn to_query_builder<B>(&self, builder: &mut B, encode_as: &str)
    where
        B: QueryBuilder,
    {
        if let Some(value) = self.as_ref() {
            value.to_query_builder(builder, encode_as);
        }
    }
}

impl<T> ToQueryValue for Vec<T>
where
    T: ToQueryValue,
{
    fn to_query_builder<B>(&self, builder: &mut B, encode_as: &str)
    where
        B: QueryBuilder,
    {
        for v in self.iter() {
            v.to_query_builder(builder, encode_as);
        }
    }
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
            None => true,
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

impl FromQueryValue for bool {
    type QueryValueAccumulator = QueryValueAccumulatorBool;
}

#[macro_export]
macro_rules! impl_from_query_value_for_parse {
        ($($t:ty),*) => {
            $(
                impl $crate::query::FromQueryValue for $t {
                    type QueryValueAccumulator = $crate::query::QueryValueAccumulatorFromStr<Self>;
                }
            )*
        };
    }

#[macro_export]
macro_rules! impl_to_query_value_for_display {
    ($($t:ty),*) => {
        $(
            impl $crate::query::ToQueryValue for $t {
                fn to_query_builder<B>(&self, builder: &mut B, encode_as: &str)
                where
                    B: $crate::query::QueryBuilder,
                {
                    builder.emit_key_value(encode_as, self);
                }
            }
        )*
    };
}

impl_from_query_value_for_parse!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, String
);

impl_to_query_value_for_display!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, &str, String);

#[cfg(test)]
mod tests {
    use crate::query::from_query;

    use super::{
        to_query, ConsumeStatus, FromQuery, FromQueryValue, QueryAccumulator, QueryPair,
        QueryParseError, QueryValueAccumulator, Result, ToQuery, ToQueryValue,
    };

    struct Nested {
        field_d: u32,
        field_e: Option<String>,
    }

    impl ToQuery for Nested {
        fn to_query_builder<B>(&self, builder: &mut B)
        where
            B: super::QueryBuilder,
        {
            self.field_d.to_query_builder(builder, "field_d");
            self.field_e.to_query_builder(builder, "field_e");
        }
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

        let test = Nested {
            field_d: 3,
            field_e: Some("4".to_string()),
        };
        let query = to_query(&test);
        assert_eq!(query, "field_d=3&field_e=4");
    }
}
