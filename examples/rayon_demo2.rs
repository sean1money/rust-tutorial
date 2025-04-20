fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(10)
        .build()
        .unwrap();
    pool.spawn(|| {
        println!("Hello, world!");
    });

    let r = pool.join(|| 1, || "2");
    eprintln!("{:?}", r);
}
