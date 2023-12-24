use clap::Parser;

use async_once::AsyncOnce;
use lazy_static::lazy_static;

use std::path::Path;

use crate::{
	load_claster::load_claster,
	structs::ClasterSpace,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
	#[arg(long)]
	pub db_name: Box<str>,
	#[arg(long)]
	pub claster_path: Box<Path>,
	#[arg(long)]
	pub tg_token: Option<String>,
}

lazy_static! {
	pub static ref CONFIG: Config = {
		Config::parse()
	};
	pub static ref CLASTER_SPACE: AsyncOnce<ClasterSpace> = AsyncOnce::new( async {
		load_claster(&CONFIG.claster_path).await.unwrap_or_else(|error| {
			eprintln!("Error: {:#?}", error);
			std::process::exit(0x100);
		})
	});
} 
