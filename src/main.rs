extern crate reqwest;

fn main()
{
    let response_text = reqwest::get("http://birdboard.test/")
        .expect("Couldn't make request")
        .text().expect("Could not read response text");

    println!("Response Text: {}", response_text);

    {
        match reqwest::get("http://birdboard.test/") {
            Ok(mut response) => {
                if response.status() == reqwest::StatusCode::Ok {
                    match response.text() {
                        Ok(text) => println!("Response Text: {}", text),
                        Err(_) => println!("Could not read response text")
                    }
                } else {
                    println!("Response was not ok.");
                }
            }
            Err(_) => println!("Could not make the request!"),
        }
    }
}
