use std::{
    error::Error,
    fs::File
};
use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Category {Open, Female}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Swimmer {
    name:       String,
    category:   Category,
    fly:        Option<f32>,
    back:       Option<f32>,
    brs:        Option<f32>,
    free:       Option<f32>
}

const MAX: f32 = 999.0;
impl PartialEq for Swimmer {
    fn eq(&self, other: &Self) -> bool {
        self.free.unwrap_or(MAX) == other.free.unwrap_or(MAX)
    }
}

impl PartialOrd for Swimmer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.free.unwrap_or(MAX).partial_cmp(&other.free.unwrap_or(MAX))
    }
}

// allow pretty-printing
impl std::fmt::Display for Swimmer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
            "{}, {:?}\n50 fly: {}  |  50 back: {}  |  50 brs: {}  |  50 fs: {}",
                self.name, self.category, 
                self.fly.unwrap_or(0.0), 
                self.back.unwrap_or(0.0), 
                self.brs.unwrap_or(0.0), 
                self.free.unwrap_or(0.0))
    }
}

// Vec<T> doesn't implement Display, and we can't edit a type that doesn't exist here
// so we use newtypes to create a wrapper that we can edit
// also i feel like a Box or something could have been smarter but anyway
struct SwimmerVectorWrapper<'a, T: 'a>(Vec<&'a T>);
impl<'a, T: std::fmt::Display + 'a> std::fmt::Display for SwimmerVectorWrapper<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for swimmer in &self.0 {
            if let Err(e) = writeln!(f, "{}", swimmer) {
                panic!("something went seriously wrong: {}", e);
            }
        }
        Ok(())
    }
}

fn medley() {
    todo!()
}

// god this is some ugly code
// i can definitely do better with Box<Swimmer>
fn mixed_medley(team: &[Swimmer]) {
    // split by category
    let female_swimmers = team.iter().filter(|v| v.category == Category::Female);
    let open_swimmers = team.iter().filter(|v| v.category == Category::Open);

    // get all pairs
    let female_combinations: Vec<Vec<&Swimmer>> = female_swimmers.into_iter().combinations(2).collect();
    let open_combinations: Vec<Vec<&Swimmer>> = open_swimmers.into_iter().combinations(2).collect();
    

    let mut fastest: SwimmerVectorWrapper<Swimmer> = SwimmerVectorWrapper(Vec::new());
    let mut fastest_time: f32 = 9999.0;
    let mut curr_time: f32;
    let mut curr_four: Vec<&Swimmer>;

    // exhaustively generate every combination of pairs in every possible permutation
    for f_curr in female_combinations.iter() {
        for o_curr in open_combinations.iter() {
            // extract each vector and combine them
            curr_four = [f_curr.as_slice(), o_curr.as_slice()].concat();

            for p in curr_four.iter().permutations(4) {
                curr_time = 0.0;
                
                curr_time += p.first().unwrap().back.unwrap_or(MAX);
                curr_time += p.get(1).unwrap().brs.unwrap_or(MAX);
                curr_time += p.get(2).unwrap().fly.unwrap_or(MAX);
                curr_time += p.get(3).unwrap().free.unwrap_or(MAX);

                if curr_time < fastest_time {
                    fastest_time = curr_time;
                    fastest = SwimmerVectorWrapper(curr_four.clone());
                }
            }
        }
    }

    let predicted_mins : u8 = (fastest_time.floor() as u8) / 60;
    let predicted_secs : u8 = (fastest_time.floor() as u8) - (predicted_mins * 60);
    let predicted_ms = format!("{:0.2}", fastest_time - fastest_time.floor());

    println!("fastest possible time: {}:{}{:.2}\n{}", predicted_mins, predicted_secs, predicted_ms.trim_start_matches('0'), fastest);
}

fn free(team: Vec<Swimmer>) {
    // use the same code for each gender, just pass it in
    // also this one is easy, just sort

    let mut sorted = team.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // this is such a hack lmao
    // i really should rewrite this to use slices
    let (fastest, _) = sorted.split_at(4);
    let fastest_time : f32 = fastest.iter().map(|s| s.free.unwrap()).sum();
    
    let predicted_mins : u8 = (fastest_time.floor() as u8) / 60;
    let predicted_secs : u8 = (fastest_time.floor() as u8) - (predicted_mins * 60);
    let predicted_ms = format!("{:0.2}", fastest_time - fastest_time.floor());

    let print_fastest = SwimmerVectorWrapper(fastest.iter().collect_vec());
    println!("fastest possible time: {}:{}{:.2}\n{}", predicted_mins, predicted_secs, predicted_ms.trim_start_matches('0'), print_fastest);
}

// take a length parameter to determine how many swimmers from each category
// allows use for traditional mixed free and canon
// TODO: generalise this for all relays - split it down to mixed vs single category
// also this is very stupid, i was using it as a test for the mixed medley
fn mixed_free(team: &[Swimmer], members: usize) {
    // filter by category
    let female_swimmers = team.iter().filter(|v| v.category == Category::Female);
    let open_swimmers = team.iter().filter(|v| v.category == Category::Open);
    
    // generate all combinations of size (members)
    let female_combinations: Vec<Vec<&Swimmer>> = female_swimmers.into_iter().combinations(members).collect();
    let open_combinations: Vec<Vec<&Swimmer>> = open_swimmers.into_iter().combinations(members).collect();

    let mut fastest: SwimmerVectorWrapper<Swimmer> = SwimmerVectorWrapper(Vec::new());
    let mut fastest_time: f32 = 9999.0;
    let mut curr_time: f32;
    
    // iterate through and find the fastest relay
    for f_curr in female_combinations.iter() {
        for o_curr in open_combinations.iter() {
            curr_time = 0.0;

            for swimmer in o_curr.iter() {
                curr_time += swimmer.free.unwrap_or(MAX);
            }
            for swimmer in f_curr.iter() {
                curr_time += swimmer.free.unwrap_or(MAX);
            }
            
            // compare to current fastest time
            if curr_time < fastest_time {
                fastest_time = curr_time;
                fastest = SwimmerVectorWrapper([f_curr.as_slice(), o_curr.as_slice()].concat());
            }
        }
    }

    let predicted_mins : u8 = (fastest_time.floor() as u8) / 60;
    let predicted_secs : u8 = (fastest_time.floor() as u8) - (predicted_mins * 60);
    let predicted_ms = format!("{:0.2}", fastest_time - fastest_time.floor());

    println!("fastest possible time: {}:{}{:.2}\n{}", predicted_mins, predicted_secs, predicted_ms.trim_start_matches('0'), fastest);
    // write functions for pretty-printing the relays
}

fn read_csv(team: &mut Vec<Swimmer>) -> Result<(), Box<dyn Error>> {
    let file_path = "data/time_trials.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;

        let name = record[0].to_string();
        let category = match &record[1] {
            "O" => Category::Open,
            "F" => Category::Female,
            _ => panic!("unrecognised category")
        };
        
        // not everyone has a time for every 50
        let fly:    Option<f32> = record[2].parse().ok();
        let back:   Option<f32> = record[3].parse().ok();
        let brs:    Option<f32> = record[4].parse().ok();
        let free:   Option<f32> = record[5].parse().ok();

        let swimmer = Swimmer {name, category, fly, back, brs, free};
        team.push(swimmer);
    }
    Ok(())
}

fn main() {
    // all swimmers
    let mut team = Vec::new();
    let _ = read_csv(&mut team);

    // i shouldn't need these calls to clone, my method signatures are bad
    println!("women's free relay:");
    free(team.clone().into_iter().filter(|v| v.category == Category::Female).collect_vec());

    println!("open free relay:");
    free(team.clone().into_iter().filter(|v| v.category == Category::Open).collect_vec());

    println!("mixed free relay:");
    mixed_free(&team, 2);

    //println!("women's medley relay:");
    //medley(team.iter().filter(|v| v.category == Category::Female).collect_vec());
    //
    //println!("open medley relay:");
    //medley(team.clone().into_iter().filter(|v| v.category == Category::Open).collect_vec());

    println!("mixed medley relay:");
    mixed_medley(&team);

    println!("canon:");
    mixed_free(&team, 3);
}

    //let paddy = Swimmer {name: "paddy", category: Category::Open, fly: Some(25.0), back: Some(32.8), brs: Some(36.5), fs: Some(25.3)};
    //// TODO: throw these in as Options when i'm turning them into tests
    //let jake = Swimmer {name: "jake", category: Category::Open, fly: 28.0, back: 32.2, brs: 29.3, fs: 25.5};
    //let eidhne = Swimmer {name: "eidhne", category: Category::Open, fly: 26.0, back: 32.7, brs: 29.7, fs: 23.7};
    //let fergal = Swimmer {name: "fergal", category: Category::Open, fly: 29.0, back: 34.8, brs: 33.5, fs: 25.7};
    //let stuart = Swimmer {name: "stuart", category: Category::Open, fly: 30.0, back: 32.8, brs: 31.5, fs: 28.7};
    //let robin = Swimmer {name: "robin", category: Category::Open, fly: 25.5, back: 29.8, brs: 33.5, fs: 23.9};
    //let leah = Swimmer {name: "leah", category: Category::Female, fly: 25.5, back: 29.8, brs: 33.5, fs: 24.9};
    //let caoilfhinn = Swimmer {name: "caoilfhinn", category: Category::Female, fly: 25.5, back: 29.8, brs: 33.5, fs: 23.9};
    //let naoise = Swimmer {name: "naoise", category: Category::Female, fly: 25.5, back: 29.8, brs: 33.5, fs: 25.9};
    //
    //team.insert(String::from("paddy"), paddy);
    //team.insert(String::from("jake"), jake);
    //team.insert(String::from("eidhne"), eidhne);
    //team.insert(String::from("fergal"), fergal);
    //team.insert(String::from("stuart"), stuart);
    //team.insert(String::from("robin"), robin);
    //team.insert(String::from("leah"), leah);
    //team.insert(String::from("caoilfhinn"), caoilfhinn);
    //team.insert(String::from("naoise"), naoise);