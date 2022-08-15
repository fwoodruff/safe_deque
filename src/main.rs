mod deque;
use deque::Queue;

use std::thread;
use std::sync::{Mutex, Arc, Condvar};

const LOOPCOUNT : i32 = 5000;

fn cond_wait((lock, cvar) : &(Mutex<bool>, Condvar)) {
    {
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }
    cvar.notify_one();
}

fn main() {
    let mutcond = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = mutcond.clone();
    let pair2 = mutcond.clone();
    let pair3 = mutcond.clone();
    let pair4 = mutcond.clone();

    let deque_handle : Arc<Mutex<Queue<i32>>> = Arc::new(Mutex::new(Queue::new()));
    let clone_handle1 = deque_handle.clone();
    let clone_handle2 = deque_handle.clone();
    let clone_handle3 = deque_handle.clone();
    let clone_handle4 = deque_handle.clone();
    
    let thread_handle1 = thread::spawn(move || {
        cond_wait(&pair1);
        let mut sum : i32 = 0;
        for _ in 0..LOOPCOUNT {
            let mut dq = clone_handle1.lock().unwrap();
            let val = dq.pop_head();
            match val {
                Some(v) => { sum += v; }
                None => { }
            }   
        }
        sum
    });

    
    let thread_handle2 = thread::spawn(move || {
        cond_wait(&pair2);
        let mut sum : i32 = 0;
        for _ in 0..LOOPCOUNT {
            let mut dq = clone_handle2.lock().unwrap();
            let val = dq.pop_tail();
            match val {
                Some(v) => { sum += v; }
                None => { }
            }
        }
        sum
    });

    let thread_handle3 = thread::spawn(move || {
        cond_wait(&pair3);
        for _ in 0..LOOPCOUNT {
            let mut dq = clone_handle3.lock().unwrap();
            dq.push_tail(1);
        }
    });
    
    let thread_handle4 = thread::spawn(move || {
        cond_wait(&pair4);
        for _ in 0..LOOPCOUNT {
            let mut dq = clone_handle4.lock().unwrap();
            dq.push_head(1);
        }
        
    });

    let (lock, cvar) = &*mutcond;
    {
        let mut started = lock.lock().unwrap();
        *started = true;
    }
    cvar.notify_one();

    let res1 = thread_handle1.join();
    let res2 = thread_handle2.join();
    let _res3 = thread_handle3.join();
    let _res4 = thread_handle4.join();
    
    let mut dq = deque_handle.lock().unwrap();
    let mut end_sum : i32 = 0;
    

    loop {
        let val = dq.pop_head();
        match val {
            Some(v) => {
                end_sum += v;
            }
            None => {
                break;
            }
        }
    }


    println!("{}", res1.as_ref().unwrap());
    println!("{}", res2.as_ref().unwrap());
    println!("{}", end_sum);
    println!("{}", (res1.unwrap()+res2.unwrap()+end_sum));
    
}
