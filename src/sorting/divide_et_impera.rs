pub fn quicksort(arr : &mut [i32], l:i32, r:i32){
    if l >= r { return; }
    let mut min_index = l - 1;
    let mut max_index = r + 1;
    let p = arr[(l + r) as usize / 2]; // pivot
    loop
    {
        min_index += 1;
        while arr[min_index as usize] < p
        {
            min_index += 1;
        }
        max_index -= 1;
        while arr[max_index as usize] > p
        {
            max_index -= 1;
        }
        if min_index <= max_index
        {
            arr.swap(min_index as usize, max_index as usize);
        }
        else
        {
            break;
        }
    }
    if min_index < r
    {
        quicksort(arr, min_index, r);
    }
    if max_index > l
    {
        quicksort(arr, l, max_index);
    }
}

pub fn vec_quicksort(vector: &mut Vec<i32>, l:i32, r:i32){
    if l >= r { return; }
    let mut min_index = l - 1;
    let mut max_index = r + 1;
    let p = vector[(l + r) as usize / 2]; // pivot
    loop
    {
        min_index += 1;
        while vector[min_index as usize] < p
        {
            min_index += 1;
        }
        max_index -= 1;
        while vector[max_index as usize] > p
        {
            max_index -= 1;
        }
        if min_index <= max_index
        {
            vector.swap(min_index as usize, max_index as usize);
        }
        else
        {
            break;
        }
    }
    if min_index < r
    {
        quicksort(vector, min_index, r);
    }
    if max_index > l
    {
        quicksort(vector, l, max_index);
    }
}