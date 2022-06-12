fn check(org_arr:&[i32], sub_arr:&[i32]) ->bool {
    if sub_arr.len() == 0 {
        return true;
    }
   for window in org_arr.windows(sub_arr.len()) {
             if window == sub_arr {
                return true;
             }
   } 
   return false;
}

fn main() {
    let first_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let second_arr = [6, 8, 10];
    if check(&first_arr, &second_arr) {
        println!("co mang con");}
    else { 
        println!("khong co mang con");}
}