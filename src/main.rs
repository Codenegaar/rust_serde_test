use serde::{Serialize, Deserialize};

//Data storage struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    username: String,
    email: String,
}

//Data structure for storing responses
#[derive(Serialize, Deserialize, Debug)]
struct Failure {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    users: Vec<User>,
}

fn main() {
    //Create a list of users
    let users = vec![
        User {
            id: String::from("1"),
            username: String::from("ali"),
            email: String::from("alirashidi127@gmail.com"),
        },
        User {
            id: String::from("2"),
            username: String::from("rasool"),
            email: String::from("rasool@sth.s"),
        }
    ];

    //Create a failure result
    let f = Failure { message: String::from("Operation failed successfully")};
    //And a successful result returning a user
    let s1 = UserData{
        users: vec![users.get(0).unwrap().clone()],
    };
    //And finally a successful result returning all users
    let s2 = UserData {
        users
    };

    //Serialize the results
    let ser_f = serde_json::to_string(&f).unwrap();
    let ser_s1 = serde_json::to_string(&s1).unwrap();
    let ser_s2 = serde_json::to_string(&s2).unwrap();
    println!("{}", ser_f);
    println!("{}", ser_s1);
    println!("{}", ser_s2);

    //Now deserialize them
    let des_f: Failure = serde_json::from_str(&ser_f).unwrap();
    let des_s1: UserData = serde_json::from_str(&ser_s1).unwrap();
    let des_s2: UserData = serde_json::from_str(&ser_s2).unwrap();
    println!("{:?}", des_f);
    println!("{:?}", des_s1);
    println!("{:?}", des_s2);
}