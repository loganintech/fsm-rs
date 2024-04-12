use crate::fsm::DBObj;

pub struct DB<T: DBObj + Clone> {
    db: Option<T>,
}

impl<T: DBObj + Clone> DB<T> {
    pub fn new() -> DB<T> {
        DB {
            db: None,
        }
    }

    pub fn save(&mut self, obj: T) {
        self.db = Some(obj);
    }

    pub fn pop(&self) -> Option<T> {
        self.db.clone()
    }
}