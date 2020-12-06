use aoclib;
use std::collections::BTreeSet;

fn main() -> aoclib::Result<()> {
    let psets: Vec<Vec<BTreeSet<char>>> = aoclib::get_inputs_pwd()?
        .split("\n\n")
        .map(|grp| {
            grp.lines()
                .map(|ln| ln.chars().collect::<BTreeSet<_>>())
                .collect()
        })
        .collect();
    println!(
        "Part 1: {}",
        psets
            .iter()
            .map(|g| g
                .iter()
                .fold(BTreeSet::new(), |a, e| a.union(e).cloned().collect())
                .len())
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        psets
            .iter()
            .map(|g| g
                .iter()
                .fold(None, |ao: Option<BTreeSet<_>>, e| ao
                    .map_or(Some(e.clone()), |a| Some(
                        a.intersection(e).cloned().collect()
                    )))
                .map_or(0, |gs| gs.len()))
            .sum::<usize>()
    );
    Ok(())
}
