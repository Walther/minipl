/// Runtime [Object] type of the Mini-PL language
mod object;
pub use object::Object;
/// Runtime [Environment] type of the Mini-PL language, used for variable storage
mod environment;
pub use environment::Environment;
mod errors;
pub use errors::RuntimeError;
