struct User {
    username: String,
    email: String,
}

impl User {
    pub fn exists(&self, target_email: &str, users: &mut Vec<String>) -> Result<bool, String> {
        for email in users.iter() {
            if *email == target_email.to_string() {
                return Ok(true);
            }
        }
        return Err(String::from("User not found"))
    }
    
    pub fn login(&self, password: &str) -> Result<bool, String>{
        Ok(true)
    }
}

fn main() {
    let mut users: Vec<String> = vec![String::from("guilherme@mail.com")];

    let me:User = User {
        username: String::from("Guilherme Sampaio"),
        email: String::from("guilherme@mail.com")
    };

    match me.exists(&me.email, &mut users) {
        Ok(value) => {
            if value {
                println!("User found in vector!");
            } else {
                println!("User not found in vector!");
            }
        }
        Err(err) => println!("User not found in vector: {}", err),
    }
}
