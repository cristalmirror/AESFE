use std::env;
use std::io::{Read, Write, BufRead, BufReader};
use std::fs::File;
use aes::Aes128;
use std::error::Error;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt,
    KeyInit, generic_array::GenericArray,
};
use typenum::U16;
use std::io::Result;
use colored::*;
use rand::Rng;

const BLOCK_SIZE: usize = 16;


/*Objet that content
all tools to encrypt
and decrypt*/
struct EncryptDecryptObject {
    key: GenericArray<u8, U16>,
    block: GenericArray<u8, U16>,
    blocks:Vec<Vec<u8>>,
}

/*methods*/
impl EncryptDecryptObject {

    pub fn new() -> Self {
	let key = GenericArray::from([0u8; 16]);
	let block = GenericArray::from([42u8; 16]);
	let blocks = vec![];
	
	EncryptDecryptObject {key, block, blocks}
    }

    pub fn open_file(&mut self,file_name: &str) -> Result<Vec<Vec<u8>>> {
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);

	//process to part in blocks
	for _block in reader.lines() {
	    match _block {
		Ok(line) => {
		    let chars: Vec<_> = line.chars().collect();
		    let chunks = chars.chunks(16);
		    for chunk in chunks {
			let bytes: Vec<u8> = chunk.iter().map(|&c| c as u8).collect();
			&self.blocks.push(bytes);
		    }
		}
		Err(err) => {
		    eprintln!("[AESFE]: ***ERROR*** in line: {}",err);
		}
	    }
	}
	Ok(self.blocks.clone())
    }

    //generator of key
    pub fn generate_random_key() -> GenericArray<u8, U16> {
	let mut rng = rand::rngs::OsRng::default();
	let mut key = GenericArray::from([0u8; 16]);
	rng.fill(&mut key[..]);
	key
    }
    
   //calcula el porcentaje de bloques procesados
   fn porcent_of_process_unit(&mut self, tam: usize) {
	let message = "[AESFE]: *BLOCKS*";
	let colorMes = message.blue();
	let mut result = (tam * 100) / self.blocks.len();
	print!("{} {:?} %", colorMes, result);
    }
    
    /*function create to encrypt*/
    pub fn encrypt(&mut self) -> Result<Vec<Vec<u8>>> {

	//load key
	self.key = Self::generate_random_key();
	
	// Initialize cipher
	let cipher = Aes128::new(&self.key);

	// Create or open the output file in write mode
        let mut output_file = File::create("bin_output.bin")?;

	//we create a vec for allocate the cipher blocks
	let mut encrypted_blocks = Vec::new();
	let mut cont: usize = 0;

	//cipher each block of data
	for _block in &self.blocks {

	    let mut padded_block = _block.clone(); // Copia el bloque original
	    if padded_block.len() < BLOCK_SIZE {
		// Si el bloque es más corto que 16 bytes, rellenarlo con ceros
		padded_block.resize(BLOCK_SIZE, 0u8);
	    } else if padded_block.len() > BLOCK_SIZE {
		// Si el bloque es más largo que 16 bytes, truncarlo a 16 bytes
		padded_block.truncate(BLOCK_SIZE);
	    }
	    
	    let mut encrypted_data =GenericArray::clone_from_slice(&padded_block[..]);
	    cipher.encrypt_block(&mut encrypted_data);
	    encrypted_blocks.push(encrypted_data.to_vec());

	    // Write the encrypted block to the output file
            output_file.write_all(&encrypted_data)?;
	}
	
	Ok(encrypted_blocks)
    }

 
}

/*main function*/
fn main() {
    //definicion la instancia del objeto encriptador decriptador
    let mut estructura = EncryptDecryptObject::new();
    let mut text1 = "[AESFE]: ***TEXT PART***";
    let mut text2 =  "[AESFE]: ***KEY*** -> ";
    let arg2 =  env::args().nth(2);
    let name_text: &str = arg2.as_deref().unwrap_or_default();
    
    //tomamos los valores de los argumentos
    match env::args().nth(1).as_deref() {
	Some("-e")  => {
	    //init the process to part the archive in blocks
	    match estructura.open_file(name_text) {
		Ok(blocks) => {
		    for block in blocks {
			println!( "{} - {:?}",text1.green(), block);
		    }
		}
		Err(err) => {
		    eprintln!("[AESFE]: Error al procesar el archivo: {}", err);
		}
	    }
	    
	    //init the process to encypt the blocks archive
	    match estructura.encrypt() {
		Ok(encrypted_blocks) => {
		    
		    //pocesa el total de los bloques
		    let mut cont: usize = 0;
		    for block in &encrypted_blocks {
			estructura.porcent_of_process_unit(cont);
			println!(" - Cipher Block: {:?}", block);
			cont += 1;
		    }
		    println!("{} {:?}",text2.red(),estructura.key);
		} Err(err) => {
	    	    println!("[AESFE]: ERROR in cipher process: {:?}", err);
		}
	    }
	},
	Some("-d") => {
	  
	},
	Some(_) => {
	    let message_error = "[AESFE]: ***ERROR*** no have args corrects";
	    let color_mess = message_error.red();
	    println!("{}", color_mess);
	},
	None => {
	    println!("[AESFE] ***ERROR*** not args");
	    println!("[AESFE] --> key: {:?}",estructura.key);
	    println!("[AESFE] --> block: {:?}",estructura.block);
	    println!("[AESFE] --> blocks: {:?}",estructura.blocks);
	}
    }
}
