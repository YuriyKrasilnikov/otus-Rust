//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-1.41.1-nightly
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-20

//! Each action is encapsulated into a struct with the trait Command
//! where only the method `execute()` is run.
trait Command {
    fn execute(&self) -> String;
}

// Use a Null struct to initialize the remote control.
struct NullCommand;
impl NullCommand {
    #[allow(dead_code)]
    fn new() -> NullCommand {
        NullCommand
    }
}
impl Command for NullCommand {
    fn execute(&self) -> String {
        format!("Nothing to do!")
    }
}

// The object to handle: a light
#[derive(Copy, Clone)]
struct Light;
impl Light {
    #[allow(dead_code)]
    fn new() -> Light {
        Light
    }
    fn on(&self) -> String {
        format!("Light is on")
    }
    fn off(&self) -> String {
        format!("Light is off")
    }
}

// The first command on the object: light on
struct LightOnCommand {
    light: Light,
}
impl LightOnCommand {
    #[allow(dead_code)]
    fn new(light: Light) -> LightOnCommand {
        LightOnCommand { light }
    }
}
impl Command for LightOnCommand {
    fn execute(&self) -> String {
        self.light.on()
    }
}

// The second command on the object: light off
struct LightOffCommand {
    light: Light,
}
impl LightOffCommand {
    #[allow(dead_code)]
    fn new(light: Light) -> LightOffCommand {
        LightOffCommand { light }
    }
}
impl Command for LightOffCommand {
    fn execute(&self) -> String {
        self.light.off()
    }
}

// The command will be launched by a remote control.
#[allow(dead_code)]
struct SimpleRemoteControl<'a> {
    command: Box<dyn Command + 'a>,
}
impl<'a> SimpleRemoteControl<'a> {
    #[allow(dead_code)]
    fn new() -> SimpleRemoteControl<'a> {
        SimpleRemoteControl {
            command: Box::new(NullCommand::new()),
        }
    }
    #[allow(dead_code)]
    fn set_command(&mut self, cmd: Box<dyn Command + 'a>) {
        self.command = cmd;
    }
    #[allow(dead_code)]
    fn button_was_pressed(&self) -> String {
        self.command.execute()
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        let mut remote = SimpleRemoteControl::new();
        let light = Light::new();
        let light_on = LightOnCommand::new(light);
        let light_off = LightOffCommand::new(light);

        assert_eq!(remote.button_was_pressed(), "Nothing to do!".to_string());
        remote.set_command(Box::new(light_on));
        assert_eq!(remote.button_was_pressed(), "Light is on".to_string());
        remote.set_command(Box::new(light_off));
        assert_eq!(remote.button_was_pressed(), "Light is off".to_string());
    }
}
