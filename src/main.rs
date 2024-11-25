use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Transaction {
    id: String,
    data: String,
}

#[derive(Debug)]
struct DAG {
    transations : HashMap<String, Transaction>,
}

impl DAG {
    //create new empty DAG
    fn new() -> DAG {
        DAG {
            transations: HashMap::new(),
        }
    }

    
}

fn main() {
    let tx = Transaction {
        id: "T1".to_string(),
        data: "Donnée de test".to_string(),
    };

    println!("Transaction créée {:?}", tx);
}
