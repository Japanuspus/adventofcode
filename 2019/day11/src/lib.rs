extern crate num_bigint;
extern crate num_traits;

use std::collections::{BTreeMap};
use std::iter;
use num_bigint::{BigInt};
use num_traits::{Zero, One};
use num_traits::cast::ToPrimitive;
use std::convert::From;

fn digits_from_right(k: &BigInt) -> impl Iterator<Item=u8> {
    k.to_string().into_bytes().into_iter().rev().map(|c| c-b'0').chain(iter::repeat(0))
}

#[derive(Debug, Clone)]
pub struct State {
    tape: BTreeMap<BigInt, BigInt>,
    pc: BigInt,
    sp: BigInt //"relative base"
}

impl State {
    fn get_adress(&mut self, m: &mut impl iter::Iterator<Item=u8>) -> BigInt {
        // TODO: Avoid key clones by returning reference to static zero on lookup miss
        let g1 = self.pc.clone();
        match m.next() {
            Some(0) => { //normal
                self.tape.entry(g1).or_insert(Zero::zero()).clone()
            }
            Some(1) => { // immediate
                g1
            }
            Some(2) => { //relative
                self.tape.entry(g1).or_insert(Zero::zero()).clone() + &self.sp
            }
            _ => {panic!("Unknown mode");}
        }
    }

    fn get(&mut self, m: &mut impl iter::Iterator<Item=u8>) -> &BigInt {
        let g0 = self.get_adress(m);
        self.pc+=1;
        self.tape.entry(g0).or_insert(Zero::zero())
    }

    fn put(&mut self, v: BigInt, m: &mut impl iter::Iterator<Item=u8>) {
        let g0 = self.get_adress(m);
        self.pc+=1;
        self.tape.insert(g0, v);
    }

    pub fn next_output_callback<F>(&mut self, inputs: F) -> Result<Option<BigInt>, ()> 
    where
        F: FnMut() -> Option<BigInt>
    {
        next_output(self, inputs)
    }

    pub fn next_number_callback<F>(&mut self, mut inputs: F) -> Result<Option<isize>, ()> 
    where
        F: FnMut() -> Option<isize>
    {
        let r = next_output(self, || inputs().and_then(|v| Some(BigInt::from(v))));
        let w = r.or(Err(()))?;
        if let Some(v) = w {
            let vi = v.to_isize().ok_or(())?; 
            Ok(Some(vi))
        } else {
            Ok(None) //no output
        }
    }


    pub fn next_output(&mut self, inputs: &[isize]) -> Result<Option<BigInt>, ()> 
    {
        let mut ii = inputs
            .iter()
            .map(|v| BigInt::from(*v));
        next_output(self, || ii.next())
    }

    /// Get exactly n outputs as a vector
    /// 
    /// If machine halts before output, None is returned. Any other deviation results in Err.
    /// Outputs are cast back to isize, which may fail.
    pub fn next_numbers<F>(&mut self, n: usize, mut inputs: F) -> Result<Option<Vec<isize>>,()> 
    where
        F: FnMut() -> Option<isize>
    {        
        let mut res = Vec::new();
        let mut ii = || inputs().and_then(|x| Some(BigInt::from(x)));
        for _ in 0..n {
            if let Some(v) = next_output(self, &mut ii)? {
                if let Some(v_isize) = v.to_isize() {
                    res.push(v_isize)
                } else {
                    // output was too big for isize
                    return Err(())
                }
            } else {
                // halt before output
                break
            }
        };
        let n_out = res.len();
        if n_out==0 {Ok(None)} else {
            if n_out==n {Ok(Some(res))} 
            else {Err(())} // partial output
        }
    }

    pub fn poke(&mut self, addr: usize, value: isize) {
        self.tape.insert(BigInt::from(addr), BigInt::from(value)); 
    }

    pub fn reset(&mut self) {
        self.pc = Zero::zero();
        self.sp = Zero::zero();
    }
}

fn next_output<F>(s: &mut State, mut inputs: F) -> Result<Option<BigInt>,()> 
where
    F: FnMut() -> Option<BigInt>
{
    loop {
        let m = &mut digits_from_right(
            s.tape.get(&s.pc).unwrap_or(&Zero::zero())
        );
        let op=m.take(2).zip(&[1, 10]).map(|(v, m)| v*m).sum();
        s.pc += 1;
        match op {
            1 => { // add
                let v = s.get(m).clone() + s.get(m);
                s.put(v, m);
            }
            2 => { // mul
                let v = s.get(m).clone() * s.get(m);
                s.put(v, m);
            }
            3 => { // in
                if let Some(v) = inputs() {
                    s.put(v, m);
                } else {
                    return Err(())
                }
            }
            4 => { // out
                let a = s.get(m);
                return Ok(Some(a.clone()));
            }
            5 => { // jnz
                let cond = *s.get(m) != Zero::zero();
                let d = s.get(m);
                if cond { s.pc = d.clone();}
            }
            6 => { // jz
                let cond = *s.get(m) == Zero::zero();
                let d = s.get(m);
                if cond { s.pc = d.clone();}
            }
            7 => { // lt
                let v = s.get(m).clone() < *s.get(m);
                s.put(if v {One::one()} else {Zero::zero()}, m);
            }
            8 => { // eq
                let v = s.get(m).clone() == *s.get(m);
                s.put(if v {One::one()} else {Zero::zero()}, m);
            }
            9 => { // adjust relbase
                let v = s.get(m).clone();
                s.sp += v;
            }
            99 => { // halt
                break;
            }
            _ => {
                //dbg!(s);
                println!("Unknown operand");
                return Err(());
            }
        }
    };
    Ok(None)
}

impl<T> From::<T> for State 
where
    T: AsRef<str>
{
    fn from(stringlike: T) -> Self {
        let s: &str = stringlike.as_ref();
        let input: Vec<BigInt> = s
            .lines().next().unwrap()
            .split(',').map(|s| s.parse::<BigInt>().unwrap())
            .collect();    
        State{
            tape: input.iter().enumerate().map(|(i,v)| (BigInt::from(i), v.clone())).collect(),
            pc: Zero::zero(),
            sp: Zero::zero()
        }
    }
}

