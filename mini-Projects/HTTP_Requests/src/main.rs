use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;

fn main() {
    let http_client = Client::new();
    let http_result = http_client.get("https://www.trevorsullivan.net/").send();

    if http_result.is_ok(){
        println!("Body: {:#?}", http_result.ok().unwrap().text().unwrap());
    }
    else if http_result.is_err(){
        //Handle Error
        println!("Error occurred: {:#?}", http_result.err());
    }
    
    let post_result = http_client.post("http://localhost:3000/send_data")
    .body("{\"first_name\":\"Trevor\"}").header("User-Agent", "Trevor's Rust Application").send();

    println!("{:#?}", post_result.ok().unwrap().text().unwrap());

    // Example: Redirects
    let redir_policy = Policy::limited(0);
    let http_client1 =  ClientBuilder::new().redirect(redir_policy).build().ok().unwrap();

    let http_result1 = http_client1.get("http://localhost:3000/weather").send();

    if http_result1.is_err(){
        println!("{:#?}", http_result1.err());
    }

    // println!("Weather app result : {:#?}", http_result1);

}
