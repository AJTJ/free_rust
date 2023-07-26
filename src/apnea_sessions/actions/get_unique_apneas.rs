use crate::{
    apnea_sessions::dto::unique_apnea_dto::{UniqueApnea, UniqueApneaRetrievalData},
    diesel::ExpressionMethods,
    utility::errors::{BigError, DieselQuerySnafu},
};
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;
use tracing::{event, Level};
use uuid::Uuid;

pub fn get_unique_apneas(
    conn: &mut PgConnection,
    retrieval_ids: Vec<UniqueApneaRetrievalData>,
) -> Result<Option<Vec<UniqueApnea>>, BigError> {
    event!(Level::DEBUG, "in get_unique_apneas");
    use crate::schema::unique_apneas::dsl::{session_id, unique_apneas, user_id};

    let mut session_ids: Vec<Uuid> = vec![];
    let mut user_ids: Vec<Uuid> = vec![];
    for variant in retrieval_ids.into_iter() {
        match variant {
            UniqueApneaRetrievalData::Session(id) => session_ids.push(id),
            UniqueApneaRetrievalData::User(id) => user_ids.push(id),
        }
    }

    unique_apneas
        .filter(session_id.eq_any(session_ids))
        .or_filter(user_id.eq_any(user_ids))
        .get_results::<UniqueApnea>(conn)
        .optional()
        .context(DieselQuerySnafu)
}

// match dive_retrieval {
//     DiveRetrievalData::Session(s) => unique_apneas.filter(session_id.eq(&s)).get_results::<Dive>(conn),
//     DiveRetrievalData::User(u) => unique_apneas.filter(user_id.eq(&u)).get_results::<Dive>(conn),
// }
