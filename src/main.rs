use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
enum Action {
    Add,
    Sub,
    Mul,
    Div,
    RSub,
    RDiv,
}

impl Action {
    fn perform(&self, a: i32, b: i32) -> Option<i32> {
        match self {
            Self::Add => Some(a + b),
            Self::Sub => Some(a - b),
            Self::RSub => Some(b - a),
            Self::Mul => Some(a * b),
            Self::Div => {
                Self::check_div(a, b)?;
                Some(a / b)
            }
            Self::RDiv => {
                Self::check_div(b, a)?;
                Some(b / a)
            }
        }
    }

    fn check_div(a: i32, b: i32) -> Option<()> {
        if b != 0 && (a / b) as f64 == a as f64 / b as f64 {
            Some(())
        } else {
            None
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Action::Add => "+".to_owned(),
            Action::Sub => "-".to_owned(),
            Action::RSub => "-".to_owned(),
            Action::Mul => "*".to_owned(),
            Action::Div => "÷".to_owned(),
            Action::RDiv => "÷".to_owned(),
        };
        write!(f, "{}", symbol)?;
        Ok(())
    }
}

#[derive(Debug)]
enum Number {
    Single(i32),
    Double(Action, Box<Number>, Box<Number>, i32),
}

impl Number {
    fn single(n: i32) -> Self {
        Self::Single(n)
    }

    fn double(action: Action, a: Number, b: Number) -> Option<Self> {
        let (a_val, b_val) = (a.value(), b.value());
        Some(Self::Double(
            action,
            Box::new(a),
            Box::new(b),
            action.perform(a_val, b_val)?,
        ))
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Self::Single(n) => Self::Single(*n),
            Self::Double(action, a, b, value) => {
                Self::Double(*action, a.clone(), b.clone(), *value)
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Number::Single(n) => n.to_string(),
            Number::Double(action, a, b, _) => match action {
                Action::RSub | Action::RDiv => format!("({b} {action} {a})"),
                _ => format!("({a} {action} {b})"),
            },
        };
        write!(f, "{}", string)?;
        Ok(())
    }
}

impl Number {
    fn value(&self) -> i32 {
        match self {
            Self::Single(n) => *n,
            Self::Double(_, _, _, value) => *value,
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        self.value() == *other
    }
}

impl Eq for Number {}

type Combination = Vec<Number>;

struct MyPermutator {
    init_combination: Combination,
    a: usize,
    b: usize,
    actions: ActionIter,
}

impl MyPermutator {
    fn new(numbers: Combination) -> Self {
        MyPermutator {
            init_combination: numbers,
            a: 0,
            b: 1,
            actions: Action::iter(),
        }
    }

    fn next_pair(&mut self) -> Option<()> {
        self.b += 1;
        if self.b == self.init_combination.len() {
            self.a += 1;
            if self.a == self.init_combination.len() - 1 {
                return None;
            }
            self.b = self.a + 1;
        }
        Some(())
    }
}

impl Iterator for MyPermutator {
    type Item = Combination;

    fn next(&mut self) -> Option<Self::Item> {
        if self.init_combination.len() == 1 {
            return None;
        }

        let action = self.actions.next();
        let action = match action {
            Some(action) => action,
            None => {
                self.next_pair()?;
                self.actions = Action::iter();
                self.actions.next().unwrap()
            }
        };

        let new_number = Number::double(
            action,
            self.init_combination[self.a].clone(),
            self.init_combination[self.b].clone(),
        );

        match new_number {
            Some(new_number) => {
                let mut result = vec![new_number];

                for (i, item) in self.init_combination.iter().enumerate() {
                    if i != self.a && i != self.b {
                        result.push(item.clone())
                    }
                }
                Some(result)
            }
            None => self.next(),
        }
    }
}

fn solve() -> Option<Number> {
    let target = 334;

    let init_vec = vec![100, 50, 25, 8, 5, 8];

    let init_numbers: Combination = init_vec.iter().map(|x| Number::single(*x)).collect();
    let mut combinations = vec![init_numbers];

    while !combinations.is_empty() {
        let mut new_combinations: Vec<Combination> = vec![];
        for combination in combinations {
            let my_permutator = MyPermutator::new(combination);

            for item in my_permutator {
                if item[0] == target {
                    return Some(item[0].clone());
                }
                new_combinations.push(item);
            }
        }
        combinations = new_combinations;
    }
    return None;
}

fn main() {
    let solution = solve();
    match solution {
        Some(number) => println!("{} = {}", number, number.value()),
        None => println!("There's no solution"),
    }
}
