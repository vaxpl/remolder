use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiMessage<D, E, M> {
    pub data: Option<D>,
    pub errors: Option<E>,
    pub meta: Option<M>,
    pub status: Option<String>,
}

impl<D, E, M> ApiMessage<D, E, M> {
    pub fn failure(errors: Option<E>) -> Self {
        Self {
            data: None,
            errors,
            meta: None,
            status: Some("failure".to_string()),
        }
    }

    pub fn success(data: Option<D>, meta: Option<M>) -> Self {
        Self {
            data,
            errors: None,
            meta,
            status: Some("sucess".to_string()),
        }
    }
}
