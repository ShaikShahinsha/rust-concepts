

fn main(){
    let t:(i32,String,bool) = (10,"shah".to_string(),true);

    printtvalue(t);

}

fn printtvalue(t:(i32,String,bool)){
    println!("Inside print method");
    println!("{:?}",t);

    //destructing tuples values 

    let (age,is_male,cgpa ) = t; //assigns a tuple to distinct variables
   println!("Age is {} , Namae {}, is developer {}",age,is_male,cgpa);

}