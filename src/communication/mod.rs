use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use crate::hashcash::{find_seed, MD5HashCashInput, MD5HashCashOutput};
use crate::montrous_maze::{find_path, MonstrousMazeInput, MonstrousMazeOutput};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message{
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    pub version: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult{
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge{
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult{
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeTimeout {
    message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer{
    MD5HashCash(MD5HashCashOutput),
    MonstrousMaze(MonstrousMazeOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult{
    name: String,
    value: ChallengeValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary{
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame{
    leader_board: Vec<PublicPlayer>
}

pub fn send_message(stream: &mut TcpStream, message: &Message) {
    println!("Sending message: {:?}", message);
    let message = serde_json::to_string(&message).expect("Failed to serialize message");
    println!("Serialized message: {}", message);
    stream.write_all(&(message.len() as u32).to_be_bytes()).expect("Failed to write message length");
    stream.write_all(message.as_bytes()).expect("Failed to write message");
}

pub fn receive_message(stream: &mut TcpStream) -> Message {
    println!("Receiving message");
    let mut buffer = [0 as u8; 4];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Received message length: {}", u32::from_be_bytes(buffer));
            let buffer: u32 = u32::from_be_bytes(buffer);
            let mut new_buffer = vec![0; buffer as usize];
            match stream.read_exact(&mut new_buffer) {
                Ok(_) => {
                    let message = from_utf8(&new_buffer).expect("Failed to read message");
                    println!("Received message: {}", message);
                    return serde_json::from_str(message).expect("Failed to deserialize message");
                }
                Err(err) => panic!("Failed to read message: {}", err),
            }
        },
        Err(e) => {
            println!("Failed to read message: {}", e);
        }
    }
    Message::Hello
}

pub fn message_challenge(stream: &mut TcpStream, challenge: Challenge, leaderboard: &Vec<PublicPlayer>, current_player: usize){
    let mut rng = thread_rng();
    let mut index = rng.gen_range(0..leaderboard.len());
    let challenge_answer: ChallengeAnswer;

    match challenge {
        Challenge::MD5HashCash(input) => {
            println!("Hashcash message : {:?}", &input);
            let answer = find_seed(&input);
            println!("Hashcash answer : {:?}", &answer);
            challenge_answer = ChallengeAnswer::MD5HashCash(answer);
        }
        Challenge::MonstrousMaze(input) => {
            let answer = find_path(&input);
            challenge_answer = ChallengeAnswer::MonstrousMaze(answer);
        }
        _ => unreachable!()
    }

    if leaderboard.len() > 1 {
        while current_player == index {
            index = rng.gen_range(0..leaderboard.len());
        }
    }


    let challenge_result = ChallengeResult {
        answer: challenge_answer,
        next_target: leaderboard[index].name.clone()
    };

    println!("Sending challenge result: {:?}", challenge_result);

    send_message(stream, &Message::ChallengeResult(challenge_result));
}