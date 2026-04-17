use std::{collections::HashMap, sync::Arc};

use crate::cmd::command;

mod cat;
mod cd;
mod chmod;
mod clear;
mod echo;
mod exit;
mod ls;
mod pwd;
mod rm;

pub fn get_commands() -> HashMap<String, Arc<command::Command>> {
    let mut commands = HashMap::new();

    let inbuilts = vec![
        Arc::new(cat::register_command()),
        Arc::new(cd::register_command()),
        Arc::new(chmod::register_command()),
        Arc::new(echo::register_command()),
        Arc::new(exit::register_command()),
        Arc::new(ls::register_command()),
        Arc::new(pwd::register_command()),
        Arc::new(rm::register_command()),
        Arc::new(clear::register_command()),
    ];

    for cmd in inbuilts {
        if cmd.is_casesensitive() {
            commands.insert(cmd.get_name(), cmd.clone());
        } else {
            commands.insert(cmd.get_name().to_lowercase(), cmd.clone());
        }

        let alias = cmd.get_alias();
        if !alias.is_empty() {
            for a in alias {
                if cmd.is_casesensitive() {
                    commands.insert(a, cmd.clone());
                } else {
                    commands.insert(a.to_lowercase(), cmd.clone());
                }
            }
        }
    }

    commands
}
