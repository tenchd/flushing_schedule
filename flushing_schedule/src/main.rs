
pub struct LertVisualizer{
    pub ram_size: usize,
    pub disk_size: usize,
    pub depth: usize,
    pub expansion_factor: usize,
    pub time_stretch: usize,
    pub num_bins: usize,
    pub epoch_counter: usize,
}

impl LertVisualizer{
    pub fn new(ram_size: usize, disk_size: usize, expansion_factor: usize, time_stretch: usize) -> LertVisualizer{
        let depth = (disk_size as f64/(2.0*ram_size as f64).log(expansion_factor as f64)).ceil() as usize;
        let num_bins = (1.0/(time_stretch as f64)).ceil() as usize +1;
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

    pub fn bin_status(&self, level: u32, bin_id: usize, timestep: usize) -> bool {
        let is_it_my_turn: bool = timestep == (self.expansion_factor.pow(level)*(self.num_bins-1) + bin_id) % (self.expansion_factor.pow(level) * self.num_bins);
        let have_i_been_flushed_to: bool = timestep >= self.expansion_factor.pow(level)*self.num_bins;
        is_it_my_turn && have_i_been_flushed_to
    }

    pub fn display_bins(&self, timestep: usize) {
        let line_length: usize = 4*self.num_bins+1;
        let num_lines: usize = 2*self.depth+1;
        let mut horizontal_line = String::with_capacity(line_length);
        for _ in 0..=line_length {
            horizontal_line.push('-');
        }

        for j in 0..=self.depth {
            println!("{}", horizontal_line);

            let mut line_frame = String::with_capacity(line_length);
            for i in 0..=line_length {
                if i%4==0 {
                    line_frame.push(' ');
                }
                else if i%2==0 {
                    let mut full: char = 'x';
                    if self.bin_status(u32::from(j), ((i-2)as f64/4.0 as f64).floor() as usize, timestep) {
                        full = ' ';
                    }
                    line_frame.push(full);
                }
                else {
                    line_frame.push(' ');
                }
            }

        }
        
        println!("{}", horizontal_line);
       
    }
}


fn main() {
    let ram_size: usize = 5;
    let disk_size: usize = 20;
    let expansion_factor: usize = 2;
    let time_stretch: usize = 1;
    let l = LertVisualizer::new(ram_size, disk_size, expansion_factor, time_stretch);
    l.display_parameters();
    l.display_bins(0);
}
