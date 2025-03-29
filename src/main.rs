mod markov;
use clap::Parser;
use markov::MarkovChainModel;

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "A simple random gibberish generator")]
struct Args {
    #[arg(default_value_t = 10)]
    min_len: usize,
}

fn main() {
    let args = Args::parse();
    let text = "was talking. Gis is a car iot imes that give him the rig in and the ic wat, Tei sad the mashe lithbe fore she lith Melved. Thente rri sa Terri loo det table. Thereen is hing outon. Drun keenager, plowis kup inhis campith thold coup in it. Tere upin their seventies. The eighteen taken the wheel they were alive, you wordsaid. lilles and nose and mouth. And shes lung uponit. Well was very depst. He draom hilas. to keep this short, So wook the wo of emup and worike most of the ni. They had these once in a while.";

    // TODO make state selectable?
    let m = MarkovChainModel::new(text, 2);

    let gener = m.generate(args.min_len);
    println!("{}", gener);
}
