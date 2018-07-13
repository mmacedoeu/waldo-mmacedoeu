#![cfg_attr(feature = "flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature = "flame_it", plugin(flamer))]

extern crate actix;
extern crate cv;
extern crate env_logger;
extern crate failure;
#[cfg(feature = "flame_it")]
extern crate flame;
extern crate futures;
extern crate tokio;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

mod actors;
mod cli;
mod errors;

quick_main!(run);

fn run() -> errors::Result<()> {
	cli::run(::std::env::args())
}
