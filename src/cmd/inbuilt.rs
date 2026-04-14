use std::{collections::HashMap, sync::Arc};

use crate::cmd::command;

mod exit;
mod ls;

pub fn get_commands() -> HashMap<String, Arc<command::Command>> {
    let mut commands = HashMap::new();

    let inbuilts = vec![
        //
        Arc::new(exit::register_command()),
    ];

    for cmd in inbuilts {
        if cmd.is_casesensitive() {
            commands.insert(cmd.get_name(), cmd.clone());
        } else {
            commands.insert(cmd.get_name().to_lowercase(), cmd.clone());
        }

        let alias = cmd.get_alias();
        if alias.len() > 0 {
            for a in alias {
                if cmd.is_casesensitive() {
                    commands.insert(a, cmd.clone());
                } else {
                    commands.insert(a.to_lowercase(), cmd.clone());
                }
            }
        }
    }

    return commands;
}
