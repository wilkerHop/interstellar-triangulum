pub mod assets;
pub mod parser;
pub mod renderer;
pub mod script;
pub mod audio;

pub use assets::AssetLoader;
pub use parser::ScriptParser;
pub use renderer::{Compositor, FrameBuffer, RenderEngine, Timeline};
pub use script::VideoScript;
pub use audio::{AudioDecoder, AudioMixer};
