pub mod sorting;
use sorting::divide_et_impera;

#[derive(Debug)]
struct User { // normal struct
    _id : u64,
    name : String,
    banned : bool
}

impl User {
    fn gets_fucking_banned(&mut self) {
        self.banned = true;
    }

    fn user_create(_id: u64, name: String, ban: bool) -> User{
        User {
            _id, // Line id: id can be skipped thanks to field init shorthand
            name,
            banned: ban
        }
    }
}

struct _MySanity(/* FUCKING NOTHING */); 

enum WeBallin {
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
    let mut vec: Vec<i32> = Vec::new();
    vec.resize(100, 0);

    println!("Margaryna sort\n");
    sorting::randomize_vec(&mut vec);
    sorting::margarynowy_vec_sort(&mut vec);
    for i in &vec
    {
        print!("{}, ", i);
    }
    println!("\nQuicksort\n");
    sorting::randomize_vec(&mut vec);
    divide_et_impera::vec_quicksort(&mut vec, 0, 99);
    for i in &vec
    {
        print!("{}, ", i);
    }

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
    let ball = WeBallin::Ballin(String::from("WEED"));
    let baller = WeBallin::NoBallin;

    WeBallin::read_ballin(&ball);
    WeBallin::read_ballin(&baller);

    let ball = WeBallin::Ballin(String::from("Romper 12%"));
    let mut mess = String::new();
    if let WeBallin::Ballin(text) = ball {mess = text;}
    println!("{mess}");

}


