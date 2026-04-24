use crate::system::permission::Permissions;

pub fn init_world() {
    self.create_dir_absolute("/home", "root", Permissions::directory_default())?;
    self.create_dir_absolute("/home/player", "player", Permissions::directory_default())?;
    self.create_file_absolute(
        "/home/player/readme.txt",
        "player",
        "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem.".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/home/player/aaaaaa",
        "player",
        "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem.".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/home/player/bbbbbb",
        "player",
        "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem.".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/home/player/ffffff",
        "player",
        "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem.".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/home/player/zzzzzz",
        "player",
        "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem.".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/home/player/yyyyyy",
        "player",
        "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem.".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/home/player/qqqqqqq",
        "player",
        "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem.".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/home/player/notes.txt",
        "player",
        "Bits are your carrying capacity.\n".to_string(),
        Permissions::file_default(),
    )?;
    self.create_dir_absolute("/player", "root", Permissions::directory_default())?;
    self.create_dir_absolute("/player/memory", "player", Permissions::directory_default())?;
    self.create_dir_absolute("/monster", "root", Permissions::directory_default())?;
    self.create_dir_absolute("/monster/slime", "root", Permissions::directory_default())?;
    self.create_file_absolute(
        "/monster/slime/hp",
        "root",
        "12\n".to_string(),
        Permissions::file_default(),
    )?;
    self.create_file_absolute(
        "/monster/slime/ai.sh",
        "root",
        "echo slime attacks\n".to_string(),
        Permissions::executable_file(),
    )?;
    self.create_dir_absolute("/etc", "root", Permissions::directory_default())?;
}
