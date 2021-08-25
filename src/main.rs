use rand::rngs::StdRng;
use rand_core::SeedableRng;
use std::str::FromStr;
use std::thread;
use structopt::StructOpt;
use wagyu_ethereum::{
    EthereumAddress, EthereumDerivationPath, EthereumFormat, EthereumMnemonic, EthereumNetwork,
    EthereumWordlist,
};
use wagyu_model::{
    ExtendedPrivateKey, ExtendedPublicKey, MnemonicCount, MnemonicExtended, PublicKey,
};

type N = wagyu_ethereum::Mainnet;
type W = wagyu_ethereum::English;

#[derive(StructOpt, Debug)]
#[structopt(name = "addr-searcher")]
struct Opt {
    #[structopt(short, long)]
    threads: usize,

    #[structopt(short, long)]
    zeroes: usize,
}

fn main() {
    let opt = Opt::from_args();
    let mut children = vec![];

    for _ in 0..opt.threads {
        let mut rng = StdRng::from_entropy();
        let prefix = format!("0x{}", (0..opt.zeroes).map(|_| "0").collect::<String>());
        let child = thread::spawn(move || loop {
            let (addr, mnemonic) = generate_address::<N, W>(&mut rng);
            if addr.to_string().starts_with(&prefix) {
                println!("{},\"{}\"", addr, mnemonic);
            }
        });
        children.push(child);
    }

    for child in children {
        let _ = child.join();
    }
}

fn generate_address<N: EthereumNetwork, W: EthereumWordlist>(
    rng: &mut StdRng,
) -> (EthereumAddress, EthereumMnemonic<N, W>) {
    let mnemonic = EthereumMnemonic::<N, W>::new_with_count(rng, 24).unwrap();
    let master_extended_private_key = mnemonic.to_extended_private_key(None).unwrap();
    let derivation_path = EthereumDerivationPath::from_str("m/44'/60'/0'/0/0").unwrap();
    let extended_private_key = master_extended_private_key
        .derive(&derivation_path)
        .unwrap();
    let extended_public_key = extended_private_key.to_extended_public_key();
    let public_key = extended_public_key.to_public_key();
    let address = public_key.to_address(&EthereumFormat::Standard).unwrap();

    (address, mnemonic)
}
