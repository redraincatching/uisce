use uisce::*;

use itertools::Itertools;

fn main() {
    // all swimmers
    let mut team : Vec<Swimmer> = Vec::new();
    let _ = read_csv(&mut team);

    // i shouldn't need these calls to clone, my method signatures are bad
    println!("women's free relay:");
    free(team.clone().into_iter().filter(|v| v.category == Category::Female).collect_vec());

    println!("open free relay:");
    free(team.clone().into_iter().filter(|v| v.category == Category::Open).collect_vec());

    println!("mixed free relay:");
    mixed_free(&team, 2);

    println!("women's medley relay:");
    medley(team.clone().into_iter().filter(|v| v.category == Category::Female).collect_vec());

    println!("open medley relay:");
    medley(team.clone().into_iter().filter(|v| v.category == Category::Open).collect_vec());

    println!("mixed medley relay:");
    mixed_medley(&team);

    println!("canon:");
    mixed_free(&team, 3);
}
