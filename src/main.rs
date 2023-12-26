use std::thread;
use std::time::Duration;
use tikv_jemalloc_ctl::{stats, epoch};

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    // Obtain a MIB for the `epoch`, `stats.allocated`, and
    // `atats.resident` keys:
    let e = epoch::mib().unwrap();
    let allocated = stats::allocated::mib().unwrap();
    let resident = stats::resident::mib().unwrap();
    
    loop {
        // Many statistics are cached and only updated 
        // when the epoch is advanced:
        e.advance().unwrap();
        
        // Read statistics using MIB key:
        let allocated = allocated.read().unwrap();
        let resident = resident.read().unwrap();
        println!("{} bytes allocated/{} bytes resident", allocated, resident);
        thread::sleep(Duration::from_secs(10));
    }
}
