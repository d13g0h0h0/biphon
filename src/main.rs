use rand_distr::{Normal, Distribution};
use rand::{seq::SliceRandom, random};

const MAX_CANDIDATES: usize = 2;

fn main() {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 2.0).unwrap();
    let mut list_Con =
        [Constraint{name: String::from("NoCoda"), ranking: 5.0, disharmony: 0.0},
        Constraint{name: String::from("Faith"), ranking: 7.0, disharmony: 0.0}];
    let mut listCandidates = 
        ([Candidate{name: String::from("/.pak./"), listViolations: [1, 0]}, Candidate{name: String::from("/.pa./"), listViolations: [0, 1]}],
        [Candidate{name: String::from("/.ban./"), listViolations: [1, 0]}, Candidate{name: String::from("/.ba./"), listViolations: [0, 1]}],
        [Candidate{name: String::from("/.kal./"), listViolations: [1, 0]}, Candidate{name: String::from("/.ka./"), listViolations: [0, 1]}]);
    let mut listInputs = 
        [Input::newInput("pak", listCandidates.0),
        Input::newInput("ban", listCandidates.1),
        Input::newInput("kal", listCandidates.2)];
    let input_Randomizer = rand::distributions::Uniform::from(1..listInputs.len());
    
    let up: i32 = 100;
    for _i in 0..up{
        for n1 in 0..list_Con.len(){
            let noise: f64 = normal.sample(&mut rand::thread_rng());
            list_Con[n1].disharmony = list_Con[n1].ranking + noise;
        }
        let n2 = input_Randomizer.sample(&mut rng);
        println!("{}", listInputs[n2].name);
    }
}

struct Constraint {
    name: String,
    ranking: f64,
    disharmony: f64,
}

struct Input{
    name: String,
    candidates: [Candidate; MAX_CANDIDATES],
}

impl Input {
    fn newInput(newName: &str, list: [Candidate; MAX_CANDIDATES]) -> Input{
        Input{
            name: String::from(newName),
            candidates: list,
        }
    }
}

struct Candidate{
    name: String,
    listViolations: [i32; 2],
}
