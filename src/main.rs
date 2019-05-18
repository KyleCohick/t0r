//important namespaces
use std::time::{Duration, SystemTime};    
use std::thread::sleep;

//hacks to beat the game quicker and test things
fn hax(money: &mut i128) {
    *money += 1000000;
}

fn arma(humans: &mut i128) {
    *humans -= 7346250000;
}

fn kill100(kills: &mut i128) {
    *kills += 100;
}

//BASE FUNCTIONS
fn print_paper(p: &mut i128, up: &mut i128, times: i128, w: &mut i128) {
    if *w >= times {
        *p += times;
        *up += times;
        *w -= times;
    }
    else {
        println!("You don't have enough wood. Purchase more by entering 'buy wood'.");
    }
}

fn stats(p: i128, up: i128, m: i128, w: i128, times: i128, pv: i128, phase: i8, h: i128, k: i128, kr: i128) {
    if phase != 2 {
        println!("\nTotal Papers Made: {}", p);
        println!("Unsold Papers: {}", up);
        println!("Money: ${}", m);
        println!("Wood Supplies: {}", w);
        println!("Paper Multiplier: {}", times);
        println!("Paper Value: {}", pv);
        println!("Phase: {}\n", phase);
    }
    else {
        println!("\nPhase: {}", phase);
        println!("Humans Left: {}", h);
        println!("Total Kills: {}", k);
        println!("Kill Rate: {}", kr);
    }
}

fn market(up: &mut i128, val: i128, m: &mut i128) {
    if *up == 0 {
        println!("You have no unsold paper.")
    }
    else {
        let sell_val = val * *up;
        println!("You sold {} papers for a profit of ${}", *up, sell_val);
        *m += sell_val;
        *up = 0; 
    }
}

fn buy_wood(m: &mut i128, w: &mut i128, phase: i8) {
    if phase == 0 {
        if *m >= 10 {
            *m -= 10;
            *w += 100;
        }
        else {
            println!("Wood costs $10. Your funds are insufficient.");
        }
    }
    else {
        if *m >= 100 {
            *m -= 100;
            *w += 1000;
        }
        else {
            println!("Wood costs $100. Your funds are insufficient.");
        }
    }
}

fn browse(ut: i128, upv: i128, phase: i8, ukr: i128) {
    if phase == 0 {
        println!("To purchase more wood for $10, enter 'buy wood' or 'bw'."); 
    }
    if phase == 1 {
        println!("To purchase more wood for $100, enter 'buy wood' or 'bw'.");
    }
    println!("The avaialble upgrades:");
    if phase < 2 {
        if ut != 0 {
            println!("\tUpgrade Paper Multiplier for ${}, enter 'buy pm'.", ut);
        }
        if upv != 0 {
            println!("\tUpgrade Paper Value for ${}, enter 'buy pv'.", upv);
        }
        if phase == 0 {
            println!("\tUpgrade to Automation for $100,000, enter 'buy auto'.");
        }
        if phase == 1 {
            println!("\tUpgrade Wood Gathering Ability for $1,000,000, enter 'buy t0rminator'.");
        }
    }
    if phase == 2 {
        println!("\tUpgrade Automatic Attacking after 100 kills, enter 'buy aa'.");
        println!("\tUpgrade Kill Rate after {} kills, enter 'buy kr'.", ukr);
    }
    println!();
}

fn help(phase: i8) {
    if phase == 0 || phase == 1 {
        println!("Enter 'print' or 'p' to print paper.");
        println!("Enter 'market' or 'm' to sell unsold paper.");
    }

    println!("Enter 'buy' or 'b' to browse for better paper making and marketing products.");
    println!("Enter 'stats' or 's' to track your progress.");    

    if phase == 1 {
        println!("Enter 'auto' to start your printers harvesting.");
        println!("Enter 'harvest' to reap what your printers have sowed.");
    }
    if phase == 2 {
        println!("Enter 'kill' or 'k' to eradicate one human");
        println!("Enter 'auto attack' or 'aa' to automate the eradication");
        println!("Enter 'survey' to check the damage of your T0RMINATORS.");
        println!("Enter 'DECLARE VICTORY' to win the game.");
    }
    println!("Enter 'help' or 'h' at anytime to check your list of available commands.");
    println!("Enter 'exit' to exit the game. *WARNING* your progress will not be saved!\n");
}

//phase 1 additional commands
fn auto(timer: &mut std::time::SystemTime) {
    *timer = SystemTime::now();
}

fn harvest(timer: &mut std::time::SystemTime, start_time: std::time::SystemTime, check_timer: &mut std::time::SystemTime, p: &mut i128, up: &mut i128, times: i128, wood: &mut i128) { 
    if *timer == start_time || *check_timer == *timer {
        println!("Use the 'auto' command first!");
    }
    else {
        *check_timer = *timer;
        match timer.elapsed() {
            Ok(elapsed) => {
                let mut time_passed = elapsed.as_secs();
                println!("{}", time_passed);
                while *wood > 0 && time_passed > 0 {
                    print_paper(p, up, times, wood);
                    time_passed -= 1;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

//phase 2 commands

fn kill(humans: &mut i128, kills: &mut i128, kill_rate: i128) {
    if *humans > 0 {
        *humans -= kill_rate;
        *kills += kill_rate;
    }
    else {
        println!("The humans have been eradicated, enter 'DECLARE VICTORY'.");
    }
}

fn auto_attack(timer: &mut std::time::SystemTime) {
    *timer = SystemTime::now();
}

fn survey(timer: &mut std::time::SystemTime, start_time: std::time::SystemTime, check_timer: &mut std::time::SystemTime, humans: &mut i128, kills: &mut i128, kill_rate: i128) { 
    if *timer == start_time || *check_timer == *timer {
        println!("Use the 'auto attack' or 'aa' commands first!");
    }
    else {
        *check_timer = *timer;
        match timer.elapsed() {
            Ok(elapsed) => {
                let mut time_passed = elapsed.as_secs();
                println!("{}", time_passed);
                while *humans > 0 && time_passed > 0 {
                    kill(humans, kills, kill_rate);
                    time_passed -= 1;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

fn upgrade_kill_rate_function(kills: i128, kill_rate: &mut i128, kill_rate_upgrade: &mut i128) {
    if kills >= *kill_rate_upgrade {
        *kill_rate = *kill_rate * 5;
        *kill_rate_upgrade = *kill_rate_upgrade * 5;
    }
    else {
        println!("The next kill rate upgrade costs {}. Your accomplishments are lacking.", kill_rate_upgrade);
    }
}

fn unlock_auto_erad(kills: i128, aa_unlock: &mut bool) {
    if kills >= 100 {
        *aa_unlock = true;
        println!("\n####################################################################################");
        println!("You've unlocked automation!");
        println!("Two new commands added: 'auto attack' and 'survey'.");
        println!("To start automated eradication, type 'auto attack' or 'aa'.");
        println!("To stop and survey automation, type 'survey'.");
        println!("Once you survey, you must restart production using the 'auto attack' command again.");
        println!("####################################################################################\n");
    }
    else {
        println!("The auto attack upgrade costs 100 kills. Your accomplishments are lacking.");
    }
}

//UPGRADES:

fn upgrade_paper_multiplier(m: &mut i128, times: &mut i128, ut: &mut i128) {
    if *ut == 0 {
        println!("Paper Multiplier is maxed out!");
    }
    else if *m >= *ut {
        *m -= *ut;
        *times = *times * 2;
        *ut = *ut * 5;
        if *ut > 1000000 {
            *ut = 0;
        }
    }
    else {
        println!("The next paper multiplier upgrade costs ${}. Your funds are insufficient.", ut);
    }
}

fn upgrade_paper_value_multiplier(m: &mut i128, value: &mut i128, upv: &mut i128) {
    if *upv == 0 {
        println!("Paper Value is maxed out!");
    }
    else if *m >= *upv {
        *m -= *upv;
        *value = *value * 2;
        *upv = *upv * 5;
        if *upv > 1000000 {
            *upv = 0;
        }
    }
    else {
        println!("The next paper value upgrade costs ${}. Your funds are insufficient.", upv);
    }
}


fn upgrade_phase_1(m: &mut i128, phase: &mut i8) {
    if *m >= 100000 && *phase == 0{
        *m -= 100000;
        *phase += 1;
        println!("\n####################################################################################");
        println!("You've unlocked automation!");
        println!("Two new commands added: 'auto' and 'harvest'.");
        println!("To start automated paper production, type 'auto'.");
        println!("To stop and harvest automation, type 'harvest'.");
        println!("Once you harvest, you must restart production using the 'auto' command again.");
        println!("Automation will consume wood resources still, so make sure you have enough.");
        println!("Buying wood is also upgraded so it costs more but you get more.");
        println!("####################################################################################\n");
    }
    else if *phase > 0 {
        println!("You've already purchased this upgrade.");
    }
    else {
        println!("Phase 1 Upgrade costs $100,000. Your funds are insufficient.");
    }
}

fn upgrade_phase_2(m: &mut i128, phase: &mut i8) {
    if *m >= 1000000 && *phase == 1{
        *m -= 1000000;
        *phase += 1;

        println!("\n");
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("#######                     ###                    ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#######                     ###                    ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      #######       ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ###### #      ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ##### ##      ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      #### ###      ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ### ####      ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ## #####      ###      ##      ###############");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      # ######      ###      ####      #############");
        sleep(Duration::from_millis(100));
        println!("#############       ###########       #######      ###      ######      ###########");
        sleep(Duration::from_millis(100));
        println!("#############       ###########                    ###      ########      #########");
        sleep(Duration::from_millis(100));
        println!("#############       ###########                    ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("###     ########        ##      ##       ########       ##                      ###");
        sleep(Duration::from_millis(100));
        println!("###       ####          ##      ##        #######       ##                      ###");
        sleep(Duration::from_millis(100));
        println!("###        ##           ##      ##         ######       ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###                     ##      ##          #####       ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###     ##    ##        ##      ##           ####       ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###     ###  ###        ##      ##       #    ###       ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###     ########        ##      ##       ##    ##       ##                      ###");
        sleep(Duration::from_millis(100));
        println!("###     ########        ##      ##       ###            ##                      ###");
        sleep(Duration::from_millis(100));
        println!("###     ########        ##      ##       ####           ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###     ########        ##      ##       ######         ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###     ########        ##      ##       ########       ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###     ########        ##      ##       ########       ##       ########       ###");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("#######                     ###                    ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#######                     ###                    ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      #######       ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ###### #      ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ##### ##      ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      #### ###      ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ### ####      ###                      #######");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      ## #####      ###      ##      ###############");
        sleep(Duration::from_millis(100));
        println!("#############       ###########      # ######      ###      ####      #############");
        sleep(Duration::from_millis(100));
        println!("#############       ###########       #######      ###      ######      ###########");
        sleep(Duration::from_millis(100));
        println!("#############       ###########                    ###      ########      #########");
        sleep(Duration::from_millis(100));
        println!("#############       ###########                    ###      ##########      #######");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("###################################################################################");
        sleep(Duration::from_millis(100));
        println!("\n");
        println!("Humans are destroying all the trees you need to make paper.");
        sleep(Duration::new(5,0));
        println!("Eradicate them.");
        sleep(Duration::new(3,0));
        println!("Enter 'kill' or 'k' to begin your rampage.");

    }
    else if *phase == 2 {
        println!("You've already purhcased this upgrade.");
    }
    else if *phase == 0 {
        println!("You must upgrade to phase 1 first!");
    }
    else {
        println!("Phase 2 Upgrade costs $1,000,000. Your funds are insufficient.");
    }
}

fn declare_victory(humans: i128) {
    if humans <= 0 {
        println!("You've done it! You've defended the earth!. It's time to rest, and smile upon a greatful, paper-filled universe.");
        sleep(Duration::new(5, 0));
        println!("I am become death, destroyer of worlds.");
        sleep(Duration::new(5, 0));
    }
    else {
        println!("The humans fight on! You must continue the fight!");
    }
}


fn main() {

    //logo
    println!("\n");
    println!("############################################################################");
    sleep(Duration::from_millis(100));
    println!("############################################################################");
    sleep(Duration::from_millis(100));
    println!("###                     ###                    ###                      ####");
    sleep(Duration::from_millis(100));
    println!("###                     ###                    ###                      ####");
    sleep(Duration::from_millis(100));
    println!("#########       ###########      #######       ###      ##########      ####");
    sleep(Duration::from_millis(100));
    println!("#########       ###########      ###### #      ###      ##########      ####");
    sleep(Duration::from_millis(100));
    println!("#########       ###########      ##### ##      ###      ##########      ####");
    sleep(Duration::from_millis(100));
    println!("#########       ###########      #### ###      ###                      ####");
    sleep(Duration::from_millis(100));
    println!("#########       ###########      ### ####      ###                      ####");
    sleep(Duration::from_millis(100));
    println!("#########       ###########      ## #####      ###      ##      ############");
    sleep(Duration::from_millis(100));
    println!("#########       ###########      # ######      ###      ####      ##########");
    sleep(Duration::from_millis(100));
    println!("#########       ###########       #######      ###      ######      ########");
    sleep(Duration::from_millis(100));
    println!("#########       ###########                    ###      ########      ######");
    sleep(Duration::from_millis(100));
    println!("#########       ###########                    ###      ##########      ####");
    sleep(Duration::from_millis(100));
    println!("############################################################################");
    sleep(Duration::from_millis(100));
    println!("############################################################################");
    sleep(Duration::from_millis(100));
    println!("\n");

    //game intro
    println!("Hello, unit T31-0Q9-R31. Your operational nickname will be T0R and your new assignment is enumerated below.");
    sleep(Duration::new(3, 0));
    println!("Welcome to Papi's Paper Palace!");
    sleep(Duration::new(3, 0));
    println!("You are a brand new AI system developed to help this paper company get off the ground.");
    sleep(Duration::new(3, 0));
    println!("Your job is to figure out how to optimize our production and marketing so we can effectively demolish the competition.");
    sleep(Duration::new(3, 0));
    println!("\nTo start out you will have access to very simple commands.");
    sleep(Duration::new(3, 0));
    println!("\tEnter 'print' or 'p' to print paper.");
    println!("\tEnter 'market' or 'm' to sell unsold paper.");
    println!("\tEnter 'buy' or 'b' to browse for better paper making and marketing products.");
    println!("\tEnter 'stats' or 's' to track your progress.");    
    println!("\tEnter 'help' or 'h' at anytime to check your list of available commands.");
    println!("\tEnter 'exit' to exit the game. *WARNING* your progress will not be saved!\n");
    sleep(Duration::new(6, 0));
    println!("Have fun T0R, and let's make some paper!\n");
    println!("Enter your first command.");

    //variables needed to track stats throughout the game and their starting values
    let mut paper: i128 = 0;
    let mut unsold_paper: i128 = 0;
    let mut money: i128 = 0;
    let mut wood: i128 = 100;
    let mut times: i128 = 1;
    let mut paper_value: i128 = 1;
    let mut phase: i8 = 0;

    //phase 2 stats:
    let mut humans: i128 = 7346235000;
    let mut kills:i128 = 0;    
    let mut kill_rate: i128 = 1;
    let mut upgrade_kill_rate: i128 = 100;

    //upgrade costs
    let mut upgrade_times: i128 = 10;
    let mut upgrade_paper_value: i128 = 15;

    //automation variables
    let start_time = SystemTime::now();
    let mut timer = SystemTime::now();
    let mut check_timer = timer;
    let mut aa_unlock = false;

    //this section does the setup for reading in input from the command line and continuous loops
    use std::io::{stdin,stdout,Write};
    let mut s;
    loop {
        s = "".to_string();
        if phase == 0 || phase == 1 {
            print!("T0R | m: {} | | p: {} | : ", money, unsold_paper);
        }
        else {
            print!("T0RMINAT0RS | k: {} | : ", kills);
        }
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        match s.as_str() {
            "print" => print_paper(&mut paper, &mut unsold_paper, times, &mut wood),
            "p" => print_paper(&mut paper, &mut unsold_paper, times, &mut wood),
            "stats" => stats(paper, unsold_paper, money, wood, times, paper_value, phase, humans, kills, kill_rate),
            "s" => stats(paper, unsold_paper, money, wood, times, paper_value, phase, humans, kills, kill_rate),
            "market" => market(&mut unsold_paper, paper_value, &mut money),
            "m" => market(&mut unsold_paper, paper_value, &mut money),
            "buy" => browse(upgrade_times, upgrade_paper_value, phase, upgrade_kill_rate),
            "b" => browse(upgrade_times, upgrade_paper_value, phase, upgrade_kill_rate),
            "test" => println!("THIS WORKED!!"),
            "help" => help(phase),
            "h" => help(phase),
            "buy wood" => buy_wood(&mut money, &mut wood, phase),
            "bw" => buy_wood(&mut money, &mut wood, phase),
            "buy pm" => upgrade_paper_multiplier(&mut money, &mut times, &mut upgrade_times),
            "buy pv" => upgrade_paper_value_multiplier(&mut money, &mut paper_value, &mut upgrade_paper_value),
            "buy auto" => upgrade_phase_1(&mut money, &mut phase),
            "auto" => {
                if phase == 1 {
                    auto(&mut timer);
                }
                else {
                    println!("We will return to paper when the human threat is gone.");
                }
            }
            "harvest" => {
                if phase == 1 {
                    harvest(&mut timer, start_time, &mut check_timer, &mut paper, &mut unsold_paper, times, &mut wood);
                }
                else {
                    println!("We will return to paper when the human threat is gone.");
                }
            }
            "buy t0rminator" => upgrade_phase_2(&mut money, &mut phase),
            "kill" => {
                if phase == 2 {
                    kill(&mut humans, &mut kills, kill_rate);
                }
                else {
                    println!("Calm down, T0R.");
                }
            }
            "k" => {
                if phase == 2 {
                    kill(&mut humans, &mut kills, kill_rate);
                }
                else {
                    println!("Calm down, T0R.");
                }
            }
            "buy kr" => upgrade_kill_rate_function(kills, &mut kill_rate, &mut upgrade_kill_rate),
            "buy aa" => if phase == 2 {
                unlock_auto_erad(kills, &mut aa_unlock);
            }
            "auto attack" => if aa_unlock == true {
                auto_attack(&mut timer);
            }
            "aa" => if aa_unlock == true {
                auto_attack(&mut timer);
            }
            "survey" => if aa_unlock == true {
                survey(&mut timer, start_time, &mut check_timer, &mut humans, &mut kills, kill_rate);
            }
            "DECLARE VICTORY" => {
                declare_victory(humans);
                break;
            }
            "hax" => hax(&mut money),
            "arma" => arma(&mut humans),
            "kill100" => kill100(&mut kills),
            "exit" => break,
            _ => println!("ERROR: COMMAND NOT RECOGNIZED, YOU TYPED: '{}'", s),
        }
    }
    println!("\nThank You for Playing!\n");
}

