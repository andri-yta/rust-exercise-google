use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

fn main() {
    let (tx, rx) = mpsc::sync_channel(10);

    let vec = Arc::new(vec![1, 2, 3]);

    let vec2 = vec.clone(); // read-only

    let t = thread::spawn(move || {
        let thread_id = thread::current().id();
        // Arc only provides shared ref -> cannot be mutable here
        // vec.push(100);
        return format!("{thread_id:?} {}", vec.len());
        //panic!("panic attack!");
    });

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 100..105 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });

    let res = t.join();
    match res {
        Ok(x) => println!("Ok {x:?}"),
        Err(e) => println!("Not OK {e:?}"),
    }

    // let res = t.join().unwrap();
    // println!("res: {res:?}");

    for msg in rx.iter() {
        println!("Main: got {}", msg);
    }

    // This will not work
    // vec2.push(123);
    // But this will work because reading only
    print_len(vec2);

    // MUTEX
    let v: Mutex<Vec<i32>> = Mutex::new(vec![10, 20, 30]);
    println!("v: {:?}", v.lock().unwrap());

    {
        let v: &Mutex<Vec<i32>> = &v;
        let mut guard = v.lock().unwrap();
        guard.push(40);
    }

    v.lock().unwrap().push(99);
    println!("v: {:?}", v.lock().unwrap());

    // NO MUTEX
    let mut v = vec![10, 20, 30];
    println!("No mutex v: {:?}", v);

    {
        v.push(40);
    }

    v.push(99);
    println!("No mutex v: {:?}", v);

    // MORE EXAMPLE: ARC + MUTEX
    let v = Arc::new(Mutex::new(vec!["v1".to_string()]));
    let v2 = Arc::clone(&v);
    let _ = thread::spawn(move || {
        let mut guard = v2.lock().unwrap();
        for i in 1..5 {
            guard.push(format!("Message {}", i.to_string()));
        }
    })
    .join();

    let mut guard = match v.lock() {
        Ok(guard) => guard,
        Err(e) => e.into_inner(),
    };

    guard.push("main thread".to_string());
    println!("guard v: {:?}", guard);

}

fn print_len(vec: Arc<Vec<i32>>) {
    println!("Main thread: {:?}", vec.len());
}
