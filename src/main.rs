use aws_sdk_sesv2::types::{Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;

#[derive(Deserialize)]
struct EmailRequest {
    from_email: String,
    from_name: String,
    subject: String,
    sentmessage: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Parse JSON payload
    let body = event.body();
    let email_request: EmailRequest = match serde_json::from_slice(body) {
        Ok(req) => req,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .body("Invalid JSON payload".into())
                .unwrap());
        }
    };

    // Initialize SES client
    let config = aws_config::load_from_env().await;
    let ses_client = Client::new(&config);

    // Prepare the email content
    let destination = Destination::builder()
        .to_addresses("mac@macpatterson.com")
        .build();

    let html_body: String = format!("<html>
        <head></head>
            <body>
                <h1>The follwoing was submitted from macpatterson.com.</h1>
                    <p>Name: {}</p>
                    <p>Email: {}</p>
                    <p>Message: {}</p>
                    <p>Reply to address set to senders email address.</p>
                </body>
        </html>",email_request.from_name,email_request.from_email,email_request.sentmessage.to_string());   

    let message = Message::builder()
        .subject(Content::builder().data(email_request.subject.clone()).build()?)
        .body(
            aws_sdk_sesv2::types::Body::builder()
                .text(Content::builder().data(html_body.clone()).build()?)
                .build(),
        )
        .build();

    let email_content = EmailContent::builder().simple(message).build();

    // Send the email using SES
    match ses_client
        .send_email()
        .from_email_address("no_reply@macpatterson.com")
        .reply_to_addresses(email_request.from_email.clone())
        .destination(destination)
        .content(email_content)
        .send()
        .await
    {
        Ok(_) => Ok(Response::builder()
            .status(200)
            .body("Email sent successfully".into())
            .unwrap()),
        Err(err) => {
            eprintln!("Error sending email: {:?}", err);
            Ok(Response::builder()
                .status(500)
                .body("Failed to send email".into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}