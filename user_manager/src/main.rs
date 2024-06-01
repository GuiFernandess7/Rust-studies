struct User {
    name: String,
    age: i32,
    title: Profession,
}

enum Profession {
    Manager,
    Developer,
    Director
}

fn main() {
    let name = String::from("Jorge");
    let age: i32 = 15;

    println!("Adding user {}...", &name);
    match set_new_user(name, age, Profession::Developer){
        Ok(_) => println!("User added sucessfully!"),
        Err(err) => eprintln!("That was an error creating an user {}", err)
    };
}

fn set_new_user(name: String, age: i32, profession: Profession) -> Result<(), String> {
    let new_user: User = User{
        name: name,
        age: age,
        title: profession
    };
    Ok(())
}
