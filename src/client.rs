use crate::communication::{Message, message_challenge, PublicPlayer, receive_message, send_message, Subscribe};
use std::net::TcpStream;
use std::{env, fs, process};
use crate::hashcash::{find_seed, MD5HashCashInput, u64_to_hexadecimal_string};

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
    let stream = TcpStream::connect("localhost".to_owned() + ":7878");

    let mut board: Vec<PublicPlayer> = vec![];
    let mut current_player = 0;
    let player_name = "shan".to_string();
    match stream {
        Ok(mut stream) => {
            println!("LOG : Connected to server");
            println!("#######################################################\n");
            send_message(&mut stream, &Message::Hello);
            loop {
                let record = receive_message(&mut stream);
                println!("{:?}", record);
                match record {
                    Message::Hello => {}
                    Message::Welcome(message) => {
                        println!("Welcome to the server, version {}", message.version);
                        send_message(&mut stream, &Message::Subscribe(Subscribe { name: player_name.clone() }));
                    }
                    Message::SubscribeResult(subscribe) => {
                        println!("{:?}", subscribe);
                    },
                    Message::PublicLeaderBoard(leaderboard) => {
                        println!("{:?}", leaderboard);
                        board = leaderboard;
                    },
                    Message::Challenge(challenge) => {
                        message_challenge(&mut stream, challenge, &board, current_player);
                        current_player += 1;
                        if current_player > board.len() - 1 {
                            current_player = 0;
                        }
                    },
                    Message::Subscribe(subscribe) => {
                        println!("Suscribe: {:?}", subscribe);
                    }
                    Message::ChallengeResult(challenge_result) => {
                        println!("Challenge result received: {:?}", challenge_result);
                    },
                    Message::ChallengeTimeout(timeout) => {
                        println!("Challenge timeout: {:?}", timeout);
                    }
                    Message::RoundSummary(round_summary) => {
                        println!("{:?}", round_summary);
                    },
                    Message::EndOfGame(end_of_game) => {
                        println!("{:?}", end_of_game);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            println!("Une erreur est survenue: {}", err.to_string());
            process::exit(1);
        }
    }
}

// HASHCASH
// fn main() {
//     let input = MD5HashCashInput {
//         complexity: 16,
//         message: String::from("Your smart cat talks to your giant guitar.")
//     };
//
//     let output = find_seed(&input);
//     println!("seed : {:?}, hash : {}", output.seed, output.hashcode);
    // let contenu = fs::read_to_string("maze.txt")
    //     .expect("Quelque chose s'est mal passé lors de la lecture du fichier");
    //
    // let input = MonstrousMazeInput {
    //     grid: "|Y M X|".to_string(),
    //     // grid: contenu,
    //     endurance: 2
    // };
// }
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
