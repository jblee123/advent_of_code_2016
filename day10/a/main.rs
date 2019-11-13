use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fs;
use std::result::Result;
use std::str;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum ChipDest {
    Bot(u32),
    Output(u32),
}

#[derive(Clone, Debug)]
struct BotRules {
    lo_dest: ChipDest,
    hi_dest: ChipDest,
}

struct Bot {
    rules: BotRules,
    chips: HashSet<u32>,
}

impl Bot {
    fn new(rules: BotRules) -> Bot {
        Bot {
            rules: rules,
            chips: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct BotInput {
    bot_num: u32,
    chip: u32,
}

fn parse_bot_rules(s: &str) -> Result<(u32, BotRules), Box<dyn error::Error>> {
    let parts = s.split(" ").collect::<Vec<&str>>();

    let bot_num = u32::from_str(parts[1])?;

    let lo_dest_num = u32::from_str(parts[6])?;
    let lo_dest = match parts[5] {
        "bot" => ChipDest::Bot(lo_dest_num),
        "output" => ChipDest::Output(lo_dest_num),
        _ => panic!("badsadbad")
    };

    let hi_dest_num = u32::from_str(parts[11])?;
    let hi_dest = match parts[10] {
        "bot" => ChipDest::Bot(hi_dest_num),
        "output" => ChipDest::Output(hi_dest_num),
        _ => panic!("badsadbad")
    };

    Ok((bot_num, BotRules { lo_dest: lo_dest, hi_dest: hi_dest }))
}

fn generate_bots(s: &str) -> Result<HashMap<u32, Bot>, Box<dyn error::Error>> {
    let mut bots = HashMap::new();

    let bot_lines = s.lines()
        .filter(|line| line.starts_with("bot "));
    for line in bot_lines {
        let (bot_id, bot_rules) = parse_bot_rules(line)?;
        bots.insert(bot_id, Bot::new(bot_rules));
    }

    Ok(bots)
}

fn parse_bot_inputs(s: &str) -> Result<Vec<BotInput>, Box<dyn error::Error>> {
    let mut inputs = Vec::<BotInput>::new();
    let bot_lines = s.lines()
        .filter(|line| line.starts_with("value "));
    for line in bot_lines {
            let mut parts = line.split(" ");
        let chip = u32::from_str(parts.nth(1).unwrap())?;
        let bot_num = u32::from_str(parts.nth(3).unwrap())?;
        inputs.push(BotInput { bot_num: bot_num, chip: chip });
    }

    Ok(inputs)
}

fn run_bot_rule(bots: &mut HashMap<u32, Bot>, chip_dest: &ChipDest,
    chip_num: u32) {

    match chip_dest {
        ChipDest::Bot(targe_bot_num) => {
            {
                let target_bot = bots.get_mut(&targe_bot_num).unwrap();
                assert!(target_bot.chips.len() < 2,
                    "bot full of chips in run_bot_rule");
                target_bot.chips.insert(chip_num);
            }
            run_bot(bots, *targe_bot_num);
        },
        ChipDest::Output(_) => {},
    };
}

fn run_bot(bots: &mut HashMap<u32, Bot>, bot_num: u32) {
    let (lo_chip, hi_chip, rules) = {
        let bot = bots.get(&bot_num).unwrap();
        if bot.chips.len() < 2 {
            return;
        }

        let mut chip_iter = bot.chips.iter();
        let chip1 = *(chip_iter.next().unwrap());
        let chip2 = *(chip_iter.next().unwrap());
        (
            std::cmp::min(chip1, chip2),
            std::cmp::max(chip1, chip2),
            bot.rules.clone(),
        )
    };

    {
        bots.get_mut(&bot_num).unwrap().chips.clear();
    }

    if lo_chip == 17 && hi_chip == 61 {
        println!("robot {} got nums {} and {}", bot_num, lo_chip, hi_chip);
    }

    run_bot_rule(bots, &rules.lo_dest, lo_chip);
    run_bot_rule(bots, &rules.hi_dest, hi_chip);
}

fn run_bots(bots: &mut HashMap<u32, Bot>, inputs: &Vec<BotInput>)
    -> Result<(), Box<dyn error::Error>> {

    for input in inputs {
        {
            let bot = bots.get_mut(&input.bot_num).unwrap();
            assert!(bot.chips.len() < 2, "bot full of chips in run_bots");
            bot.chips.insert(input.chip);
        }
        run_bot(bots, input.bot_num);
    }

    Ok(())
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut bots = match generate_bots(&input) {
        Ok(bots) => bots,
        Err(error) => panic!("Problem generating bots: {:?}", error),
    };

    let inputs = match parse_bot_inputs(&input) {
        Ok(inputs) => inputs,
        Err(error) => panic!("Problem getting bot inputs: {:?}", error),
    };

    run_bots(&mut bots, &inputs);

    println!("bots len: {}", bots.len());
    println!("inputs len: {}", inputs.len());
}
