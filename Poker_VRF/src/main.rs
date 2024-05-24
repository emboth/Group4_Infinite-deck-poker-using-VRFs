use rand::rngs::OsRng;
use rand::RngCore;
use schnorrkel::{
    context,
    vrf::{VRFInOut, VRFPreOut, VRFProof},
    Keypair,
};

struct Player {
    keypair: Keypair,
    vrf_inout: Option<VRFInOut>,
    vrf_proof: Option<VRFProof>,
}

impl Player {
    /// Creates a new player with a generated keypair.
    fn new() -> Self {
        let keypair = Keypair::generate_with(OsRng);
        Self {
            keypair,
            vrf_inout: None,
            vrf_proof: None,
        }
    }

    /// Generates a random 32-byte commitment.
    fn commit(&self) -> [u8; 32] {
        let mut csprng = OsRng;
        let mut commit = [0u8; 32];
        csprng.fill_bytes(&mut commit);
        commit
    }

    /// Reveals the VRF output and proof based on the input.
    fn reveal(&mut self, input: &[u8; 32]) {
        let context = context::SigningContext::new(b"example context");
        let (vrf_inout, proof, _batchable) = self.keypair.vrf_sign(context.bytes(input));
        self.vrf_inout = Some(vrf_inout);
        self.vrf_proof = Some(proof);
    }

    /// Retrieves the VRF output and proof if available.
    fn get_vrf_output(&self) -> Option<(&VRFInOut, &VRFProof)> {
        match (&self.vrf_inout, &self.vrf_proof) {
            (Some(vrf_inout), Some(vrf_proof)) => Some((vrf_inout, vrf_proof)),
            _ => None,
        }
    }
}

fn main() {
    let mut players: Vec<Player> = vec![Player::new(), Player::new(), Player::new()];
    let mut commits: Vec<[u8; 32]> = Vec::new();

    // Step 1: Commit
    for player in &players {
        let commit = player.commit();
        commits.push(commit);
    }

    // Step 2: Reveal and generate VRF outputs
    for (i, player) in players.iter_mut().enumerate() {
        player.reveal(&commits[i]);
    }

    // Step 3: Determine the winner
    let mut highest_vrf_output: Option<VRFPreOut> = None;
    let mut winner: Option<usize> = None;

    for (i, player) in players.iter().enumerate() {
        if let Some((vrf_inout, _vrf_proof)) = player.get_vrf_output() {
            let vrf_preout = vrf_inout.to_preout();
            match &highest_vrf_output {
                Some(highest) => {
                    if vrf_preout.to_bytes() > highest.to_bytes() {
                        highest_vrf_output = Some(vrf_preout);
                        winner = Some(i);
                    }
                }
                None => {
                    highest_vrf_output = Some(vrf_preout);
                    winner = Some(i);
                }
            }
        }
    }

    match winner {
        Some(index) => println!("Player {} wins!", index + 1),
        None => println!("No winner."),
    }
}
