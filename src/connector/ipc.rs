use crate::connector::Connector;
use crate::config::Config;

use shared_memory::*;

extern crate shared_memory;

pub struct Ipc {
    shmem: Shmem,
}

impl Connector for Ipc {
    fn from_config(config: Config) -> Self where Self: Sized {
        let config = config.connector.ipc;
        let path = config.game_id;
        //TODO be smarter
        //let rand_num: u16 = rand::random();
        //let path = format!("{}{}", path, rand_num);
        println!("Path: {}", path);

        let mut is_creator = true;
        let shmem = match ShmemConf::new().size(4096).flink(&path).create() {
            Ok(m) => m,
            Err(ShmemError::LinkExists) => {
                is_creator = false;
                ShmemConf::new().flink(&path).open().unwrap()
            },
            Err(e) => panic!("Unable to create IPC connection: {}", e),
        };

        let mem = shmem.as_ptr();
        if is_creator {
            unsafe {
                *mem.offset(0) = 0;
                *mem.offset(1) = 0;
            }
        }

        Ipc { shmem }
    }

    fn get_next_move_list(&mut self) -> Vec<String> {
        let mut moves = Vec::new();
        let mem = self.shmem.as_ptr();

        unsafe {
            let count = *mem.offset(0);
            for i in 0..count {
                let idx = ((i + 1) * 2) as isize;
                let first_char = *mem.offset(idx) as char;
                let second_char = *mem.offset(idx + 1) as char;
                moves.push(format!("{}{}", first_char, second_char));
            }
        }

        moves
    }

    fn submit_move(&mut self, new_move: &String) {
        println!("TODO: I'M GONNA SUBMIT IT! {}", new_move);

        let mem = self.shmem.as_ptr();
        unsafe {
            let count = *mem.offset(0) + 1;
            *mem.offset(0) = count;

            let bytes = new_move.as_bytes();
            let idx = (count * 2) as isize;
            *mem.offset(idx) = bytes[0];
            *mem.offset(idx+1) = bytes[1];
        }
    }
}