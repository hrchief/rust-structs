struct User {
    email: String,
    name: String,
    active: bool,
    sign_in_count: i32,
}

fn create_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 0,
    }
}

#[derive(Debug)] // Adding Useful Functionality with Derived Traits
struct Rect {
    width: u32,
    height: u32,
}
// Method
impl Rect {
    fn rect_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let sample_user = create_user(String::from("Anonymous"), String::from("Anonymous"));
    println!(
        "email: {}\nname:{}\nactive:{}\nsignincount:{}\n",
        sample_user.email, sample_user.name, sample_user.active, sample_user.sign_in_count
    );
    let bomji_user = User {
        email: String::from("bomji10123@mail.ru"),
        name: String::from("Bomji"),
        ..sample_user
    };
    println!(
        "email: {}\nname:{}\nactive:{}\nsignincount:{}\n",
        bomji_user.email, bomji_user.name, bomji_user.active, bomji_user.sign_in_count
    );
    // calculate rectangle area
    let rect = Rect {
        width: 30,
        height: 30,
    };
    println!("area of rectangle is {}", rect.rect_area()); // Method
                                                           // Adding Useful Functionality with Derived Traits
    let rectangle = Rect {
        width: 10,
        height: 10,
    };
    println!("rect: {:?}", rectangle);
}
