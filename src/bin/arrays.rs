fn main(){
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

    ///array with default values
    
    let ar = [-1;4];

    println!("array is {:?}",ar);
    println!("array size is :{}",ar.len());

    //iterating over arrays using looops
    for index in 0..4 {
        println!("index is: {} & value is : {}",index,ar[index]);
     }
 }

