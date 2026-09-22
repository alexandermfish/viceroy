pub struct User{
    username: String,
    id: u64,
}

impl User {
    pub fn new(username: String, id: u64) -> User{
        return User{
            username,id
        }
    }

    pub fn id(&self) -> u64{
        return self.id;
    }
}
