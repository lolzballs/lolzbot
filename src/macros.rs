#[macro_export]
macro_rules! get_data {
    ($x:ident, $t:ty) => {{
        let mut data = $x.data.write();
        match data.get_mut::<$t>() {
            Some(db) => db.clone(),
            None => bail!("Could not get {}", stringify!($t)),
        }
    }};
}
