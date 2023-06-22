pub use crate::cozy::bindings_wrapper::*;

// Math
pub static SECONDS_IN_YEAR: u128 = 365 * 24 * 3600;

// Static sim update tags
pub static SET_DEPLOYMENT: &str = "Set Deployment";

// Agent names
pub static COST_MODELS_DEPLOYER: &str = "Cost Models Deployer";
pub static DRIP_DECAY_MODELS_DEPLOYER: &str = "Drip Decay Models Deployer";
pub static TRIGGERS_DEPLOYER: &str = "Triggers Deployer";
pub static WETH_DEPLOYER: &str = "wETH Deployer";
pub static BASE_TOKEN_DEPLOYER: &str = "Base Token Deployer";
pub static PROTOCOL_DEPLOYER: &str = "Protocol Deployer";
pub static SET_ADMIN: &str = "Set Admin";
pub static PASSIVE_SUPPLIER: &str = "Passive Supplier";
pub static PASSIVE_BUYER: &str = "Passive Buyer";
pub static BASE_TOKEN: &str = "Cozy Base Token";
