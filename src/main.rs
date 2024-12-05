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



/*



use lambda_http::{run, http::{StatusCode, Response}, service_fn, Error, IntoResponse, Request, RequestPayloadExt, LambdaEvent};
use lettre::message::{Message, MultiPart};
use rusoto_ses::{RawMessage, SendRawEmailRequest, Ses, SesClient};
use serde::{Deserialize};
use serde_json::json;
use base64::{encode};

#[derive(Deserialize)]
struct EmailRequest {
from_address: String,
from_name: String,
subject: String,
message: String,
}

/// This is the main body for the function.
async fn function_handler(event: LambdaEvent<EmailRequest>) -> Result<String, Error> {
    let email_request = event.payload;
    let ses_client = SesClient::new(rusoto_core::Region::UsWest2);

    let from_address = email_request.from_address;
    let to = "mac@macpatterson.com";
    let subject = email_request.subject.to_string();
    let message = email_request.message;
    let from_name = email_request.from_name;

    send_email_ses(&ses_client, from_address, from_name, to, subject, message.to_string()).await;

    let resp = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(json!({
            "message": format!("Hello, message sent!"),
          }).to_string())
        .map_err(Box::new)?;
    Ok(resp)    
}

pub async fn send_email_ses(
    ses_client: &SesClient,
    from_address: &str,
    from_name: &str,
    to: &str,
    subject: &str,
    message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let text_body: String = format!("The following was submitted from macpatterson.com.\r\n
                    Name: {} \r\n
                    Email: {} \r\n
                    Message: {} \r\n\r\n
                    Reply to address set to senders email address.",from_name,from_address,message.to_string());
    let html_body: String = format!("<html>
                        <head></head>
                            <body>
                                <h1>The follwoing was submitted from macpatterson.com.</h1>
                                    <p>Name: {}</p>
                                    <p>Email: {}</p>
                                    <p>Message: {}</p>
                                    <p>Reply to address set to senders email address.</p>
                                </body>
                        </html>",from_name,from_address,message.to_string());   

    let m = Message::builder()
        .from("No Reply <nnoreply@macpatterson.com>".parse()?)
        .reply_to(format!("{} <{}>",from_name,from_address).parse()?)
        .to(to.parse()?)
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(
            String::from(text_body),
            String::from(html_body),
        ))?;
    
    let raw_email = m.formatted();

    /*let email = MessageBuilder::new()
        .from((from_name.parse()?,from_address.parse()?))
        .to(to.parse()?)
        .subject(subject)
        .text_body(text_body)
        .html_body(html_body)?;

    let raw_email = email.formatted();
    */
    let ses_request = SendRawEmailRequest {
        raw_message: RawMessage {
            data: base64::encode(raw_email).into(),
        },
        ..Default::default()
    };

    ses_client.send_raw_email(ses_request).await?;

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();
    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    ///run(service_fn(function_handler)).await
}

*/