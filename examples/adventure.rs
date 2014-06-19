//! An adventure story modeled as actions on bits.
//!
//! Each state in the story is a sequence of bits, each representing something that happened.
//! There are two numbers stored for each state, one for the state and one for the options.
//! This type of modeling can be used to experiment with a story in an explorative way.
//!
//! By applying filtering to the data, one could build stories as abstractions
//! and reuse them in contexts where one needs control of the events.

type Data = Vec<(u64, u64)>;
type Actions = Vec<&'static str>;

fn is_set(state: u64, i: u64) -> bool {
    ((state >> i) & 0b1) == 0b1
}

fn set(state: u64, i: u64) -> u64 {
    state ^ (0b1 << i)
}

/// Computes the changed bit and whether it is inverse or not.
///
/// Returns index and whether it is inverted.
fn change(state: u64, prev_state: u64) -> uint {
    let diff = state ^ prev_state;
    for i in range(0u64, 64) {
        if is_set(diff, i) {
            if ((prev_state >> i) & 0b1) == 0b1 {
                fail!("Can not invert action.");
            } else {
                return i as uint;
            }
        }
    }

    fail!("State differs more or less than one bit");
}

fn with_options(state: u64, data: &Data, f: |u64|) {
    for &(id, options) in data.iter() {
        if id != state { continue; }
        for i in range(0u64, 64) {
            if is_set(options, i) {
                f(set(state, i))
            }
        }
    }
}

fn main() {
    let actions: Actions = vec!(
        "I used to live each summer in a house by the ocean.",
        "One day I went on a fishing trip but came out for a storm.",
        "The storm crushed my boat against a lonely island.",
        "Luckily a big boat picked me up before I sank.",

        // _0000
        "The boat was owned by am old fisherman.",
        "I search the whole island for water, but could not find any.",
        "Finally I settle down for the night in front of a fire.",
        "Suddenly I heard a roar in the dark.",

        // _0000_0000
        "A huge troll charged against me with a liften axe.",
        "Needless to say, the troll was soon dead.",
        "I climbed up in a tree faster than a squirrel in flight.",
        "Unfortunately the tree was no match for the troll's axe.",

        // _0000_0000_0000
        "I thanked him for his bravery.",
        "'You're welcome' he said and smiled with pointed teeth.",
        "That sight made me jump overboard and sim as fast as I could away from the boat.",
        "Fortunately the closest island was not far away.",
    
        // _0000_0000_0000_0000
        "Before I had time to reflect uppon the situation, I heard a squeeky sound.",
        "I looked down and saw a small troll grabbing my boots.",
        "A rope had tangled my feet and dragged me underwater.",
        "For a moment I could hold my breath, but then everything went black.",

        // _0000_0000_0000_0000_0000
        "I woke up in a strange place.",
        "There were people around me, but they were not quite human.",
        "Instead of arms and legs, they had fins.",
        "They told me every human that drown ends up here.",

        // _0000_0000_0000_0000_0000_0000
        "The tree hit the ground slowly enough for me to run.",
        "In the dark I did not saw where I was headed.",
        "My body fell down a cliff and everything went black.",
        "Instead of arms and legs, they had wings.",

        // _0000_0000_0000_0000_0000_0000_0000
        "They told me every human that falls down a cliff ends up here.",
        "Next day I looked in every direction, but there was no signs of land.",
        "I built a boat of the troll's skeleton.",
    );
	let data: Data = vec!(
        (0b0, 0b1),
        (0b1, 0b10),
        (0b11, 0b100_0000_0000_0000_1100),
        (0b_1011, 0b1_0000),
        (0b111, 0b10_0000),
        (0b10_0111, 0b100_0000),
        (0b110_0111, 0b1000_0000),
        (0b1110_0111, 0b1_0000_0000),
        (0b1_1110_0111, 0b110_0000_0000),
        (0b101_1110_0111, 0b1000_0000_0000),
        (0b11011, 0b1_0000_0000_0000),
        (0b1_0000_0001_1011, 0b10_0000_0000_0000),
        (0b11_0000_0001_1011, 0b100_0000_0000_0000),
        (0b111_0000_0001_1011, 0b1000_0000_0000_0000),
        (0b1111_0000_0001_1011, 0b1_0000_0000_0000_0000),
        (0b1_1111_0000_0001_1011, 0b10_0000_0000_0000_0000),
        (0b11_1111_0000_0001_1011, 0b1000_0000),
        (0b11_1111_0000_1001_1011, 0b1_0000_0000),
        (0b100_0000_0000_0000_0011, 0b1000_0000_0000_0000_0000),
        (0b1100_0000_0000_0000_0011, 0b1_0000_0000_0000_0000_0000),
        (0b1_1100_0000_0000_0000_0011, 0b10_0000_0000_0000_0000_0000),
        (0b11_1100_0000_0000_0000_0011, 0b100_0000_0000_0000_0000_0000),
        (0b111_1100_0000_0000_0000_0011, 0b1000_0000_0000_0000_0000_0000),
        (0b1101_1110_0111, 0b1_0000_0000_0000_0000_0000_0000),
        (0b1_0000_0000_0000_1101_1110_0111, 0b10_0000_0000_0000_0000_0000_0000),
        (0b11_0000_0000_0000_1101_1110_0111, 0b100_0000_0000_0000_0000_0000_0000),
        (0b111_0000_0000_0000_1101_1110_0111, 0b1_0000_0000_0000_0000_0000),
        (0b111_0001_0000_0000_1101_1110_0111, 0b10_0000_0000_0000_0000_0000),
        (0b111_0011_0000_0000_1101_1110_0111, 0b1000_0000_0000_0000_0000_0000_0000),
        (0b1111_0011_0000_0000_1101_1110_0111, 0b1_0000_0000_0000_0000_0000_0000_0000),
        (0b11_1110_0111, 0b10_0000_0000_0000_0000_0000_0000_0000),
        (0b10_0000_0000_0000_0000_0011_1110_0111, 0b100_0000_0000_0000_0000_0000_0000_0000),
    );
    let states: Vec<u64> = vec!(
        0b0,
        0b1,
        0b11,

            0b111,
            0b10_0111,
            0b110_0111,
            0b1110_0111,
            0b1_1110_0111,

                0b11_1110_0111,
                0b10_0000_0000_0000_0000_0011_1110_0111, // building a boat of the troll's skeleton

                // 0b101_1110_0111,
                // 0b1101_1110_0111,
                // 0b1_0000_0000_0000_1101_1110_0111,
                // 0b11_0000_0000_0000_1101_1110_0111,
                // 0b111_0000_0000_0000_1101_1110_0111,
                // 0b111_0001_0000_0000_1101_1110_0111,
                // 0b111_0011_0000_0000_1101_1110_0111, // waking up with bird people
                // 0b1111_0011_0000_0000_1101_1110_0111,

            // 0b_1011,
            // 0b1_1011, // the old fisherman picked me up.
            // 0b1_0000_0001_1011,
            // 0b11_0000_0001_1011,
            // 0b111_0000_0001_1011,
            // 0b1111_0000_0001_1011, // squeeky sound
            // 0b1_1111_0000_0001_1011,
            // 0b11_1111_0000_0001_1011,
            // 0b11_1111_0000_1001_1011,

            // 0b100_0000_0000_0000_0011, // dragged down under water
            // 0b1100_0000_0000_0000_0011,
            // 0b1_1100_0000_0000_0000_0011,
            // 0b11_1100_0000_0000_0000_0011,
            // 0b111_1100_0000_0000_0000_0011,
    );
    let mut i = 0;
    for state in states.as_slice().windows(2) {
        let ch = change(state[1], state[0]);
        println!("{}", actions.get(ch));
        if i == states.len() - 2 {
            with_options(state[1], &data, |opt: u64| {
                let opt_ch = change(opt, state[1]);
                println!("0b{:t}: {}", opt, actions.get(opt_ch));
            });
        }
        i += 1;
    }
    println!("0b{:t}", *states.get(states.len() - 1));
}

