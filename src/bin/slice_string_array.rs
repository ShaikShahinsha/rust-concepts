fn main(){

    let arrayval:[i32;6] = [10,20,40,23,23,11];

    let  name = "Shahenshah".to_owned();

    slice_array(&arrayval);
    slice_string(&name);
}

fn slice_array(ar:&[i32]){
    println!("the passed array is: {:?}",ar);
    println!("the requested array is: {:?}",&ar[0..5]);
}

fn slice_string(s:&String){

    println!("the passed String is: {:?}",s);
    println!("the reqeuested String is: {:?}",&s[0..4]); 
}

