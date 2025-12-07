/// cmd houses entry functions for the cli subcommands.
pub mod cp;
pub mod edit;
pub mod generate;
pub mod ls;
pub mod new;
pub mod rm;

pub use cp::cp;
pub use edit::edit;
pub use generate::generate;
pub use ls::list;
pub use new::new;
pub use rm::rm;
