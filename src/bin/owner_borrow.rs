fn main(){

    let v = vec![10,23];

    printvec(&v);

    println!("{:?}",v);

    let tupval:(i32,String) = (10,"Shah".to_owned());

    println!("{:?}",tupval)
}

fn printvec(v:&Vec<i32>){
    println!("inseide the pritvecfunc() {:?}",v);
}