use crate::schema::team_seaking;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Insertable, Debug)]
#[diesel(table_name = team_seaking)]
pub struct TeamSeaking {
    pub id: i64,
    pub name: String,
	pub discordname: String,
    pub description: String,
}