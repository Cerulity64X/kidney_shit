#![allow(arithmetic_overflow)]
#![allow(overflowing_literals)]

use std::{env::Args, fs, io::{stdout, Write}, /*collections::HashMap*/};

use getch::Getch;

#[derive(Clone, Debug)]
struct KSHead {
    script_location: isize,
    loop_brack_stack: Vec<isize>,
    sloop_brack_stack: Vec<isize>,
    mem_pointer: isize,
    current_char: char,
    script: String
}

impl KSHead {
    fn current(&mut self) -> Option<char> {
        match self.script.get(self.script_location as usize..(self.script_location + 1) as usize) {
            Some(s) => {
                let cco: Option<char> = s.chars().next();
                match cco {
                    Some(c) => {
                        self.current_char = c;
                        cco
                    },
                    None => {None}
                }
            },
            None => {None}
        }
    }
    fn next(&mut self) -> Option<char> {
        self.script_location += 1;
        self.current()
    }
    fn next_number(&mut self) -> i32 {
        let mut num: String = String::new();
        let mut c = self.next();
        let mut ch = c.unwrap_or_else(||'?');
        while "-0123456789".contains(ch) {
            if "0123456789".contains(ch) {num.push(c.unwrap());} else {break;}
            c = self.next();
            ch = c.unwrap_or_else(||'?');
        }
        self.script_location -= 1;
        i32::from_str_radix(num.as_str(), 10).unwrap_or_else(|_|0)
    }
}

/*fn char_at(str: &String, i: usize) -> Option<char> {
    match str.get(i..) {
        Some(s) => {s.chars().next()}
        None => None
    }
}*/

fn move_right(v: &mut Vec<i32>, head: &mut KSHead) {
    head.mem_pointer += 1; if head.mem_pointer >= v.len() as isize { v.push(0); }
}

fn move_left(v: &mut Vec<i32>, head: &mut KSHead) {
    head.mem_pointer -= 1; if head.mem_pointer < 0 { v.insert(0, 0); head.mem_pointer += 1; }
}

fn main() {
    let mut args: Args = std::env::args();
    args.next().unwrap();
    let path: String = args.next().expect("Enter an input file!");
    let file: String = fs::read_to_string(path).expect("Input file does not exist!");
    let mut heap: Vec<i32> = vec![];
    let mut stack: Vec<i32> = vec![];
    let mut heads: Vec<KSHead> = vec![KSHead { script_location: -1, loop_brack_stack: vec![], sloop_brack_stack: vec![], mem_pointer: 0, current_char: ' ', script: file}];
    let mut head: KSHead = (*heads.last().unwrap()).clone();
    /*let mut fstack: Vec<usize> = vec![];
    let mut funcs: HashMap<i32, KSHead> = HashMap::new();*/
    head.current();
    let g: Getch = Getch::new();
    loop {
        let co: Option<char> = head.next();
        match co {
            Some(c) => {
                match c {
                    '$' => { heap[head.mem_pointer as usize] += head.next_number(); }
                    '#' => { heap[head.mem_pointer as usize] -= head.next_number(); }
                    '=' => { heap[head.mem_pointer as usize] = head.next_number(); }
                    '+' => { heap[head.mem_pointer as usize] += 1; }
                    '-' => { heap[head.mem_pointer as usize] -= 1; }
                    '>' => { move_right(&mut heap, &mut head); }
                    '<' => { move_left(&mut heap, &mut head); }
                    '[' => { head.loop_brack_stack.push(head.script_location); }
                    ']' => {
                        if heap[head.mem_pointer as usize] != 0 {
                            match head.loop_brack_stack.last() {
                                Some(l) => { head.script_location = *l; }
                                None => {}
                            }
                        } else {
                            head.loop_brack_stack.pop();
                        }
                    }
                    '.' => {
                        print!("{}", match char::from_u32(heap[head.mem_pointer as usize] as u32) {Some(c) => c, None => '?'});
                        match stdout().flush(){Ok(_)=>{}Err(_)=>{}};
                    },
                    ',' => {
                        heap[head.mem_pointer as usize] = match g.getch() {
                            Ok(u) => { if u == 3 {break;} u.into() }
                            Err(_) => {0}
                        }
                    }
                    '!' => {
                        print!("{}", heap[head.mem_pointer as usize]);
                        match stdout().flush(){Ok(_)=>{}Err(_)=>{}};
                    }
                    's' => {
                        match head.next() {
                            Some(c) => {
                                match c {
                                    '+' => {
                                        let left: i32 = stack.pop().unwrap_or_else(||0);
                                        let right: i32 = stack.pop().unwrap_or_else(||0);
                                        stack.push(left + right);
                                    }
                                    '-' => {
                                        let left: i32 = stack.pop().unwrap_or_else(||0);
                                        let right: i32 = stack.pop().unwrap_or_else(||0);
                                        stack.push(left + right);
                                    }
                                    _ => {}
                                }
                            }
                            None => {}
                        }
                    }
                    '*' => {
                        let left: i32 = stack.pop().unwrap_or_else(||0);
                        let right: i32 = stack.pop().unwrap_or_else(||0);
                        stack.push(left * right);
                    }
                    '/' => {
                        let left: i32 = stack.pop().unwrap_or_else(||0);
                        let right: i32 = stack.pop().unwrap_or_else(||0);
                        stack.push(left / right);
                    }
                    '%' => {
                        let left: i32 = stack.pop().unwrap_or_else(||0);
                        let right: i32 = stack.pop().unwrap_or_else(||0);
                        stack.push(left % right);
                    }
                    '^' => {
                        stack.pop();
                    }
                    'v' => {
                        stack.push(head.next_number());
                    }
                    'o' => {
                        stack.push(heap[head.mem_pointer as usize]);
                    }
                    'p' => {
                        heap[head.mem_pointer as usize] = stack.pop().unwrap_or_else(||0);
                    }
                    'c' => {
                        heap[head.mem_pointer as usize] = *stack.last().unwrap_or_else(||&0);
                    }
                    't' => {
                        stack.push((head.next().unwrap_or_else(||'?') as u32) as i32);
                    }
                    'u' => {
                        heap[head.mem_pointer as usize] = (head.next().unwrap_or_else(||'?') as u32) as i32;
                    }
                    /*'f' => {
                        match head.next() {
                            Some(c) => {
                                match c {
                                    'd' => {
                                        let sl = head.script_location;
                                        let mut s: &mut String = &mut String::new();
                                        loop {
                                            let mut c: Option<char> = head.next();
                                            match c {
                                                Some(r) => {
                                                    if "\n\r".contains(r) {
                                                        break;
                                                    } else {
                                                        s.push(r);
                                                    }
                                                }
                                                None => {break;}
                                            }
                                        }
                                        let ksh: KSHead = KSHead { script_location: sl, loop_brack_stack: vec![], mem_pointer: 0, current_char: ' ', script: (*s).clone() };
                                        funcs.insert(heap[head.mem_pointer as usize], ksh);
                                    }
                                    't' => {
                                        if funcs.contains_key(&heap[head.mem_pointer as usize]) {
                                            heads.push((*funcs.get(&heap[head.mem_pointer as usize]).unwrap()).clone());
                                            head = heads.last().unwrap().clone();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            None => {}
                        }
                    }*/
                    '}' => {
                        for _ in 0..head.next_number() {
                            move_right(&mut heap, &mut head);
                        }
                    }
                    '{' => { 
                        for _ in 0..head.next_number() {
                            move_left(&mut heap, &mut head);
                        }
                    }
                    ';' => {
                        loop {
                            match head.next() {
                                Some(c) => {
                                    match c {
                                        '\n' => {break;}
                                        '\r' => {break;}
                                        _ => {}
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                    '(' => { head.sloop_brack_stack.push(head.script_location); }
                    ')' => {
                        if let Some(_) = stack.last() {
                            match head.sloop_brack_stack.last() {
                                Some(l) => { head.script_location = *l; }
                                None => {}
                            }
                        } else {
                            head.loop_brack_stack.pop();
                        }
                    }
                    _ => {}
                }
            }
            None => {
                match heads.pop() {
                    Some(_) => {
                        match heads.last() {
                            Some(l) => {
                                head = l.clone();
                            }
                            None => {break;}
                        }
                    }
                    None => {break;}
                }
            }
        }
    }
}
