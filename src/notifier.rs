use lettre::{Message, SmtpTransport, Transport}; // For constructing and sending emails using SMTP.

/**
 * Sends an email notification.
 *
 * # Arguments
 * * `to` - The recipient's email address.
 * * `subject` - The subject of the email.
 * * `body` - The body content of the email.
 *
 * # Panics
 * This function will panic if:
 * - The email address format is invalid.
 * - The SMTP server relay is not properly configured or fails to send the email.
 *
 * # Example
 * ```
 * send_email("user@example.com", "Test Subject", "Test Body");
 * ```
 */
pub fn send_email(to: &str, subject: &str, body: &str) {
    // Build the email message with the specified parameters.
    let email = Message::builder()
        .from("Server Monitor <monitor@example.com>".parse().unwrap()) // Sender's email address.
        .to(to.parse().unwrap()) // Recipient's email address.
        .subject(subject) // Email subject line.
        .body(body.to_string()) // Email body content.
        .unwrap();

    // Configure the SMTP transport with the relay server.
    let mailer = SmtpTransport::relay("smtp.example.com") // Specify the SMTP relay server.
        .unwrap()
        .build();

    // Send the email via the SMTP server.
    mailer.send(&email).unwrap();
}
