use std::str::FromStr;

use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Project<D> {
    pub id: Id,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    pub data: D,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub const fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.simple().to_string()
    }
}

#[derive(Debug, Error)]
#[error("Invalid project ID")]
pub struct IdParseError;

impl FromStr for Id {
    type Err = IdParseError;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        let uuid = from.parse::<Uuid>().map_err(|_| IdParseError)?;
        Ok(Self(uuid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    const EXAMPLE_ID: Uuid = uuid!("157296c3-4a0c-4794-91bb-34008da55535");

    #[test]
    fn id_to_string() {
        let id = Id::from_uuid(EXAMPLE_ID);
        assert_eq!(id.to_string(), "157296c34a0c479491bb34008da55535");
    }

    #[test]
    fn parse() {
        let id = "157296c3-4a0c-4794-91bb-34008da55535"
            .parse::<Id>()
            .unwrap();
        assert_eq!(id.0, EXAMPLE_ID);

        let id = "157296c34a0c479491bb34008da55535".parse::<Id>().unwrap();
        assert_eq!(id.0, EXAMPLE_ID);

        let id = "157296C34A0C479491BB34008DA55535".parse::<Id>().unwrap();
        assert_eq!(id.0, EXAMPLE_ID);
    }
}
