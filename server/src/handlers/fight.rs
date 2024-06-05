use ebobo_shared::Utc;
use rocket::{futures::*, State};
use sea_orm::{prelude::*, *};

use crate::{
    entities::{matches, prelude::*, queue, users},
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
        id: ActiveValue::set(Uuid::new_v4()),
        fighter: ActiveValue::set(auth.fingerprint.clone()),
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
                    .filter(queue::Column::Fighter.ne(auth.fingerprint.clone()))
                    .limit(1)
                    .all(db.as_ref())
                    .await;

                if let Ok(e) = queue {
                    if !e.is_empty() {
                        let e = e
                            .first()
                            .unwrap()
                            .find_related(users::Entity)
                            .one(db.as_ref())
                            .await
                            .unwrap()
                            .unwrap();

                        let fighter = auth.fighter.unwrap();

                        // let result = if e.rank == fighter.rank {
                        //     "draw".to_string()
                        // } else if e.rank < fighters.rank {
                        //     "right".to_string()
                        // } else {
                        //     "left".to_string()
                        // };

                        Matches::insert(matches::ActiveModel {
                            id: ActiveValue::set(Uuid::new_v4()),
                            left: ActiveValue::set(e.fighter.clone()),
                            right: ActiveValue::set(auth.fingerprint.clone()),
                            result: ActiveValue::set("".to_string()),
                            date: ActiveValue::set(Utc::now().naive_utc()),
                        })
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        let _ = stream
                            .send(rocket_ws::Message::Text(e.id.to_string()))
                            .await;
                        break;
                    }
                }
            }

            Ok(())
        })
    })
}
