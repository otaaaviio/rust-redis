macro_rules! init_snapshotting {
    ($storage:expr, $tokio:expr) => {
        let storage_clone = Arc::clone(&$storage);
        tokio::spawn(async move {
            let mut i = 0;
            let mut time_to_sleep= 60;
            loop {
                tokio::time::sleep(Duration::from_secs(time_to_sleep)).await;
                let mut storage = storage_clone.lock().await;
                if storage.should_take_snapshot() {
                    println!("Executing snapshot: {}", i);
                    i += 1;

                    if let Err(e) = storage.save_rdb_file() {
                        println!("Error executing snapshot: {}", e);
                    }
                } else {
                    time_to_sleep = storage.snapshot.snapshot_period_secs as u64;
                }
            }
        });
    };
}

macro_rules! load_rdb_file {
    ($storage:expr) => {
        {
            let mut storage = $storage.lock().await;
            println!("Init loading RDB File...");
            match storage.load_rdb_file() {
                Ok(()) => println!("RDB File loaded successfully"),
                Err(e) => println!("Error loading RDB File: {}", e)
            }
        }
    };
}