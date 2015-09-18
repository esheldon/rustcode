use std::thread;

// don't mangle the names, so the name can be seen "as-is" from C
// also declare it is public so it can be called from outside this
// module, and external so it can be called from C

#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn( || {
            let mut x=0;
            for _ in (0..5_000_000) {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
                 h.join().map_err(|_| "could not join thread!").unwrap() );
    }

    println!("done!");
}
