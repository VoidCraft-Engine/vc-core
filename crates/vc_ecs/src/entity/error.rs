use core::error::Error;
use core::fmt;
use core::panic::Location;

use crate::utils::DebugLocation;

use super::{Entity, EntityGeneration};

// -----------------------------------------------------------------------------
// InvalidEntityError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidEntityError {
    pub entity: Entity,
    pub current_generation: EntityGeneration,
}

impl fmt::Display for InvalidEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The entity with ID {} is invalid; its index now has generation {}.",
            self.entity, self.current_generation,
        )
    }
}

impl Error for InvalidEntityError {}

// -----------------------------------------------------------------------------
// SpawnError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnError {
    Invalid(InvalidEntityError),
    AlreadySpawned,
}

impl fmt::Display for SpawnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpawnError::Invalid(invalid_entity_error) => {
                write!(f, "Invalid id: {}", invalid_entity_error)
            }
            SpawnError::AlreadySpawned => {
                f.write_str("The entity can not be spawned as it already has a location.")
            }
        }
    }
}

impl Error for SpawnError {}

// -----------------------------------------------------------------------------
// ValidEntityButNotSpawnedError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidEntityButNotSpawnedError {
    pub entity: Entity,
    pub location: DebugLocation<&'static Location<'static>>,
}

impl fmt::Display for ValidEntityButNotSpawnedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The entity with ID {} is not spawned; ", self.entity)?;

        match self.location.into_option() {
            Some(location) => write!(f, "its index was last despawned by {location}."),
            None => write!(f, "enable `track_location` feature for more details."),
        }
    }
}

impl Error for ValidEntityButNotSpawnedError {}

// -----------------------------------------------------------------------------
// NotSpawnedError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotSpawnedError {
    /// The entity was invalid.
    Invalid(InvalidEntityError),
    /// The entity was valid but was not spawned.
    ValidButNotSpawned(ValidEntityButNotSpawnedError),
}

impl fmt::Display for NotSpawnedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotSpawnedError::Invalid(invalid_entity) => {
                writeln!(f, "Entity despawned: {}", invalid_entity)?;
                f.write_str("Maybe interacting with a despawned entity (or other reason).\n")
            }
            NotSpawnedError::ValidButNotSpawned(not_spawned) => {
                writeln!(f, "Entity not yet spawned: {}", not_spawned)?;
                f.write_str("Maybe interacting with a not-yet-spawned entity (or other reason).\n")
            }
        }
    }
}

impl Error for NotSpawnedError {}

impl NotSpawnedError {
    pub fn entity(&self) -> Entity {
        match self {
            NotSpawnedError::Invalid(err) => err.entity,
            NotSpawnedError::ValidButNotSpawned(err) => err.entity,
        }
    }
}
