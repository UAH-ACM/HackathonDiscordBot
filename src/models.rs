use crate::schema::team_seaking;
use crate::schema::teams;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Insertable, Eq, PartialEq, Debug)]
#[diesel(table_name = team_seaking)]
pub struct TeamSeaking {
    pub id: i64,
    pub name: String,
	pub discordname: String,
    pub description: String,
}

#[derive(Identifiable, Queryable, Insertable, Eq, PartialEq, Debug)]
#[diesel(table_name = teams)]
pub struct Teams {
    pub id: i64,
	pub name: String,
	pub description: String,
}