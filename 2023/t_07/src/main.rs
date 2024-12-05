use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use tracing::Level;

#[derive(Clone, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Into<usize> for HandType {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Debug)]
struct Hand {
    cards: String,
    first_card: u8,
    bid: u32,
}

fn main() {
    let inp = parse("./input.txt").unwrap();
    println!("{:?}", inp);
}

fn parse(path: &str) -> anyhow::Result<u32> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    tracing::info!("starting parser");
    let mut file = File::open(path)?;
    let mut raw = String::new();

    file.read_to_string(&mut raw)?;
    std::mem::drop(file);

    let raw: Vec<_> = raw.lines().collect();

    // matrix of Hand mapped by position
    let mut hands: Vec<Vec<Hand>> = vec![Vec::new(); 7];

    use HandType::*;

    // parse into scores vec
    tracing::debug!("making hands matrix");
    for line in raw.iter() {
        if let Some((cards, bid)) = line.split_once(' ') {
            let bid = bid.parse::<u32>()?;

            let counts = cards.chars().counts();
            let values = counts.values().sorted().rev().join("");
            let hand_type = match values.as_ref() {
                "5" => FiveOfAKind,
                "41" => FourOfAKind,
                "32" => FullHouse,
                "311" => ThreeOfAKind,
                "221" => TwoPair,
                "2111" => OnePair,
                "11111" => HighCard,
                _ => unreachable!(),
            };
            hands[hand_type.clone() as usize].push(Hand {
                bid,
                first_card: to_u8(cards.chars().next()),
                cards: cards.to_string(),
            });
        }
    }
    tracing::debug!("sort hands matrix");
    let mut sum: Vec<u32> = Vec::with_capacity(7);
    let mut count: u32 = 0;
    for row in hands[5..6].iter_mut() {
        //tracing::debug!("sorting row based on cards in each hand");
        for hand in row.iter() {
            dbg!(&hand.cards);
        }
        println!("---------------");
        hand_row_sort(row);
        for hand in row.iter() {
            dbg!(&hand.cards);
        }
        println!("---------------");

        sum.push(
            row.iter()
                .map(|hand| {
                    count += 1;
                    count * hand.bid
                })
                .sum(),
        );
    }

    Ok(sum.into_iter().sum())
}

// could not find how to sort in place
fn hand_row_sort(row: &mut Vec<Hand>) {
    let ln = row.len();
    if ln < 2 {
        return;
    }
    let mut prev = row[0].cards.clone();
    let mut swapped = true;

    while swapped == true {
        swapped = false;

        // iterate thought hands in row
        'row: for i in 1..ln {
            let cur = row[i].cards.clone();
            // iterate throught both hands
            for j in 0..5 {
                let pc = to_u8(prev.chars().nth(j));
                let cc = to_u8(cur.chars().nth(j));
                println!("{} {} {} {}", pc, prev, cc, cur);
                if pc == cc {
                    continue;
                }
                if pc > cc {
                    prev = cur;
                    row.swap(i, i - 1);
                    //stt = i;
                    swapped = true;
                    break 'row;
                } else {
                    break;
                }
            }
        }
    }
}

fn to_u8(c: Option<char>) -> u8 {
    let c = c.unwrap();
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => c.to_digit(10).unwrap() as u8,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exp() -> anyhow::Result<()> {
        use super::parse;
        use std::fs::canonicalize;
        use std::path::PathBuf;

        // get abs path to src dir
        let src = canonicalize(&PathBuf::from("./src/test.txt"))?;

        let inp = parse(src.to_str().unwrap())?;

        assert_eq!(inp, 6440);

        Ok(())
    }
}
