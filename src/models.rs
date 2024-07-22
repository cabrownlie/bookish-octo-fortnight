use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::hunts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Hunts {
	pub id: i32,
	pub title: String,
	pub locations: Vec<Locations>,
	pub published: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Locations {
	pub id: i32,
	pub title: String,
	pub position: (f32, f32),
}
