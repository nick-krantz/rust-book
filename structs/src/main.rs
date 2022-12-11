fn main() {
    let mut user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
      email: String::from("anotherone@example.com");,
      ..user1, // user1 username now has moved to `user2`, so user1 is no longer valid
      // if only active & sign_in_count were `copied` user1 could still be valid
    }
  }

// implicitly return a struct
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username, // shorthand syntax
    active: true,
    sign_in_count: 1
  }
}

struct User {
  active: bool;
  username: String;
  email: String;
  sign_in_count: u64;
}
