pub struct Task {
    pub name: String,
    pub completed: bool,
}

impl Task {
    pub fn new(name: String) -> Task {
        Task {
            name,
            completed: false,
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = !self.completed;
    }
}