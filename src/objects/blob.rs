use crate::ApplicationError;

#[derive(Debug)]
pub struct Blob {
    // Maybe keep a slice instead of a string?
    data: String,
}

impl Blob {
    pub fn from_raw(raw: &str) -> Result<Blob, ApplicationError> {
        Ok(Blob {
            data: raw.to_string(),
        })
    }

    pub fn serialize(&self) -> Result<String, ApplicationError> {
        Ok(self.data.clone()) // FIXME: remove the clone and return a reference maybe?
    }
}
