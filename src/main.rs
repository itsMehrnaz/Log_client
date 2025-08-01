mod packet;
use packet::LogPacket;

fn main(){
    let packet = LogPacket{
        level:"info".to_string(),
        message: "hello".to_string(),
        timestamp: "12:34".to_string()
    };


    let encoded: Vec<u8> = bincode::serialize(&packet).unwrap();
    println!("encode is:{:?}", encoded);

    let decoded : LogPacket = bincode::deserialize(&encoded).unwrap();
    println!("decode is: {:?}", decoded);

}