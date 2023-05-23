use lazy_static::lazy_static;

use crate::model::Config;
use crate::model::create_config_from_env;

lazy_static! {
    pub static ref CONFIG: Config = create_config_from_env();
}
