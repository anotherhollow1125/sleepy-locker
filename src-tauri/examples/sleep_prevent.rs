use dialoguer::Select;
use sleepy_locker::sleep_prevent::{allow_sleep, prevent_sleep};

fn main() {
    let items = ["prevent", "allow", "exit"];

    loop {
        let selection = Select::new()
            .with_prompt("Select Sleep Mode")
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                prevent_sleep();
                println!("Sleep prevention enabled");
            }
            1 => {
                allow_sleep();
                println!("Sleep prevention disabled");
            }
            _ => return,
        };
    }
}
