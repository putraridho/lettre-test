use dotenv::dotenv;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let username = dotenv::var("USERNAME").unwrap();
    let password = dotenv::var("PASSWORD").unwrap();

    println!("--> Creating Email");
    let email = Message::builder()
        .from(
            "Muhammad Ridho Putra <muhammadridhoputra@gmail.com>"
                .parse()
                .unwrap(),
        )
        .to("Bobo Marde <bobomarde@yahoo.com>".parse().unwrap())
        .subject("Hello World!")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Let's get rusty"))
        .unwrap();

    println!("--> Defining Credentials");
    let creds = Credentials::new(username, password);

    println!("--> Setting up Mailer");
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

    println!("--> Sending Email");
    match mailer.send(email).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
