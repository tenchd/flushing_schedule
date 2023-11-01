use std::{thread, time};
use termion::{color,clear, cursor, raw::IntoRawMode, event::Key, input::TermRead};
use std::io::{Write, stdout, stdin};


pub struct LertVisualizer{
    pub ram_size: u32,
    pub disk_size: u32,
    pub depth: u32,
    pub expansion_factor: u32,
    pub time_stretch: f64,
    pub num_bins: u32,
    pub epoch_counter: u32,
}

impl LertVisualizer{
    pub fn new(config: &(u32, u32, u32, f64)) -> LertVisualizer{
        let ram_size = config.0;
        let disk_size = config.1;
        let expansion_factor = config.2;
        let time_stretch = config.3;
        let depth = (disk_size as f64/(2.0*ram_size as f64).log(expansion_factor as f64)).ceil() as u32;
        let num_bins = (1.0/(time_stretch)).ceil() as u32 +1;
        LertVisualizer{
            ram_size: ram_size,
            disk_size: disk_size,
            depth: depth,
            expansion_factor: expansion_factor,
            time_stretch: time_stretch,
            num_bins: num_bins,
            epoch_counter: 0,
        }
    }

    pub fn display_parameters(&self,line:u16){
        let mut lines = Vec::new();
        for i in line..=line+5{
            lines.push(cursor::Goto(1,i));
        }
        print!("{}M = memory size = {} {}D = disk size = {} {}depth = {} {}r = expansion factor = {} {}alpha = timestretch = {} {}c = number of bins per level = {}",
        lines[0], self.ram_size, 
        lines[1], self.disk_size,
        lines[2], self.depth,
        lines[3], self.expansion_factor, 
        lines[4], self.time_stretch,
        lines[5], self.num_bins);
    }

    fn display_controls(&self, line: u16){
        print!("{goto}Left/right arrow keys for previous/next epoch. q to quit.", goto = cursor::Goto(1,line));
    }

    fn display_interpretation(&self, line: u16){
        print!("{goto}Blank space =  empty bin. X = nonempty bin. ! = flushing bin", goto = cursor::Goto(1,line));
    }

    fn compute_first_flush(&self, level: u32) -> u32 {
        let r = self.expansion_factor;
        let c = self.num_bins;
        let j = level;
        let mut first_flush = 0;
        match j {
            0 => first_flush = c - 1,
            //1 => first_flush = r*c + c - 2,
            _ => {
                first_flush = r.pow(j)*c - 1;
                for k in 0..=j-1{
                    first_flush += r.pow(k)*(c-1);
                }
            }
        }
        first_flush
    }

    //returns 2.0 if the bin flushes at this time step, else returns the bin's fractional fullness (between 0.0 and 1.0 inclusive.)
    pub fn bin_status(&self, level: u32, bin_id: u32, timestep: u32, partial: bool) -> f64 {
        let r = self.expansion_factor;
        let c = self.num_bins;
        let j = level;
        let i = bin_id;
        let t = timestep;

        let mod_term = r.pow(j) * c;
        //compute the first time any bin on this level flushes.
        let first_flush = self.compute_first_flush(j);
        let mut zeroth_flush = 0;
        if j>0 {
            zeroth_flush = self.compute_first_flush(j-1);
        }
        //println!("{}",first_flush);
        let touch_step = r.pow(j)*(c + i) % mod_term + zeroth_flush;
        let touch_mod = mod_term - (zeroth_flush % mod_term);
        let flush_step = (r.pow(j)*(c + i) + first_flush) % mod_term;
        let next_bin_flush_step = (r.pow(j)*(c + i + 1) + first_flush) % mod_term;
        let is_it_my_turn: bool = t % mod_term == flush_step;

        let shifted_flush_step = (flush_step + touch_mod) % mod_term;
        let shifted_next_bin_flush_step = (next_bin_flush_step + touch_mod) % mod_term;
        let shifted_t = (t + touch_mod) % mod_term;
        //println!("shifted t = {}", shifted_t);
        //println!("shifted flush step = {}", shifted_flush_step);
        //println!("shifted next bin flush step = {}", shifted_next_bin_flush_step);
        
        //let mut im_filling: bool = t % mod_term > flush_step && t % mod_term <= (next_bin_flush_step % mod_term) - 1;
        let mut im_filling: bool = shifted_t >= (shifted_flush_step + 1) % mod_term && shifted_t < (shifted_next_bin_flush_step) % mod_term;
        //println!("am i filling= {}", im_filling);

        //let mut im_filling: bool = (t + touch_mod) % mod_term >= (flush_step + touch_mod + 1) % mod_term && (t + touch_mod) % mod_term <= (next_bin_flush_step + mod_term + touch_mod - 1) % mod_term;
        if bin_id == self.num_bins - 1 {
            im_filling = shifted_t >= (shifted_flush_step + 1) % mod_term
            //im_filling = t % mod_term <= (next_bin_flush_step + mod_term - 1) % mod_term;
           
        }
        let is_my_bin_touched = t>= touch_step;
        //println!("is my bin touched = {}", is_my_bin_touched);
        let is_my_level_full: bool = t >= first_flush;
        //println!("t = {}, mod_term = {}, r^j*(c+i) = {}", t, mod_term, r.pow(j)*(c + i));
        
        // println!("is it my turn: {}", is_it_my_turn);
        // println!("is my level full: {}", is_my_level_full);
        // println!("mod_term = {}", mod_term);
        // println!("first_flush = {}", first_flush);
        // println!("flush_step = {}", flush_step);

        //println!("is my bin touched: {}, am I filling: {}", is_my_bin_touched, im_filling);
        if is_it_my_turn && is_my_level_full{
            2.0
        }
        else { 
            if is_my_bin_touched {
                if im_filling && j > 0 && partial{
                    //println!("wow");
                    let bin_size = r.pow(j);
                    //println!("{},{}", shifted_t, shifted_flush_step);
                    let steps_since_flush = (shifted_t + (bin_size - (shifted_flush_step% bin_size))) % bin_size;
                    //println!("{}", steps_since_flush);
                    //println!("{}", steps_since_flush as f64 /r.pow(j-1) as f64);
                    //let result = (((t-1)% bin_size) as f64 /r.pow(j-1) as f64).floor()/r as f64;
                    let result = (steps_since_flush as f64 /r.pow(j-1) as f64).floor()/r as f64;
                    //println!("{}", ((t-1)% bin_size) as f64 /r.pow(j-1) as f64);

                    result
                    //((t-1)% bin_size) as f64 / bin_size as f64
                }
                else{
                1.0
                }
            }
            else{
                0.0
            }     
        }   
    }

    pub fn display_bins(&self, timestep: u32, partial: bool) {
        //assert!(timestep >=0);
        self.wipeout();
        self.display_parameters(2);
        let mut next_line: u16 = 9;
        self.display_controls(next_line);
        next_line += 1;
        self.display_interpretation(next_line);
        next_line+=1;
        print!("{goto}Epoch: {t}", t = timestep, goto = cursor::Goto(1,next_line));
        next_line += 1;

        let line_length: u32 = 4*self.num_bins+1;
        //let num_lines: u32 = 2*self.depth+1;
        let mut horizontal_line = String::with_capacity(line_length as usize);
        for _ in 0..=line_length {
            horizontal_line.push('-');
        }

        
        for j in 0..=self.depth {
            println!("{goto}{h}", h = horizontal_line, goto = cursor::Goto(1,next_line));
            next_line += 1;

            let mut line_frame = String::with_capacity(line_length as usize);
            for i in 0..=line_length {
                if i%4==0 {
                    line_frame.push('|');
                }
                else {
                    line_frame.push(' ');
                }

            }
            //print!("{goto}{}", line_frame, goto = cursor::Goto(1,next_line));
            print!("{goto}{}", line_frame, goto = cursor::Goto(1,next_line));
            for i in 0..=self.num_bins -1 {
                let status = self.bin_status(j, i, timestep, partial);
                let write_position = 4*i + 3;
                if status == 2.0 {
                    print!("{goto}!", goto = cursor::Goto(write_position.try_into().unwrap(),next_line));
                }
                else if status != 0.0 {
                    let intensity = (status * 255.0) as u8;
                    let color = color::Fg(color::Rgb(intensity, intensity, intensity));
                    let reset = color::Fg(color::Reset);
                    print!("{goto}{c}X{r}", goto = cursor::Goto(write_position.try_into().unwrap(),next_line), c = color, r = reset);
                }
            }
            next_line+=1;

        }
        
        print!("{goto}{}", horizontal_line, goto = cursor::Goto(1,next_line));
        next_line +=1;
        print!("{goto}", goto = cursor::Goto(1,next_line))
    }

    fn wipeout(&self) {
        println!("{clear}{goto}",
             // Full screen clear.
             clear = clear::All,
             // Go back to the top left.
             goto  = cursor::Goto(1, 1));
    }

    fn next(& mut self, partial: bool) {
        self.wipeout();
        self.epoch_counter += 1;
        self.display_bins(self.epoch_counter, partial);
    }

    fn previous(& mut self, partial: bool) {
        if self.epoch_counter == 0 {
        }
        else {
            self.wipeout();
            self.epoch_counter -= 1;
            self.display_bins(self.epoch_counter, partial);
        }
    }
    

    //animates the flushing schedule from epoch start_step for duration epochs. updates at a rate of refresh milliseconds.
    pub fn animate_bins_auto(&self, start_step: u32, duration: u32, refresh: u32, partial: bool){

        let refresh_rate = time::Duration::from_millis(refresh as u64);
        let end_step = start_step+duration;
        for i in start_step..=end_step{
            //self.wipeout();
            //println!("Epoch: {}", i);
            self.display_bins(i, partial);
            thread::sleep(refresh_rate);

        }
    }

    pub fn animate_bins_manual(& mut self, start_step: u32, partial: bool) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.epoch_counter = start_step;

        //self.wipeout();
        //writeln!(stdout, "{}", format!("Epoch: {}", i)).unwrap();

        self.display_bins(self.epoch_counter, partial);

        //writeln!(stdout, "Left/right arrow keys for previous/next epoch. q to quit.").unwrap();

        //self.display_controls();

        for c in stdin.keys() {
    
            match c.unwrap() {
                // Exit.
                Key::Char('q') => break,
                Key::Right => self.next(partial),
                Key::Left => self.previous(partial),
                _              => break,
            }
        }

    
    }
}

fn set_parameters(config: &mut (u32, u32, u32, f64)) {
    println!("set ram size in GB. (integer values only)");
    let mut input_line = String::new();

    stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    let ram_size: u32 = input_line.trim().parse().expect("ram size must be an integer");

    println!("set disk size in GB. (integer values only)");

    let mut input_line2 = String::new();

    stdin()
        .read_line(&mut input_line2)
        .expect("Failed to read line");

    let disk_size: u32 = input_line2.trim().parse().expect("disk dize must be an integer");

    
    println!("set expansion factor. (integer values only)");

    let mut input_line3 = String::new();

    stdin()
        .read_line(&mut input_line3)
        .expect("Failed to read line");

    let expansion_factor: u32 = input_line3.trim().parse().expect("expansion factor must be an integer");


    println!("set timestretch. (float values only - e.g., 0.5)");
    
    let mut input_line4 = String::new();

    stdin()
        .read_line(&mut input_line4)
        .expect("Failed to read line");

        let timestretch: f64 = input_line4.trim().parse().expect("timestretch must be a float");

    config.0 = ram_size;
    config.1 = disk_size;
    config.2 = expansion_factor;
    config.3 = timestretch;
        
        
}


fn choose_mode(config: &mut (u32, u32, u32, f64)){
    println!("type 'p' to specify LERT parameters, or type anything else to use default values.");

    let mut choice = String::new();

    stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    //assert!(choice.trim().eq(&String::from("p")));

    let specify = choice.trim().eq(&String::from("p"));

    if specify{
        set_parameters(config);
    }
}



fn main() {
    let mut config = (5u32, 20u32, 3u32, 0.5);
    //let mut config = (2u32, 20u32, 2u32, 1.0);

    choose_mode(&mut config);
    
    let mut l = LertVisualizer::new(&config);
    //l.display_parameters();

    // for i in 0..=100{
    //     if l.bin_status(2,0,i) == 2.0 {
    //         println!("{}", i);
    //     }
    // }

    //println!("{}", l.bin_status(1,0,2));
    let partial = false;
    
    l.animate_bins_manual(0, partial);

    

}
