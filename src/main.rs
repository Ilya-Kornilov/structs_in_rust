#[derive(Debug)]    // enables to print the entity of User struct
struct User {   
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,           // comma after each field including the last one
}                           // no sign at the end of the struct

fn main() {

    let mut user1 = User {  // if the entire struct is mut, all its fields are mut
                                  // we can't have a separate field in a struct mut  
        email: String::from("mail@gmail.com"),  
        username: String::from("User #1"),
        active: true,
        sign_in_count: 1    // comma after each field but the last one
    };                      // semicolon at the end of the struct

    let name = user1.username;
    user1.username = String::from("Changed user #1 name");

    println!("User 1 name is '{}'", name);
    println!("> and now it is '{}'", user1.username);
    println!("\temail: {}", user1.email);
    println!("\tsign-in-count: {}", user1.sign_in_count);
    println!("\tstatus: {}", user1.active);

    let user2 = build_user(
        String::from("User #2"),
        String::from("user_2@gmail.com")
    );
    
    let user3 = User {
        email: String::from("User #3"),
        username: String::from("user_3@gmail.com"),
        ..user2     // shorthand way of taking the rest fields "from" user_2
    };
    println!("{:#?}", user3);   // works only with FIRST line of code 
                                // "{:?}" prints the entity in on line
                                // "{:#?}" does it in a struct-like way

    // Example of a struct without named fields 
        //struct Color(i32, i32, i32);

}

fn build_user(email: String, username: String) -> User {
    User {
            // email: email,    
            // username: username,          
        // OR the shorthand version
        email,
        username, 
        active: true,           // goes by default
        sign_in_count: 1,       // goes by default
    }
}
