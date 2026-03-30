// src/templates/contents.rs

pub mod common {
    pub const CARGO_TOML: &str = include_str!("../../tpl/common/cargo_toml.tpl");
    pub const ENV_EXAMPLE: &str = include_str!("../../tpl/common/env_example.tpl");
    pub const CARGO_SMITH: &str = include_str!("../../tpl/common/cargo_smith.tpl");
    pub const MAIN: &str = include_str!("../../tpl/common/main.tpl");
}

pub mod modular {
    pub const MAIN: &str = include_str!("../../tpl/modular/main_server.tpl");
    pub const LIB: &str = include_str!("../../tpl/modular/lib.tpl");
    pub const SERVER: &str = include_str!("../../tpl/modular/server.tpl");
    pub const FEATURE_MOD: &str = include_str!("../../tpl/modular/feature_mod.tpl");
    
    // Feature-specific templates for modular
    pub mod feature {
        /// The main logic/endpoints
        pub const CONTROLLER: &str = include_str!("../../tpl/modular/feature/controller.tpl");
        
        /// The data structures/database logic
        pub const MODEL: &str = include_str!("../../tpl/modular/feature/model.tpl");
        
        /// The route definitions (Actix web::scope)
        pub const SERVICE: &str = include_str!("../../tpl/modular/feature/service.tpl");
        
        /// The 'glue' that exports the module and its init function
        pub const MOD: &str = include_str!("../../tpl/modular/feature/mod_rs.tpl");

        pub const ROUTES: &str = include_str!("../../tpl/modular/feature/routes.tpl");
    }
}