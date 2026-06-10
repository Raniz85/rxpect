#[derive(Debug, PartialEq, Eq)]
pub struct Parent {
    name: String,
    age: u32,
    children: Vec<Child>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Child {
    name: String,
    age: u32,
}

impl Parent {
    pub fn hammersmith() -> Self {
        Parent {
            name: "Alice Hammersmith".to_string(),
            age: 41,
            children: vec![Child::theodore(), Child::clementine()],
        }
    }

    pub fn quincey() -> Self {
        Parent {
            name: "Bartholomew Quincey".to_string(),
            age: 37,
            children: vec![Child::rosalind()],
        }
    }

    pub fn featherstone() -> Self {
        Parent {
            name: "Cordelia Featherstone".to_string(),
            age: 52,
            children: vec![Child::maximilian(), Child::genevieve(), Child::sebastian()],
        }
    }
}

impl Child {
    pub fn theodore() -> Self {
        Child {
            name: "Theodore Hammersmith".to_string(),
            age: 8,
        }
    }

    pub fn clementine() -> Self {
        Child {
            name: "Clementine Hammersmith".to_string(),
            age: 5,
        }
    }

    pub fn rosalind() -> Self {
        Child {
            name: "Rosalind Quincey".to_string(),
            age: 12,
        }
    }

    pub fn maximilian() -> Self {
        Child {
            name: "Maximilian Featherstone".to_string(),
            age: 19,
        }
    }

    pub fn genevieve() -> Self {
        Child {
            name: "Genevieve Featherstone".to_string(),
            age: 16,
        }
    }

    pub fn sebastian() -> Self {
        Child {
            name: "Sebastian Featherstone".to_string(),
            age: 14,
        }
    }
}
