use std::{thread, time};
use termion::{color,clear, cursor, raw::IntoRawMode, event::Key, input::TermRead};
use std::io::{Write, stdout, stdin};


pub struct LertVisualizer{
    pub ram_size: u32,
    pub disk_size: u32,
    pub depth: u32,
    pub expansion_factor: u32,
    pub time_stretch: u32,
    pub num_bins: u32,
    pub epoch_counter: u32,
}

impl LertVisualizer{
    pub fn new(ram_size: u32, disk_size: u32, expansion_factor: u32, time_stretch: u32) -> LertVisualizer{
        let depth = (disk_size as f64/(2.0*ram_size as f64).log(expansion_factor as f64)).ceil() as u32;
        let num_bins = (1.0/(time_stretch as f64)).ceil() as u32 +1;
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

    pub fn display_parameters(&self){
        println!("M = {} \nD = {} \ndepth = {} \nr = {} \nalpha = {} \nc = {} \n", self.ram_size, self.disk_size, self.depth, self.expansion_factor, self.time_stretch, self.num_bins);
    }

    fn display_controls(&self, line: u16){
        print!("{goto}Left/right arrow keys for previous/next epoch. q to quit.", goto = cursor::Goto(1,line));
    }

    //returns true if the bin flushes at this time step.
    pub fn bin_status(&self, level: u32, bin_id: u32, timestep: u32) -> bool {
        let mod_term = self.expansion_factor.pow(level) * self.num_bins;
        let is_it_my_turn: bool = timestep % mod_term == (self.expansion_factor.pow(level)*(self.num_bins) -1 + bin_id) % mod_term;
        let is_my_level_full: bool = timestep >= self.expansion_factor.pow(level)*self.num_bins -1;
        //println!("{}", is_it_my_turn);
        //println!("{}", is_my_level_full);
        is_it_my_turn && is_my_level_full
    }

    pub fn display_bins(&self, timestep: u32) {
        assert!(timestep >=0);
        self.wipeout();
        let mut next_line: u16 = 2;
        self.display_controls(next_line);
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
                else if i%2==0 {
                    let mut flushing: char = ' ';
                    
                    if self.bin_status(j, ((i-2)as f64/4.0 as f64).floor() as u32, timestep) {
                        flushing = 'x';
                    }
                    line_frame.push(flushing);
                }
                else {
                    line_frame.push(' ');
                }
            }
            print!("{goto}{}", line_frame, goto = cursor::Goto(1,next_line));
            next_line+=1;

        }
        
        print!("{goto}{}", horizontal_line, goto = cursor::Goto(1,next_line));
        next_line +=1;
        print!("{}", goto = cursor::Goto(1,next_line))
    }

    fn wipeout(&self) {
        println!("{clear}{goto}",
             // Full screen clear.
             clear = clear::All,
             // Go back to the top left.
             goto  = cursor::Goto(1, 1));
    }

    fn next(& mut self) {
        self.wipeout();
        self.epoch_counter += 1;
        self.display_bins(self.epoch_counter);
    }

    fn previous(& mut self) {
        self.wipeout();
        self.epoch_counter -= 1;
        self.display_bins(self.epoch_counter);
    }
    

    //animates the flushing schedule from epoch start_step for duration epochs. updates at a rate of refresh milliseconds.
    pub fn animate_bins_auto(&self, start_step: u32, duration: u32, refresh: u32){

        let refresh_rate = time::Duration::from_millis(refresh as u64);
        let end_step = start_step+duration;
        for i in start_step..=end_step{
            //self.wipeout();
            //println!("Epoch: {}", i);
            self.display_bins(i);
            thread::sleep(refresh_rate);

        }
    }

    pub fn animate_bins_manual(& mut self, start_step: u32) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.epoch_counter = start_step;

        //self.wipeout();
        //writeln!(stdout, "{}", format!("Epoch: {}", i)).unwrap();

        self.display_bins(self.epoch_counter);

        //writeln!(stdout, "Left/right arrow keys for previous/next epoch. q to quit.").unwrap();

        //self.display_controls();

        for c in stdin.keys() {
    
            // Print the key we type...
            match c.unwrap() {
                // Exit.
                Key::Char('q') => break,
                Key::Right => self.next(),
                Key::Left => self.previous(),
                _              => break,
            }
        }

    
    }
}






fn main() {
    let ram_size: u32 = 5;
    let disk_size: u32 = 20;
    let expansion_factor: u32 = 2;
    let time_stretch: u32 = 1;
    let mut l = LertVisualizer::new(ram_size, disk_size, expansion_factor, time_stretch);
    //l.display_parameters();

    l.animate_bins_manual(0);
    

}
