use crate::{
    apnea_sessions::dto::dive_dto::{Dive, DiveFilter, DiveRetrievalData},
    diesel::ExpressionMethods,
    utility::gql::query_dto::QueryParams,
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn get_dives(
    conn: &mut PgConnection,
    dive_retrieval: DiveRetrievalData,
    dive_query_input: Option<DiveFilter>,
    db_query_ob: Option<QueryParams>,
) -> diesel::QueryResult<Vec<Dive>> {
    use crate::schema::dives::dsl::{dives, session_id, user_id};

    match dive_retrieval {
        DiveRetrievalData::Session(s) => dives.filter(session_id.eq(&s)).get_results::<Dive>(conn),
        DiveRetrievalData::User(u) => dives.filter(user_id.eq(&u)).get_results::<Dive>(conn),
    }
}
