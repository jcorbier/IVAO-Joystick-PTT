use std::fmt;
use xplm::command::*;
use xplm::plugin::{Plugin, PluginInfo};

macro_rules! xdebug {
    ($($arg:tt)*) => ({
        xplm::debugln!("IVAO Joystick PTT - {}", format_args!($($arg)*));
    })
}

mod config;
mod input;

#[derive(Debug)]
struct PluginError(String);

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for PluginError {}

struct PttHandler {
    key: String,
    input_manager: input::InputManager,
}

impl CommandHandler for PttHandler {
    fn command_begin(&mut self) {
        xdebug!("Pressing {}", self.key);
        self.input_manager.press_key(&self.key);
    }

    fn command_continue(&mut self) {
        // Op is held
    }

    fn command_end(&mut self) {
        xdebug!("Releasing {}", self.key);
        self.input_manager.release_key(&self.key);
    }
}

struct IvaoPttPlugin {
    #[allow(dead_code)]
    config: config::Config,
    _owned_command: OwnedCommand,
}

impl Plugin for IvaoPttPlugin {
    type Error = PluginError;

    fn start() -> Result<Self, Self::Error> {
        xdebug!("Starting plugin...");
        let config = config::load_config();
        xdebug!("Loaded config with key: {}", config.key);

        let input_manager = input::InputManager::new();

        let handler = PttHandler {
            key: config.key.clone(),
            input_manager,
        };

        // owned command
        let owned_command =
            OwnedCommand::new("ivao_ptt/push_to_talk", "IVAO Push-to-Talk", handler)
                .map_err(|e| PluginError(format!("Creation error: {:?}", e)))?;

        xdebug!("Command registered.");

        Ok(IvaoPttPlugin {
            config,
            _owned_command: owned_command,
        })
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "IVAO Joystick PTT".to_string(),
            signature: "com.jcorbier.ivao_ptt".to_string(),
            description: "PTT Plugin".to_string(),
        }
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        xdebug!("Enabled.");
        Ok(())
    }

    fn disable(&mut self) {
        xdebug!("Disabled.");
    }
}

xplm::xplane_plugin!(IvaoPttPlugin);
