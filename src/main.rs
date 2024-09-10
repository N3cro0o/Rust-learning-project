pub mod sorting;
#[cfg(test)]
pub mod testing;

use sorting::divide_et_impera;
use std::collections::HashMap;
use std::fmt::Display;

pub trait ActiveUser {
    fn add_message(&self, mesg: &String) -> String {
        let fin_mesg = format!("Unknown pleb said: {}", mesg);
        fin_mesg
    }

    fn show_id(&self) -> u64;
}

pub enum OpType {
    Admin(u8),
    Moderator(u32),
    BetterModerator(u16),
    Overseer(u16)
}

#[derive(Debug)]
pub struct User { // normal struct
    id : u64,
    name : String,
    banned : bool
}

pub struct Admin {
    user_metadata: User,
    op_type: OpType
}

impl User {
    fn gets_fucking_banned(&mut self) {
        self.banned = true;
    }

    fn user_create(id: u64, name: String, ban: bool) -> User{
        if id == 0 {panic!("IDs can't be 0")};
        User {
            id, // Line id: id can be skipped thanks to field init shorthand
            name,
            banned: ban
        }
    }

    fn name_check<'a>(user1: &'a User, user2: &'a User) -> &'a User {
        if user1.banned { return user2 }
        else if user2.banned { return user1 }

        if user1.id > user2.id { user1 }
        else { user2 }
    }
}

impl Admin {
    fn admin_create(id: u64, name: String, ban: bool, op_type: OpType) -> Admin {
        let user_metadata = User {
            id,
            name,
            banned: ban
        };
        Admin {
            user_metadata,
            op_type
        }
    }
}

impl ActiveUser for User {
    fn show_id(&self) -> u64 {
        self.id
    }
}

impl ActiveUser for Admin {
    fn show_id(&self) -> u64 {
        self.user_metadata.id
    }

    fn add_message(&self, mesg: &String) -> String {
        let title = match self.op_type {
            OpType::Admin(num) => format!("Aristocrat no {}", num),
            OpType::BetterModerator(num) => format!("Duke no {}", num),
            OpType::Moderator(num) => format!("Worker no {}", num),
            OpType::Overseer(num) => format!("Patricius no {}", num),
        };
        let fin_mesg = format!("{} {} said: {}", title, self.user_metadata.name, mesg);
        fin_mesg
    }
}

pub struct _MySanity(/* FUCKING NOTHING */); 

pub enum WeBallin {
    NoBallin,
    Ballin(String)
}

impl WeBallin {
    fn read_ballin(data : &WeBallin){
        match data
        {
            WeBallin::Ballin(t) => println!("{t}"),
            WeBallin::NoBallin => println!("No ballin!"),
        }
    }
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut vec: Vec<i32> = Vec::new();
    vec.resize(100, 0);

    println!("Margaryna sort\n");
    sorting::randomize_vec(&mut vec);
    sorting::margarynowy_vec_sort(&mut vec);
    println!("Sorted state -> {}", sorting::check_vec_if_sorted(&mut vec));
    println!("\nQuicksort\n");
    sorting::randomize_vec(&mut vec);
    divide_et_impera::vec_quicksort(&mut vec, 0, 99);
    println!("Sorted state -> {}", sorting::check_vec_if_sorted(&mut vec));

    // Slices - also works for arrays and ect
    let strng = "String slice!".to_string();
    println!("{strng}");
    let slice_message = &strng[7..];
    println!("{}", slice_message);

    // Structs
    let mut user_a = User::user_create(1, String::from("Debil"), false);
    let user_b = User{
        name: String::from("Papaj"),
        ..user_a
    };
    println!("\n\nUser1 name: {}, user2 name: {}", user_a.name, user_b.name);
    user_a.gets_fucking_banned();
    println!("User1 {user_a:?}"); // {} - display or normal; {:?} - debug
    dbg!(&user_a);

    // Enums
    let ball = WeBallin::Ballin(String::from("ZIOŁO"));
    let baller = WeBallin::NoBallin;

    WeBallin::read_ballin(&ball);
    WeBallin::read_ballin(&baller);

    let ball = WeBallin::Ballin(String::from("Romper 12%"));
    let mut mess = String::new();
    if let WeBallin::Ballin(text) = ball {mess = text;}
    println!("{mess}");

    // Hashmap or AKA Dictionary
    let mut balling_reasons: HashMap<String, isize> = HashMap::new();
    balling_reasons.insert("Zioło".to_string(), 255);
    balling_reasons.insert(String::from("Romper 12%"), 420);
    let ziolo_score = match balling_reasons.get("Zioło") {
        Some(val) => val,
        None => {panic!("KURWA CHAT ZOSTAL ZLEAKOWANY")}
    };
    println!("Ziolo score: {ziolo_score}");
    println!("{}", balling_reasons.entry(String::from("Zioło")).or_default());

    // Traits
    let ad = Admin::admin_create(0, "Krul".to_string(), false, OpType::Admin(0));
    println!("{}", ad.add_message(&String::from("Krul ma zawsze racje")));
    println!("{}", user_a.add_message(&String::from("Pierdolnee cie zaraz, oscypku")));
    println!("{}", show_id(&ad));

    // Lifetimes
    let life_test1 = "Sranie";
    {
        let life_test2 = "Banie";
        println!("{} w {}", life_test1, life_test2);
    }

    let longest_string = longest_with_an_announcement("Gowno", "Ziolo ", 124);
    println!("{}", longest_string);
}

pub fn show_id(user: &impl ActiveUser) -> u64{
    user.show_id()
}

// Generic types, traits and lifetimes summary in a nutshell
pub fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
