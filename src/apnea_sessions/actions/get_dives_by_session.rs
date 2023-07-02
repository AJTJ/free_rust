use crate::{
    apnea_sessions::dto::dive_dto::{Dive, DiveFilter},
    diesel::ExpressionMethods,
    utility::gql::query_dto::QueryParams,
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dives_by_session(
    conn: &mut PgConnection,
    input_session_id: Uuid,
    _dive_query_input: Option<DiveFilter>,
    query_params: Option<QueryParams>,
) -> diesel::QueryResult<Vec<Dive>> {
    use crate::schema::dives::dsl::{dives, session_id};

    dives
        .filter(session_id.eq(&input_session_id))
        .get_results::<Dive>(conn)
}
