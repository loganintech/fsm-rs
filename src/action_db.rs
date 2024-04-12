use crate::fsm;
use crate::fsm::DBObj;

pub struct DB<T: DBObj> {
    db: Vec<T>,
}

impl<T: DBObj> DB<T> {
    pub fn new() -> DB<T> {
        DB {
            db: Vec::new(),
        }
    }

    pub fn save(&mut self, obj: T) {
        self.db.push(obj);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.db.pop()
    }
}