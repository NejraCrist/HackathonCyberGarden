use conception::{
	Flashcard,
	ClasterInfo,
	Claster,
};

use std::borrow::Cow;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::io::{
	stdin,
	Write,
	Read,
};

use std::fs::File;

fn input<'a>(text: &str) -> Cow<'a, str> {
	let mut line = String::new();
	println!("{}", text);
	let _ = stdin().read_line(&mut line);
	#[cfg(target_os = "linux")]
	let line = line[0..(line.len()-1)].to_string();
	#[cfg(target_os = "windows")]
	let line = line[0..(line.len()-2)].to_string();
	Cow::Owned(line)
}

fn calculate_hash<T: Hash>(value: &T) -> u64 {
	let mut hasher = DefaultHasher::new();
	value.hash(&mut hasher);
	hasher.finish()
}

fn write_to_file(data: &str) -> Result<String, Box<dyn std::error::Error>> {
	let name_file = format!("IM__{:X}.json", calculate_hash(&data));
	let mut file = File::create(&name_file)?;
	file.write_all(data.as_bytes())?;
	Ok(name_file)
}

fn read_file(name_file: &str) -> Result<String, Box<dyn std::error::Error>> {
	let mut buf = String::new();
	let mut file = File::open(name_file)?;
	file.read_to_string(&mut buf)?;
	Ok(buf)
}

fn main() {
	let info = ClasterInfo::new(
		input("Введите тему карточек"),
		input("Доп информация"),
		{
			println!("Введите источники\nВведите \"exit\" для выхода из ввода источников");
			let mut result = Vec::new();
			let mut exit = false;
			let mut index = 0;
			while exit != true {
				index += 1;
				let data = input(format!("Источник {}:", index).as_str());
				if data == "exit" {
					exit = true;
				}
				else {
					result.push(data)
				}
			}
			result
		}
	);
	let flash_cards = {
		println!("Воод карточек\nВведите \"exit\" для выхода из ввода источников");
		
		let mut result = Vec::new();
		let mut exit = false;
		let mut index = 0;
		
		while exit != true {
			index += 1;
			let data = input(format!("Карточка {}\nВопрос карточки:", index).as_str());
			if data == "exit" {
				exit = true;
			}
			else {
				result.push(Flashcard::new(data, input("Ответ")))
			}
		}
		result
	};
	let claster = Claster::new(
		info,
		flash_cards,
	);
	if let Ok(data) = serde_json::to_string(&claster) {
		println!("{}", data);
		
		let try_name = write_to_file(&data);
		println!("{:#?}", try_name);
		
		if let Ok(name_file) = try_name {
			if let Ok(text) = read_file(&name_file) {
				println!("file data: {:#?}", serde_json::from_str::<Claster>(&text) );
			}
		}
	}
}
