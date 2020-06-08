use std::process::Command;

fn main()
{
    let mut cmd = Command::new("python3.8");
    cmd.arg("hasta.py");

    match cmd.output() {
        Ok(o) => {
            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(e) => {
            println!("There was an error! {}", e);
        }
    }
}
