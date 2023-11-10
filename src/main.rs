use clap::Parser;
use std::fmt;
fn main() {

    let args = Cli::parse();

    let result = bit_reverse(args.input, args.width, args.lsb);

    let format = args.format;

    if format == "Hex" {
        println!("Result: {}", Format::Hex(result));
    } else if format == "Dec" {
        println!("Result: {}", Format::Dec(result));
    } else if format == "Bin" {
        let w = args.bwidth;
        println!("Result: {}", Format::Bin(result, w));
    }

    

    
}

enum Format {
    Hex(u128),
    Bin(u128, usize),
    Dec(u128),
}


impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::Hex(value) => write!(f, "{:#X}", value),
            Format::Dec(value) => write!(f, "{}", value),
            Format::Bin(value, width) => write!(f, "{:0width$b}", value),
        }
     }
}

#[derive(Parser)]
#[clap(author="Charles Giese", version, about="High Resolution Spectrometer Client  ")]
pub struct Cli {
    /// Input Value
    #[clap(short)]
    input: u128,
    /// Width of bits to reverse
    #[clap(default_value_t = 8, short, long)]
    width: u128,
    /// LSB to reverse
    #[clap(default_value_t = 0, short, long)]
    lsb: u128,
    /// Output Format, Hex, Dec or Bin.
    #[clap(short, long)]
    format: String,
    /// Binary representation Width
    #[clap(default_value_t = 32, short, long)]
    bwidth: usize,

}

fn get_bit_at(input: u128, n: u128) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn bit_reverse(num : u128, width : u128, lsb : u128) -> u128 {

    let mut val:u128 = 0;
    let mut rev:u128 = 0;
    let mut tmp:u128;

    let selection = (num >> lsb) & ((1 << (lsb + width - lsb)) - 1);
    
    while val < width {
        tmp = selection & (1 << val);
        if tmp>0
        {
            rev |= 1 << ((width - 1) - val);
        }
        val = val + 1;
    }
    
    let mut output = num;
    for bit in lsb .. lsb + width {
        let a = get_bit_at(rev, bit);
        if a {
            output |= 1 << (bit + lsb);
        } else {
            output &= !(1 << (bit + lsb));
        }
        
    }
    
    return output;
}
