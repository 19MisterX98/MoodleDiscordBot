use std::sync::{Arc, Mutex};
use std::time::Duration;

use rand::Rng;
use serenity::http::Http;
use serenity::model::id::GuildId;
use tokio::{spawn, time};

use crate::moodle_stuff::accounts::AccountList;
use crate::moodle_stuff::course_scanning::update_course;

/// scan for course updates every 5 mins
pub async fn scan_continuous(guild: GuildId, http: Arc<Http>) {
    // wait for up to 5 minutes to
    let start_time = rand::thread_rng().gen_range(0..=(5 * 60));
    time::sleep(Duration::from_secs(start_time)).await;

    let mut interval = time::interval(Duration::from_secs(5 * 60));

    let index = Arc::new(Mutex::new(0));

    loop {
        interval.tick().await;

        let http = http.clone();
        let accounts = AccountList::get_from_file(guild);
        let index = index.clone();

        spawn(async move {
            if let Some((course_name, course_id, client, channels)) =
                accounts.next_valid_course(index)
            {
                update_course(course_name, course_id, channels, &http, guild, client).await
            }
        });
    }
}
