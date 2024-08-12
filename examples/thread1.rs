
use std::{sync::mpsc::{self, Sender}, thread};

use anyhow::anyhow;
use::anyhow::Result;
const NUM_PRODUCERS: usize =4;

#[derive(Debug)]
#[allow(dead_code)]
struct Msg{
    idx: usize,
    val: usize,
}

fn main() ->Result<()>{
    
    let (tx,rx)=mpsc::channel();
    for i in 0..NUM_PRODUCERS{
        let tx = tx.clone();
        thread::spawn(move ||producer(i,tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx{
            println!("{:?}",msg);
        }
    });
    consumer.join().map_err(|e|anyhow!("Thread failed to join{:?}",e))?;
    Ok(())
}


fn producer(idx: usize, tx:Sender<Msg>)->Result<()>{
    loop {
        let n = rand::random::<usize>(); 
        tx.send(Msg::new(idx, n))?;
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}

impl Msg {
    fn new(idx: usize, val: usize) -> Msg{
        Msg { idx,val}
    }
}