use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::convert::TryInto;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

impl TryInto<NewSubscriber> for FormData {
    type Error = String;

    fn try_into(self) -> Result<NewSubscriber, Self::Error> {
        let name = SubscriberName::parse(self.name)?;
        let email = SubscriberEmail::parse(self.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Add a new subscriber",
    skip(form, pool),
    fields(
        user_email = %form.email,
        user_name = %form.name
    )
)]
pub async fn subscribe(
    pool: web::Data<PgPool>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, HttpResponse> {
    let new_subscriber: NewSubscriber = form
        .0
        .try_into()
        .map_err(|_| HttpResponse::BadRequest().finish())?;
    insert_subscriber(&pool, &new_subscriber)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Save details in database", skip(pool, new_subscriber))]
async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, name, email, subs_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        new_subscriber.name.as_ref(),
        new_subscriber.email.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("{}", e);
        e
    })?;
    Ok(())
}
