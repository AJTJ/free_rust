use crate::{
    apnea_sessions::dto::dive_dto::{Dive, DiveRetrievalData},
    diesel::ExpressionMethods,
    utility::errors::{BigError, DieselQuerySnafu},
};
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;
use uuid::Uuid;

pub fn get_dives(
    conn: &mut PgConnection,
    retrieval_ids: Vec<DiveRetrievalData>,
) -> Result<Option<Vec<Dive>>, BigError> {
    use crate::schema::dives::dsl::{dives, session_id, user_id};

    let mut session_ids: Vec<Uuid> = vec![];
    let mut user_ids: Vec<Uuid> = vec![];
    for variant in retrieval_ids.into_iter() {
        match variant {
            DiveRetrievalData::Session(id) => session_ids.push(id),
            DiveRetrievalData::User(id) => user_ids.push(id),
        }
    }

    dives
        .filter(session_id.eq_any(session_ids))
        .or_filter(user_id.eq_any(user_ids))
        .get_results::<Dive>(conn)
        .optional()
        .context(DieselQuerySnafu)
}

// match dive_retrieval {
//     DiveRetrievalData::Session(s) => dives.filter(session_id.eq(&s)).get_results::<Dive>(conn),
//     DiveRetrievalData::User(u) => dives.filter(user_id.eq(&u)).get_results::<Dive>(conn),
// }
