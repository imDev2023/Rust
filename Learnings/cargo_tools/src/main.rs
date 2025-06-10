use uuid::Uuid;

fn main()
{
    println!("UUID");
    print!("generated UUID is: {}",uuid::Uuid::new_v4().to_string());

}
