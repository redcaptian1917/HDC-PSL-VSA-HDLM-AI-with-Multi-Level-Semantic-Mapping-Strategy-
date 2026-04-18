use lfi_vsa_core::persistence::BrainDb;
use lfi_vsa_core::cognition::speech_act::SpeechActClassifier;
fn main() {
    let p = BrainDb::default_path();
    let db = BrainDb::open(&p).expect("open");
    println!("building classifier...");
    let t0 = std::time::Instant::now();
    let c = SpeechActClassifier::build_from_db(&db, 400);
    println!("built in {:.1}s ({} prototypes)", t0.elapsed().as_secs_f64(), c.prototype_count());
    for q in ["what is water","how do I install rust","explain quantum mechanics","why does the sky look blue","who invented electricity","list three types of trees","fix this bug in my code","summarize the plot of hamlet","compare python and rust","write a poem about fall","hello there","the answer is 42"] {
        let (a,s) = c.classify(q);
        println!("  {:<55} -> {:<13} {:.3}", q, a.as_label(), s);
    }
}
