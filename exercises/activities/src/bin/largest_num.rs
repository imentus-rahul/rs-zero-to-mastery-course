fn main(){
    let some_vec = vec![1,5,3,7,2,9,8,0,6];
    let mut max_num = some_vec[0];
    for n in some_vec{
        if max_num < n{
            max_num = n;
        } 
    }

    println!("Max number is: {:?}", max_num)
}