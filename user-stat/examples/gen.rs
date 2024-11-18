use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use anyhow::Result;

use chrono::{DateTime, Utc};
use fake::{
    faker::{chrono::en::DateTimeBetween, internet::en::SafeEmail, name::zh_cn::Name},
    Dummy, Fake, Faker,
};
use nanoid::nanoid;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "gender_type", rename_all = "lowercase")]
enum Gender {
    Female,
    Male,
    Unknown,
}

#[derive(Debug, sqlx::FromRow, Dummy, PartialEq, Eq, Serialize, Deserialize)]
struct UserStats {
    #[dummy(faker = "UniqueEmail")]
    email: String,
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "RandomGender")]
    gender: Gender,
    #[dummy(faker = "DateTimeBetween(before(365*5), before(90))")]
    created_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(30), now())")]
    last_visited_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    last_watched_at: DateTime<Utc>,
    #[dummy(faker = "IntList(50, 0, 10000)")]
    recent_watched: Vec<i32>,
    #[dummy(faker = "IntList(50, 10000, 20000)")]
    viewed_but_not_started: Vec<i32>,
    #[dummy(faker = "IntList(50, 20000, 30000)")]
    started_but_not_finished: Vec<i32>,
    #[dummy(faker = "IntList(50, 30000, 40000)")]
    finished: Vec<i32>,
    #[dummy(faker = "DateTimeBetween(before(45), now())")]
    last_email_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(15), now())")]
    last_push_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    last_sms_notification: DateTime<Utc>,
}

impl Hash for UserStats {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    for i in 1..=500 {
        let users: HashSet<UserStats> = (0..10000).map(|_| Faker.fake::<UserStats>()).collect();
        let pool =
            PgPool::connect("postgres://postgres:mysecretpassword@localhost:5432/stats").await?;

        let start = Instant::now();
        bulk_insert(users, &pool).await.unwrap();
        println!("Batch {} inserted in {:?}", i, start.elapsed());
    }

    Ok(())
}

async fn bulk_insert(users: HashSet<UserStats>, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;
    for user in users {
        sqlx::query(
            r#"
            INSERT INTO user_stats
            (email, name, gender, created_at, last_visited_at, last_watched_at, recent_watched, viewed_but_not_started, started_but_not_finished, finished, last_email_notification, last_push_notification, last_sms_notification)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#)
            .bind(user.email)
            .bind(user.name)
            .bind(user.gender)
            .bind(user.created_at)
            .bind(user.last_visited_at)
            .bind(user.last_watched_at)
            .bind(user.recent_watched)
            .bind(user.viewed_but_not_started)
            .bind(user.started_but_not_finished)
            .bind(user.finished)
            .bind(user.last_email_notification)
            .bind(user.last_push_notification)
            .bind(user.last_sms_notification)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;

    Ok(())
}

fn before(days: u64) -> DateTime<Utc> {
    Utc::now() - chrono::Duration::days(days as i64)
}

fn now() -> DateTime<Utc> {
    Utc::now()
}

struct UniqueEmail;
pub const SAFE: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UniqueEmail, rng: &mut R) -> String {
        let email: String = SafeEmail().fake_with_rng(rng);
        let id = nanoid!(8, &SAFE);
        let at = email.find('@').unwrap();
        format!("{}.{}{}", &email[..at], id, &email[at..])
    }
}

struct RandomGender;
impl Dummy<RandomGender> for Gender {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &RandomGender, rng: &mut R) -> Gender {
        [Gender::Female, Gender::Male, Gender::Unknown]
            .choose(rng)
            .unwrap()
            .to_owned()
    }
}

struct IntList(i32, i32, i32);

impl Dummy<IntList> for Vec<i32> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &IntList, rng: &mut R) -> Vec<i32> {
        let (len, min, max) = (config.0, config.1, config.2);
        (0..len).map(|_| rng.gen_range(min..max)).collect()
    }
}
