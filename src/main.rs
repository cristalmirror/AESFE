use std::env;
use std::io::{Read,Write};
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
	let blocks = vec![ "hola mundo, esto".as_bytes().to_vec(),  // Primer bloque de texto
			    "y está encriptado".as_bytes().to_vec(), // Segundo bloque de texto
	];
	
	EncryptDecryptObject {key, block, blocks}
    }

    pub fn open_file(&self,) {

    }

    //generator of key
    pub fn generate_random_key() -> GenericArray<u8, U16> {
	let mut rng = rand::rngs::OsRng::default();
	let mut key = GenericArray::from([0u8; 16]);
	rng.fill(&mut key[..]);
	key
    }
    
   //calcula el porcentaje de bloques procesados
   fn porcent_of_process_unit(&self, tam: usize) {
	let message = "[ARCHIVE CRYPT]: *BLOCKS*";
	let colorMes = message.blue();
	let mut result = (tam * 100) / self.blocks.len();
	println!("{} {:?} %", colorMes, result);
    }
    
    /*function create to encrypt*/
    pub fn encrypt(&mut self) -> Result<Vec<Vec<u8>>> {

	//load key
	self.key = Self::generate_random_key();
	println!("{:?}",&self.key);
	// Initialize cipher
	let cipher = Aes128::new(&self.key);
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
	}
	Ok(encrypted_blocks)
    }

  /*  fn decrypt(&self) {

    }*/
}

/*main function*/
fn main() {
    //definicion la instancia del objeto encriptador decriptador
    let mut estructura = EncryptDecryptObject::new();

    //tomamos los valores de los argumentos
    match env::args().nth(1).as_deref() {
	Some("-e")  => {
	    match estructura.encrypt() {
		Ok(encrypted_blocks) => {
		    
		    //pocesa el total de los bloques
		    let mut cont: usize = 0;
		    for block in &encrypted_blocks {
			println!("[ARCHIVE CRYPT]: Cipher Block: {:?}", block);
			estructura.porcent_of_process_unit(cont);
			cont += 1;
		    }
		} Err(err) => {
	    	    println!("[ARCHIVE CRYPT]: ERROR in cipher process: {:?}", err);
		}
	    }
	},
	Some("-d") => {
	    
	},
	Some(_) => {
	    let message_error = "[ARCHIVE CRYPT]: ***ERROR*** no have args corrects";
	    let color_mess = message_error.red();
	    println!("{}", color_mess);
	},
	None => {
	    println!("[ARCHIVE CRYPT] ***ERROR*** not args");
	    println!("[ARCHIVE CRYPT] --> key: {:?}",estructura.key);
	    println!("[ARCHIVE CRYPT] --> block: {:?}",estructura.block);
	    println!("[ARCHIVE CRYPT] --> blocks: {:?}",estructura.blocks);
	}
    }
}
