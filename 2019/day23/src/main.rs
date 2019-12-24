#![allow(unused)]

// use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::VecDeque;
// use std::iter;
use day11::State; // dep: day11 = {path="../day11"}
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::channel;

type Packet = (isize, isize);

struct PacketReceiver {
    id: isize,
    rx: mpsc::Receiver<Packet>,
    pending: Option<isize>
}

impl PacketReceiver {
    fn get(&mut self) -> Option<isize> {
        if let Some(v) = self.pending {
            self.pending = None;
            return Some(v)
        }
        match self.rx.try_recv() {
            Ok((x, y)) => {
                self.pending = Some(y);
                Some(x)
            }
            Err(mpsc::TryRecvError::Empty) => {
                None
            }
            _ => panic!("Read on closed pipe")
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    
    let n_machine = 50;
    let mut txs_orig: Vec<_> = Vec::new();
    let mut rxs: Vec<_> = Vec::new();
    for i in 0..n_machine {
        let (tx, rx) = channel();
        txs_orig.push(tx);
        let prx = PacketReceiver{id: i, rx, pending: Some(i)};
        rxs.push(prx);
    }
    let mut threads = Vec::new();
    for (i, mut rx) in rxs.into_iter().enumerate() {
        let mut s = State::from(&input);
        let txs = txs_orig.clone();
        let t = thread::spawn(move || {
            loop {
                if let Some(packet) = s.next_numbers(3, || rx.get().or(Some(-1))).expect("engine failure") {
                    let addr = packet[0];
                    let x = packet[1];
                    let y = packet[2];
                    println!("Packet {} --> {}: ({}, {})", i, addr, x, y);
                    if addr == 255 {panic!()}
                    txs[addr as usize].send((x,y));
                } else {
                    println!("no packet for node {}", i);
                }
            }            
        });
        threads.push(t);
    }
    dbg!(threads.pop().unwrap().join());
}