extern { fn yolo_detection(); }

fn main() {
    println!("Hello, world!");
    // unsafe {
    //     let lib = libloading::Library::new("./darknet/libdarknet.so")
    //         .expect("Failt to create a Library instance.");
    //     let func: libloading::Symbol<unsafe extern "C" fn() -> u32> = lib
    //         .get(b"run_detector")
    //         .expect("Fail to load a function from the darknet library.");
    //     let ret = func();
    // }
    
    let c = unsafe {
        yolo_detection();
    };

    println!("Finished.");
}
