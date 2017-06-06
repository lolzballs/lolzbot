#[macro_export]
macro_rules! get_database {
    ($x:ident) => {
        {
            let data = match $x.data.lock() {
                Ok(data) => data,
                Err(_) => bail!(::ErrorKind::MutexPosioned),
            };
            match data.get::<::DbPool>() {
                Some(db) => db.clone(),
                None => bail!(::ErrorKind::NoDatabase),
            }
        }
    }
}
