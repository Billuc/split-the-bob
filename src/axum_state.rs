use crate::db::DB;

#[derive(Clone)]
pub struct State {
    pub db: DB,
}
