mod packet;
use packet::Note;
std::io::{BufReader, BufRead};
use std::fs::{write, File, OpenOptions};
use bincode::{serialize_into, deserialize_from};

