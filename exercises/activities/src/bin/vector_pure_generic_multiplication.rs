fn main(){

}

fn generic_mul<T: std::ops::Mul<Output = T> +  std::ops::Add<Output = T>>(one: Vec<T>,two: Vec<T>,three: Vec<T>,four: Vec<T>) -> Vec<T>{
    let vec5 = Vec::new();
    let vec6 = Vec::new();
    let vec7 = Vec::new();


    for i in 0..one.len() {
        vec5.push(one[i] * two[i])
    }
    for i in 0..three.len() {
        vec6.push(three[i] * four[i])
    }

    for i in 0..vec5.len(){
        vec7.push(vec5[i] + vec6[i])
    }

    return vec7;

}