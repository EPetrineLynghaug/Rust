use chrono::NaiveDate;


pub struct Autorized;
pub struct Unautorized;

pub struct UserManager<State = Unautorized> {
    state: std::marker::PhantomData<State>,
    username: String,
    email: String,
    password: String,
    name: String,
    birthday: NaiveDate,
}

impl UserManager<Autorized> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn birthday(&self) -> NaiveDate {
        self.birthday
    }

    pub fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }

    pub fn logout(self) -> UserManager<Unautorized> {
        UserManager {
            state: std::marker::PhantomData::<Unautorized>,
            username: self.username,
            email: self.email,
            password: self.password,
            name: self.name,
            birthday: self.birthday,
        }
    }
}

impl UserManager<Unautorized> {
    pub fn login(self, email: String, password: String) -> Result<UserManager<Autorized>, String> {
        if email != self.email || password != self.password {
            return Err("Invalid email or password".to_owned());
        }

        Ok(UserManager {
            state: std::marker::PhantomData::<Autorized>,
            username: self.username,
            email: self.email,
            password: self.password,
            name: self.name,
            birthday: self.birthday,
        })
    }
}

impl UserManager {
    pub fn new(
        username: String,
        email: String,
        password: String,
        name: String,
        birthday: NaiveDate,
    ) -> UserManager<Unautorized> {
        UserManager {
            state: Default::default(),
            username,
            email,
            password,
            name,
            birthday,
        }
    }
}
