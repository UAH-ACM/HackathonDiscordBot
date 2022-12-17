use super::super::super::schema::{team_seaking::dsl::*, teams::dsl::*};
use diesel::prelude::*;

use super::super::super::models::{TeamSeaking, Teams};

pub fn insert_user(
    conn: &mut PgConnection,
    id_loc: &i64,
    name_loc: &str,
    discord_name: &str,
    description_loc: &str,
) -> TeamSeaking {
    use crate::schema::team_seaking;

    let new_seaker = TeamSeaking {
        id: *id_loc,
        name: String::from(name_loc),
        discordname: String::from(discord_name),
        description: String::from(description_loc),
    };

    diesel::insert_into(team_seaking::table)
        .values(&new_seaker)
        .get_result(conn)
        .expect("Error saving new seaker")
}

pub fn user_delete_row(conn: &mut PgConnection, search: String) -> usize {
    diesel::delete(team_seaking.filter(discordname.like(search)))
        .execute(conn)
        .expect("Error deleting posts")
}

pub fn user_delete_table(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(team_seaking).execute(conn)
}

pub fn user_delete_row_and_return_val(conn: &mut PgConnection, search: i64) -> (TeamSeaking, i64) {
    let mut final_row: TeamSeaking = TeamSeaking {
        id: 0,
        name: String::from(""),
        discordname: String::from(""),
        description: String::from(""),
    };
    let temp = TeamSeaking {
        id: 0,
        name: String::from(""),
        discordname: String::from(""),
        description: String::from(""),
    };
    let mut amt_deleted: i64 = -1;

    for row in team_seaking
        .load::<TeamSeaking>(conn)
        .expect("Could not load table")
    {
        if row.id == search {
            final_row = row;
        }
    }

    if final_row != temp {
        amt_deleted = user_delete_row(conn, format!("%{}%", final_row.discordname)) as i64;
    }

    (final_row, amt_deleted)
}

// Interface for team
pub fn insert_team(
    conn: &mut PgConnection,
    id_loc: &i64,
    name_loc: &str,
    description_loc: &str,
    leader_loc: &str,
) -> Teams {
    use crate::schema::teams;

    let new_seaker = Teams {
        id: *id_loc,
        team_name: String::from(name_loc),
        description: String::from(description_loc),
        leader: String::from(leader_loc),
        members: String::from(""),
        points: 0,
    };

    diesel::insert_into(teams::table)
        .values(&new_seaker)
        .get_result(conn)
        .expect("Error saving new seaker")
}

pub fn team_delete_row(conn: &mut PgConnection, search: String) -> usize {
    diesel::delete(teams.filter(team_name.like(search)))
        .execute(conn)
        .expect("Error deleting posts")
}

pub fn team_delete_table(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(teams).execute(conn)
}

pub fn team_delete_row_and_return_val(conn: &mut PgConnection, search: i64) -> (Teams, i64) {
    let mut final_row = Teams {
        id: 0,
        team_name: String::from(""),
        description: String::from(""),
        leader: String::from(""),
        members: String::from(""),
        points: 0,
    };
    let temp = Teams {
        id: 0,
        team_name: String::from(""),
        description: String::from(""),
        leader: String::from(""),
        members: String::from(""),
        points: 0,
    };
    let mut amt_deleted: i64 = -1;

    for row in teams.load::<Teams>(conn).expect("Could not load table") {
        if row.id == search {
            final_row = row;
        }
    }

    if final_row != temp {
        amt_deleted = team_delete_row(conn, format!("%{}%", final_row.team_name)) as i64;
    }

    (final_row, amt_deleted)
}
