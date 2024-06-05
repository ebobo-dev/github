use ebobo_shared::Utc;
use rocket::{futures::*, State};
use sea_orm::{prelude::*, *};

use crate::{
    entities::{matches, prelude::*, queue, fighters},
    EboboState,
};

#[options("/fight")]
pub async fn options() {}

#[post("/fight")]
pub async fn post(
    auth: crate::auth::Auth,
    ws: rocket_ws::WebSocket,
    state: &State<EboboState>,
) -> rocket_ws::Channel<'static> {
    Queue::insert(queue::ActiveModel {
        fingerprint: ActiveValue::set(auth.fingerprint.clone()),
        date: ActiveValue::set(Utc::now().naive_utc()),
    })
    .exec(state.db.as_ref())
    .await
    .unwrap();

    let db = state.db.clone();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                let queue = Queue::find()
                    .filter(queue::Column::Fingerprint.ne(auth.fingerprint.clone()))
                    .limit(1)
                    .find_also_related(fighters::Entity)
                    .all(db.as_ref())
                    .await;

                if let Ok(e) = queue {
                    if !e.is_empty() {
                        let e = e.first().unwrap().1.as_ref().unwrap();
                        let fighter = auth.fighter.unwrap();

                        let mut enemy_r = e.rank + 1;
                        let mut my_r = fighter.rank + 1;

                        let result = if e.rank == fighter.rank {
                            enemy_r += 1;
                            my_r += 1;
                            "draw".to_string()
                        } else if e.rank < fighter.rank {
                            my_r += 2;
                            "right".to_string()
                        } else {
                            enemy_r += 2;
                            "left".to_string()
                        };

                        Fighters::update(fighters::ActiveModel {
                            rank: ActiveValue::set(my_r),
                            ..Default::default()
                        })
                        .filter(fighters::Column::Fingerprint.eq(auth.fingerprint.clone()))
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        Fighters::update(fighters::ActiveModel {
                            rank: ActiveValue::set(enemy_r),
                            ..Default::default()
                        })
                        .filter(fighters::Column::Fingerprint.eq(e.fingerprint.clone()))
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        Matches::insert(matches::ActiveModel {
                            id: ActiveValue::set(Uuid::new_v4()),
                            left: ActiveValue::set(e.fingerprint.clone()),
                            right: ActiveValue::set(auth.fingerprint.clone()),
                            result: ActiveValue::set(result.clone()),
                            date: ActiveValue::set(Utc::now().naive_utc()),
                        })
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        Queue::delete_many()
                            .filter(
                                queue::Column::Fingerprint
                                    .eq(e.fingerprint.clone())
                                    .or(queue::Column::Fingerprint.eq(auth.fingerprint.clone())),
                            )
                            .exec(db.as_ref())
                            .await
                            .unwrap();

                        let _ = stream
                            .send(rocket_ws::Message::Text(result))
                            .await;
                        break;
                    }
                }
            }

            Ok(())
        })
    })
}
