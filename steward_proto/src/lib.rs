pub mod uniswapv3 {
    include!("prost/cellars.uniswapv3.v1.rs");

    pub use uniswap_v3_cellar_handler_client as client;
    pub use uniswap_v3_cellar_handler_server as server;
}