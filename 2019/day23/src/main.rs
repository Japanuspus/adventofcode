#![allow(unused)]

use std::collections::BTreeSet;
// use std::collections::HashMap;
use std::collections::VecDeque;
// use std::iter;
use day11::State; // dep: day11 = {path="../day11"}
use std::thread;
use std::time;
use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::sync::mpsc::channel;

type Packet = (isize, isize);

struct PacketReceiver {
    id: isize,
    rx: mpsc::Receiver<Packet>,
    pending: Option<isize>,
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

fn part1(input: &str) {
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

// part2
enum NatMessage {
    Packet(Packet),
    Blocking(isize),
    Unblocking(isize),
}

struct BlockingPacketReceiver {
    id: isize,
    rx: mpsc::Receiver<Packet>,
    mx: mpsc::Sender<NatMessage>,
    pending: Option<isize>,
    empty_poll_count: usize,
}

impl BlockingPacketReceiver {
    fn get(&mut self) -> Option<isize> {
        if let Some(v) = self.pending {
            self.pending = None;
            self.reset_empty_poll_count();
            return Some(v)
        }
        match self.rx.try_recv() {
            Ok((x, y)) => {
                self.pending = Some(y);
                self.reset_empty_poll_count();
                Some(x)
            }
            Err(mpsc::TryRecvError::Empty) => {
                self.empty_poll_count +=1;
                None
            }
            _ => panic!("Read on closed pipe")
        }
    }

    fn reset_empty_poll_count(&mut self) {
        self.empty_poll_count = 0;
    }

    // Will block and wait for input if empty_poll_count is larger than 3
    fn get_blocking(&mut self) -> Option<isize> {
        if self.empty_poll_count > 3 {
            self.mx.send(NatMessage::Blocking(self.id));
            let (x,y) = self.rx.recv().unwrap();
            self.mx.send(NatMessage::Unblocking(self.id));
            self.pending = Some(y);
            self.reset_empty_poll_count();
            Some(x)    
        } else {
            self.get()
        }
    }
}


fn part2(input: &str) {

    let (msg_tx, msg_rx) = channel();

    let n_machine: isize = 50;
    let mut txs_orig: Vec<_> = Vec::new();
    let mut rxs: Vec<_> = Vec::new();
    for i in 0..n_machine {
        let (tx, rx) = channel();
        txs_orig.push(tx);
        let prx = BlockingPacketReceiver{id: i, rx, pending: Some(i), empty_poll_count: 0, mx: msg_tx.clone()};
        rxs.push(prx);
    }
    let mut threads = Vec::new();
    for (i, mut rx) in rxs.into_iter().enumerate() {
        let mut s = State::from(&input);
        let txs = txs_orig.clone();
        let mx = msg_tx.clone();
        let t = thread::spawn(move || {
            loop {
                if let Some(packet) = s.next_numbers(3, || rx.get_blocking().or(Some(-1))).expect("engine failure") {
                    rx.reset_empty_poll_count();
                    let addr = packet[0];
                    let x = packet[1];
                    let y = packet[2];
                    println!("Packet {} --> {}: ({}, {})", i, addr, x, y);
                    if addr == 255 {
                        mx.send(NatMessage::Packet((x,y)));
                    } else {
                        txs[addr as usize].send((x,y));
                    }
                } else {
                    println!("no packet for node {}", i);
                }
            }            
        });
        threads.push(t);
    }

    let mut blocked = BTreeSet::new();
    let mut last_nat_packet = (0,0);
    let mut last_sent_y: Option<isize> = None;
    let mx = &msg_rx;
    let naptime = time::Duration::from_millis(1000);
    loop {
        match mx.try_recv() {
            Ok(NatMessage::Blocking(id)) => {
                blocked.insert(id);
                //println!("{:2} Blocking, count is {}", id, blocked.len());
            }
            Ok(NatMessage::Unblocking(id)) => {
                blocked.insert(id);
                //println!("{:2} Unblocking, count is {}", id, blocked.len());
            }
            Ok(NatMessage::Packet(p)) => {
                println!("Received nat packet: {:?}", &p);
                last_nat_packet = p.clone();
            }
            Err(mpsc::TryRecvError::Empty) => {
                if blocked.len()==n_machine as usize {
                    println!("*** Blocking, count is {}",  blocked.len());
                    if let Some(ly) = last_sent_y {
                        if ly == last_nat_packet.1 {
                            println!("Part 2 -- repeating y value in dispatched nat packet: {}", ly);
                            break
                        }
                    }
                    println!("Sending packet {:?} to channel 0", &last_nat_packet);
                    last_sent_y = Some(last_nat_packet.1);
                    &txs_orig[0].send(last_nat_packet.clone());
                    // TODO: Flaky wait is required here
                    thread::sleep(naptime);
                }
            }
            _ => panic!("Read on closed pipe")
        }
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    
    part1(&input);
    part2(&input);
}