#[macro_export]
macro_rules! get_data {
    ($x:ident, $t:ty) => {
        {
            let data = $x.data.lock();
            match data.get::<$t>() {
                Some(db) => db.clone(),
                None => bail!("Could not get {}", stringify!($t)),
            }
        }
    }
}
