use std::path::Path;

use mielikki::filecache;
#[ignore = "Used to test file cache features and timing functions"]
#[tokio::test]
async fn test_caching() {
    let fc = filecache::FileCache::new(Path::new("test-cache").to_owned()).unwrap();
    fc.update_filecache_file_from_memory().unwrap();
    assert!(fc.check_file_parses());
    fc.update_memory_cache();
    let t = tokio::time::Instant::now();
    let r = fc.find_file("main.rs").unwrap();
    for res in r {
        println!("{:?}", res)
    }
    println!("{}", t.elapsed().as_millis());
    panic!()
}
