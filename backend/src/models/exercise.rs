use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Exercise {
    pub name: String,
    pub sets: i32,
    pub reps: i32,
    pub tag: String,
}

impl Exercise {
    pub fn new(name: String, sets: i32, reps: i32, tag: String) -> Exercise {
        Exercise {
            name,
            sets,
            reps,
            tag,
        }
    }
}
