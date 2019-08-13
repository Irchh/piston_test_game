use roxmltree::Document;

pub fn load_map(path: &str, app: &mut crate::App, mut window: &mut piston_window::PistonWindow) {
	let map = load_tmx(path);
	let map_width = map[map.len()-1] as usize;
	use crate::collision::Cube;
	let size = crate::METER_IN_PIXELS;
	app.world.w = (map_width-1) as f64*size;
	app.world.h = ((map.len()-1)%map_width) as f64;
	// For each element in map create and push an equivalent Cube element to the game world.
	for i in 0..map.len()-1 {
		if map[i] != 0 {
			let x = ((i%map_width) as f64)*size;
			let y = ((i/map_width) as f64)*size;
			let cube = Cube::new(size, size, x, y, "assets/sprites/brick.png", &mut window);
			app.obj.push(cube);
		};
	}
}

// Load a map file and parse it to generate a vector of files
fn load_tmx(path: &str) -> Vec<u8> {
	let teststr = std::fs::read_to_string(path).unwrap();
	let doc = Document::parse(&teststr).unwrap();

	let mut map_string: String = String::new();
	let mut is_map = false;
	// Iterate through xml document and check for map data, if not then panic.
	for node in doc.descendants() {
		if node.has_tag_name("data") {
			map_string = String::from(node.first_child().unwrap().text().unwrap());
		}else if node.has_tag_name("map") {
			is_map = true;
		}
	}

	if !is_map {
		panic!("{:?} is not a valid map file!", path);
	}
	
	let mut map_vector: Vec<u8> = Vec::new();
	let mut obj = 0;
	let (mut map_width, mut comma_count) = (0, 0);
	// Convert the map data into a vector of u8
	for byte in map_string.bytes() {
		match byte {
			10 => {
				// Calculate map width.
				if map_width == 0 {
					map_width = comma_count;
				}
			},
			44 => {
				// Only push if a comma is encountered
				map_vector.push(obj);
				obj = 0;
				comma_count += 1;
			},
			_ => {
				// Crash if encountered byte is not a base10 number.
				if byte-48 < 0 || byte-48 > 9 { panic!("Byte is not number! {:?}", byte); };

				obj += byte-48;
			},
		}
	}

	// Push one last object because the data part of the tmx file doesn't end with a comma
	// which means that the last object wont get pushed.
	map_vector.push(obj);
	// Append map width to end of map_vector because there is no other way to get it.
	map_vector.push(map_width);
	map_vector
}