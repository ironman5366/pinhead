use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResultList<T> {
    pub results: Vec<T>,
}
