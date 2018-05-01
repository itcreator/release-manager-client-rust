mod error;
pub use self::error::Error;
mod incremental_version_number;
pub use self::incremental_version_number::IncrementalVersionNumber;
mod project;
pub use self::project::Project;
mod semver_generate_params;
pub use self::semver_generate_params::SemverGenerateParams;
mod semver_tag_set;
pub use self::semver_tag_set::SemverTagSet;

// TODO(farcaller): sort out files
pub struct File;
