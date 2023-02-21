#[allow(unused)]
pub fn is_none<'a, T, U>(v: T) -> bool
where
    T: AsRef<&'a Option<U>>,
    U: 'a,
{
    v.as_ref().is_none()
}
