struct AuthenticationInfo {
    username: String,
    password: String
}

impl AuthenticationInfo {
    fn get_basic_user_auth(&self) -> String {
        format!("Authorization: {}:{}", self.username, self.password)
    }
}

fn authenticate(auth_info: &AuthenticationInfo) -> () {
    //println!("{}", auth_info.get_basic_user_auth());
    ()
}

fn main() {
    let curr_user = AuthenticationInfo {
        username: String::from("gfssp"),
        password: String::from("1234pwd"),
    };

    println!("{}", curr_user.get_basic_user_auth());

    authenticate(&curr_user);
}
