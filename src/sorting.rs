use rand::Rng;

pub mod divide_et_impera;

pub fn randomize_array(arr : &mut [i32]){
    let len = arr.len();
    for x in 0..len{
        arr[x] = rand::thread_rng().gen_range(0..100);
    }
}

pub fn randomize_vec(vector: &mut Vec<i32>){
    let len = vector.len();
    for x in 0..len{
        vector[x] = rand::thread_rng().gen_range(0..100);
    }
}

// Generic type in function + traits
pub fn check_array_if_sorted<T: std::cmp::PartialOrd>(arr: &mut [T]) -> bool {
    for i in 0..(arr.len() - 1) {
        if arr[i] > arr[i + 1] {return false;}
    }
    true
}

pub fn check_vec_if_sorted<T: std::cmp::PartialOrd>(vector: &mut Vec<T>) -> bool {
    for i in 0..(vector.len() - 1) {
        if vector[i] > vector[i + 1] {return false;}
    }
    true
}

/*
fn randomize_array_user(arr : &mut [User]){
    let len = arr.len();
    for x in 0..len{
        arr[x].id = rand::thread_rng().gen_range(0..100000);
        let ban = rand::thread_rng().gen_range(0..100);
        if ban > 50 {arr[x].banned = true;}
        else {arr[x].banned = false;}
        let user_nick_len = rand::thread_rng().gen_range(3..12);
        let mut name = String::new();
        for i in 0..user_nick_len {
            let byte = rand::thread_rng().gen_range(65..90) as u8;
            name.push(byte as char);
        }
    }
}
*/

pub fn margarynowy_sort(arr :&mut [i32]){
    let len = arr.len();

    for i in 0..len{
        for j in 1..len - i{
            if arr[j-1] > arr[j]{
                arr.swap(j-1, j);
            }
        }
    }
}

pub fn margarynowy_vec_sort(vector: &mut Vec<i32>){
    let len = vector.len();

    for i in 0..len{
        for j in 1..len - i{
            if vector[j-1] > vector[j]{
                vector.swap(j-1, j);
            }
        }
    }
}
