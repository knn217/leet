impl Solution {
    fn map(num: &i32) -> String {
        match *num
        {
            1 => {return String::from("I");}
            2 => {return String::from("II");}
            3 => {return String::from("III");}
            4 => {return String::from("IV");}
            5 => {return String::from("V");}
            6 => {return String::from("VI");}
            7 => {return String::from("VII");}
            8 => {return String::from("VIII");}
            9 => {return String::from("IX");}
            10 => {return String::from("X");}
            20 => {return String::from("XX");}
            30 => {return String::from("XXX");}
            40 => {return String::from("XL");}
            50 => {return String::from("L");}
            60 => {return String::from("LX");}
            70 => {return String::from("LXX");}
            80 => {return String::from("LXXX");}
            90 => {return String::from("XC");}
            100 => {return String::from("C");}
            200 => {return String::from("CC");}
            300 => {return String::from("CCC");}
            400 => {return String::from("CD");}
            500 => {return String::from("D");}
            600 => {return String::from("DC");}
            700 => {return String::from("DCC");}
            800 => {return String::from("DCCC");}
            900 => {return String::from("CM");}
            1000 => {return String::from("M");}
            2000 => {return String::from("MM");}
            3000 => {return String::from("MMM");}
            _ => {return String::from("");}
        }
        return String::from("");
    }
    pub fn int_to_roman(num: i32) -> String {
        let mut roman = String::new();
        let mut num = num;
        let mut base = 10;
        while num > 0 {
            let digit = num % base;
            num -= digit;
            base *= 10;
            roman.insert_str(0, &Self::map(&digit));
            // println!("Digit is {}", digit);
        }
        roman
    }
}