use crate::communication::Subscribe;
use std::net::TcpStream;
use std::{env, fs, process};

pub mod communication;
pub mod hashcash;
pub mod montrous_maze;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     println!("Le nombre d'argument est invalide, le client n'a besoin que d'un seul argument: l'adresse du serveur");
    //     process::exit(1);
    // }
    //
    // let server_address = &args[1];
    // let stream = TcpStream::connect(server_address.clone() + ":7878");
    //
    // match stream {
    //     Ok(stream) => {
    //         // TODO: communication logic
    //     }
    //     Err(err) => {
    //         println!("Une erreur est survenue: {}", err.to_string());
    //         process::exit(1);
    //     }
    // }

    let s = Subscribe {
        name: "Shan".to_string(),
    };

    let v = serde_json::to_string(&s).unwrap();
    println!("{}", v);
}

// HASHCASH
// Simulate retrieved data
//     let input = MD5HashCashInput {
//         complexity: 9,
//         message: String::from("hello")
//     };
//
//     let output = find_seed(&input);
//     println!("seed : {}, hash : {}", u64_to_hexadecimal_string(output.seed), output.hashcode);
// let contenu = fs::read_to_string("maze.txt")
//     .expect("Quelque chose s'est mal passé lors de la lecture du fichier");
//
// let input = MonstrousMazeInput {
//     grid: "|Y M X|".to_string(),
//     // grid: contenu,
//     endurance: 2
// };
//
// find_path(&input);
