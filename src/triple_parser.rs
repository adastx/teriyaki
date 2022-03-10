use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, self};

const TYPE_STRING: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";

pub(crate) struct Triple {
	sub: u32,
	pred: u32,
    obj: u32,
    isType: bool
}

pub(crate) fn push_triples_into_vector(dict: &HashMap<String, u32>) -> Result<Vec<Triple>, io::Error> {
    let filename = "/Users/alankhorsid/Documents/Datalogi/6. semester/P6/teriyaki/datasets/example.nt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut vector_of_triples: Vec<Triple> = Vec::new();
    let mut is_type_pred = false;

    for line in reader.lines() {
        let line = line?;
        let line_splits: Vec<&str> = line.split(" ").collect();

        if line_splits[1] == TYPE_STRING {
            is_type_pred = true;
        }
        //GetID does not exist. Its pseudo 
        vector_of_triples.push(Triple{sub: *dict.get(line_splits[0]).unwrap(), pred: *dict.get(line_splits[1]).unwrap(), obj: *dict.get(line_splits[2]).unwrap(), isType: is_type_pred});
        //println!("{}", line_splits[i])
    }
    Ok(vector_of_triples)
}