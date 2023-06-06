//! Rust Types.

use crate::sql_types::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use postgis::ewkb::Point;
use std::convert::From;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geometry"]
pub struct GeomPoint {
    pub x: f64, // lon
    pub y: f64, // lat
    pub srid: Option<i32>,
}

impl From<Point> for GeomPoint {
    fn from(p: Point) -> Self {
        let Point { x, y, srid } = p;
        Self { x, y, srid }
    }
}
impl From<GeomPoint> for Point {
    fn from(p: GeomPoint) -> Self {
        let GeomPoint { x, y, srid } = p;
        Self { x, y, srid }
    }
}

impl FromSql<Geometry, Pg> for GeomPoint {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geometry, Pg> for GeomPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}
