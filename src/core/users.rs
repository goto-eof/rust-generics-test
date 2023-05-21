pub trait User {
    fn new(username: &'static str) -> Self;
    fn do_operation(&self) -> ();
    fn get_username(&self) -> String;
    fn to_string(&self) -> () {
        println!("{}", Self::get_username(&self));
    }
}

pub struct Customer {
    username: &'static str,
}

pub struct Administrator {
    username: &'static str,
}

impl User for Customer {
    fn new(username: &'static str) -> Self {
        Self { username: username }
    }

    fn do_operation(&self) -> () {
        println!("buying");
    }

    fn get_username(&self) -> String {
        self.username.to_owned()
    }
}

impl User for Administrator {
    fn new(username: &'static str) -> Self {
        Self { username }
    }
    fn do_operation(&self) -> () {
        println!("managing");
    }
    fn get_username(&self) -> String {
        self.username.to_owned()
    }
}
pub fn do_job<T: User>(user: T) -> () {
    println!("{:?}", user.get_username());
    user.do_operation();
}
