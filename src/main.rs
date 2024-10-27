use axum::response::IntoResponse;
use std::io::BufWriter;
use std::str::from_utf8;
use std::{fs, io};

use axum::body::Body;
use axum::extract::multipart::Field;
use axum::extract::Multipart;
use axum::http::response::Parts;
use axum::http::Response;
use axum::routing::{get, post};
use axum::Router;

mod c24;
mod ynab;

const CSV_EXPORT: &str = "./transactions.csv";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/convert", post(convert_to_ynab));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn convert_to_ynab(mut multipart: Multipart) -> impl IntoResponse {
    let mut buf = None;
    while let Ok(Some(mut field)) = multipart.next_field().await {
        if field.name().unwrap_or("") != "c24_export" {
            continue;
        }

        let payload = field.bytes().await.unwrap().to_vec();
        let csv = from_utf8(&payload).unwrap();

        let transactions = c24::parse_csv(csv.as_bytes()).map(|t| ynab::Record {
            date: t.booking_date.format("%Y-%m-%d").to_string(),
            payee: t.recipient,
            memo: t.reason_for_payment,
            amount: t.amount.to_string(),
        });

        buf = Some(ynab::write_csv(transactions).unwrap())
    }
}
