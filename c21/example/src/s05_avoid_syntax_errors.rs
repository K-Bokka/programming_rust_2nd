macro_rules! complain {
    ($msg:expr) => {
        println!("Complaint filed: {}", $msg);
    };
    (user : $userid:tt , $msg:expr) => {
        println!("Complaint from user {}: {}", $msg, $userid);
    };
}

pub fn run() {
    println!("21.5 Avoid Syntax Errors");

    complain!(user: "jimb", "thi AI lab's chatbots keep picking on me");
}
