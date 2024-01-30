use std::fs;
use std::fs::{File, metadata, Metadata, OpenOptions};
use std::io::{BufWriter, Read, Write};

pub fn  file_io_test() {

	// read string from file
	let text = fs::read_to_string("resource/1.txt").unwrap();
	println!("resource/1.txt content:{}", text);

	// read binary data from file
	let bytes = fs::read("resource/1.txt").unwrap();
	println!("resource/1.txt content:{:?}", bytes);

	//read file stream from file

	let file_res = fs::metadata("resource/2.txt");

	match file_res {
		Ok(metadata) => {
			if metadata.is_file() {
				// 文件存在，并且是一个普通的文件，可以进行打开和读取操作
				let content = fs::read_to_string("resource/2.txt").expect("Failed to read file");
				println!("file buffer:{:?}", content);

			} else if metadata.is_dir() {
				println!("The path is not a file, is dir");
			} else {
				println!("not a file or dir");
			}
		}

		Err(metadata) => {
			println!("The file does not exist:metadata{:?}", metadata);
		}
	}

	let file_name = "resource/1.txt";

	if let Ok(metadata) = fs::metadata(file_name) {
		if metadata.is_file() {

			let mut file = fs::File::open(file_name).expect("failed to read file");
			let mut buffer = [0u8, 10];
			file.read(&mut buffer).unwrap();
			println!("file buffer:{:?}", buffer);
			file.read(&mut buffer).unwrap();
			println!("file buffer:{:?}", buffer);

		} else if metadata.is_dir() {
			println!("The path is not a file, is dir");
		} else {
			println!("not a file or dir");
		}
	} else {
		println!("The file does not exist");
	}

	let file_dir = "resource";

	if let Ok(metadata) = fs::metadata(file_dir) {
		if metadata.is_dir() {
			println!("file_dir:{} is dir", file_dir);
		}
	}

	//file write

	let mut file_res = file_write();

	if file_res.is_ok() {
		println!("ouput1.txt write OK");
	}
	file_res = create_file_write();

	if file_res.is_ok() {
		println!("ouput2.txt write OK");
	}
	file_res = append_content_to_file();
	if file_res.is_ok() {
		println!("ouput2.txt append OK");
	}

	file_res =  buf_writer_to_file();

	if file_res.is_ok() {
		println!("ouput3.txt buf_writer_to_file OK");
	}
}

fn file_write() ->std::io::Result<()> {
	let file_name_for_write = "ouput1.txt";
	fs::write(file_name_for_write, "hello test.txt");
	Ok(())
}

fn create_file_write() ->std::io::Result<()> {
	println!("---create_file_write---");
	let mut file = File::create("output2.txt")?;
	file.write_all(b"hello rust!")?;
	Ok(())
}

fn append_content_to_file() -> std::io::Result<()> {
	println!("---append_content_to_file---");
	//使用BufWriter进行缓冲写入
	let mut file = OpenOptions::new().append(true).open("output2.txt")?;
	file.write_all(b"\nappend content:123456 ");
	Ok(())
}

fn buf_writer_to_file() -> std::io::Result<()> {
	println!("---buf_writer_to_file---");
	let file = File::create("output3.txt")?;
	let mut buf_writer = BufWriter::new(file);
	buf_writer.write_all(b"I am BufWriter!")?;
	buf_writer.flush()?;
	Ok(())
}


