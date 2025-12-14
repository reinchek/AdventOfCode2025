use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Clear screen.
pub fn clrscr(day: Option<u8>) {
    print!("\x1B[2J");

    match day {
        Some(day_number) => {
            println!("██████╗  █████╗ ██╗   ██╗");
            println!("██╔══██╗██╔══██╗╚██╗ ██╔╝");
            println!("██║  ██║███████║ ╚████╔╝");
            println!("██║  ██║██╔══██║  ╚██╔╝");
            println!("██████╔╝██║  ██║   ██║");
            print!("╚═════╝ ╚═╝  ╚═╝   ╚═╝");
            println!(" 🗓️ {day_number}");
        },
        None => ()
    };
}

pub fn read_input(day: u32) -> String {
    std::fs::read_to_string(format!("inputs/day{:02}.txt", day))
        .expect("Input file not found")
}

pub enum Direction {
    Left,
    Right
}

impl Direction {
    pub fn from(txt: &str) -> Self {
        if txt.starts_with("L") {
            Self::Left
        } else {
            Self::Right
        }
    }
}

#[derive(Debug)]
pub struct Node {
    number: u8,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}
#[derive(Debug, Clone)]
pub struct CircularLinkedList {
    head: Rc<RefCell<Node>>
}

impl CircularLinkedList {
    pub fn new(length: u8) -> Self {
        let mut head = Rc::new(RefCell::new(Node {
            number: 0,
            next: None,
            prev: None,
        }));

        let mut prev = Rc::clone(&head);
        let mut first = Rc::clone(&head);

        for i in 1..length {
            let node = Rc::new(RefCell::new(Node {
                number: i,
                next: None,
                prev: Some(Rc::downgrade(&prev)),
            }));

            prev.borrow_mut().next = Some(Rc::clone(&node));
            prev = node;
        }

        prev.borrow_mut().next = Some(Rc::clone(&first));
        first.borrow_mut().prev = Some(Rc::downgrade(&prev));

        Self { head }
    }

    pub fn move_forward(&mut self, times: u32, hits_zero: &mut u32) -> &mut Self {
        let mut current = Rc::clone(&self.head);
        for _ in 0..times {
            let new_current = current.borrow().next.as_ref().unwrap().clone();
            if new_current.borrow().number == 0 {
                *hits_zero += 1;
            }
            current = new_current;
        }
        self.head = current;

        self
    }

    pub fn move_backward(&mut self, times: u32, hits_zero: &mut u32) -> &mut Self {
        let mut current = Rc::clone(&self.head);
        for _ in 0..times {
            let new_current = current.borrow().prev.as_ref().unwrap().clone();
            current = new_current.upgrade().unwrap();

            if current.borrow().number == 0 {
                *hits_zero += 1;
            }
        }
        self.head = current;

        self
    }

    pub fn get_head(&self) -> u8 {
        self.head.borrow().number
    }

    pub fn traverse_forward(&self, times: u32) -> u8 {
        let mut current = Rc::clone(&self.head);
        for _ in 0..times {
            let new_current = current.borrow().next.as_ref().unwrap().clone();
            current = new_current;
        }
        current.borrow().number
    }

    pub fn traverse_backward(&self, times: u32) -> u8 {
        let mut current = Rc::clone(&self.head);
        for _ in 0..times {
            let new_current = current.borrow().prev.as_ref().unwrap().clone();
            current = new_current.upgrade().unwrap();
        }
        current.borrow().number
    }
}

pub fn add_mod_100(value: u8, delta: i32) -> u8 {
    let v = value as i32;
    let r = (v + delta).rem_euclid(100);
    r as u8
}
