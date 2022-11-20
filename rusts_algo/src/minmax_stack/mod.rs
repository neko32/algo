
use std::fmt::{Debug};
use std::cell::Cell;

pub fn exec<T>(instructions:Vec<Instruction<T>>)
where T:Debug + PartialEq + PartialOrd + Default + Copy {
    let mut st:MinMaxStack<T> = MinMaxStack::new();
    for inst in instructions {
        println!("inst - {:?}", inst);
        if inst.cmd == "push" {
            let before_siz = st.size();
            st.push(inst.value);
            let after_siz = st.size();
            assert!(before_siz + 1 == after_siz);
        } else if inst.cmd == "pop" {
            let before_siz = st.size();
            let v = st.pop();
            let after_siz = st.size();
            assert!(before_siz - 1 == after_siz);
            assert!(v.is_some() && v.unwrap() == inst.value);
        } else if inst.cmd == "peek" {
            let v = st.peek();
            assert!(v.is_some() && *v.unwrap() == inst.value);
        } else if inst.cmd == "max" {
            let v = st.max();
            assert!(v.is_some() && v.unwrap() == inst.value);
        } else if inst.cmd == "min" {
            let v = st.min();
            assert!(v.is_some() && v.unwrap() == inst.value);
        } else {
            println!("{} is not supported instruction", inst.cmd);
        }
    }
}

#[derive(Debug)]
pub struct Instruction<T>
where T:Debug + PartialEq + PartialOrd + Default + Copy {
    cmd:String,
    value:T,
}

impl <T> Instruction<T>
where T:Debug + PartialEq + PartialOrd + Default + Copy {
    pub fn new(cmd:&str, value:T) -> Self {
        Instruction { cmd: cmd.to_string(), value: value }
    }
}

pub fn run() {
    let inss = vec![
        Instruction::new("push", 3),
        Instruction::new("push", 7),
        Instruction::new("push", 5),
        Instruction::new("push", 6),
        Instruction::new("peek", 6),
        Instruction::new("pop", 6),
        Instruction::new("max", 7),
        Instruction::new("min", 3),
    ];
    exec(inss);
}

struct MinMaxStack<T>
where T:Debug + PartialEq + PartialOrd + Default + Copy {
    st:Cell<Vec<T>>,
    minmax_stack:Cell<Vec<(T,T)>>,
}

impl <T> MinMaxStack<T>
where T:Debug + PartialEq + PartialOrd + Default + Copy {
    fn new() -> Self {
        MinMaxStack { st: Cell::new(Vec::new()), minmax_stack: Cell::new(Vec::new()) }
    }

    fn peek(&mut self) -> Option<&T> {
        self.st.get_mut().last()
    }

    fn pop(&mut self) -> Option<T> {
        self.minmax_stack.get_mut().pop();
        self.st.get_mut().pop()
    }

    fn push(&mut self, v:T) {
        self.st.get_mut().push(v);
        let minmax_v = self.minmax_stack.get_mut().last();
        match minmax_v {
            None => {
                self.minmax_stack.get_mut().push((v, v));
            },
            Some((min_v, max_v)) => {
                let new_minv = if *min_v < v { *min_v } else { v };
                let new_maxv = if *max_v > v { *max_v } else { v };
                self.minmax_stack.get_mut().push((new_minv, new_maxv));
            }
        }
    }

    fn max(&mut self) -> Option<T> {
        let ov = self.minmax_stack.get_mut().last();
        match ov {
            None => None,
            Some((_, maxv)) => Some(*maxv)
        }
    }

    fn min(&mut self) -> Option<T> {
        let ov = self.minmax_stack.get_mut().last();
        match ov {
            None => None,
            Some((minv, _)) => Some(*minv)
        }
    }

    fn size(&mut self) -> usize {
        self.st.get_mut().len()
    }

}