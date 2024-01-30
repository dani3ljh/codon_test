use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::logic::DNANucleotides;
use crate::logic::RNANucleotides;
use crate::logic::AminoAcid;
use crate::logic::TRNA;

#[derive(Debug, Deserialize)]
struct StringifiedTRNA {
    codon: String,
    amino_acid: String,
    letter: char,
    full_name: String
}

pub fn read_codons_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<TRNA>, String> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)
        .expect("File Reading Error");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as a vector of Codons.
    let codons = serde_json::from_reader(reader)
        .expect("Error Deserializing");

    convert_stringified_trna_to_trna(codons)
}

pub fn string_to_dna_genetic_code(dna_string: &String) -> Result<Vec<DNANucleotides>, String> {
    let mut dna_code = Vec::new();

    for c in dna_string.chars() {
        dna_code.push(match &c {
            'A' => DNANucleotides::A,
            'T' => DNANucleotides::T,
            'C' => DNANucleotides::C,
            'G' => DNANucleotides::G,
            _ => return Err(format!("DNA nucleotide {c} not found"))
        })
    }

    Ok(dna_code)
}

fn convert_stringified_trna_to_trna(vec_of_stringified_trna: Vec<StringifiedTRNA>) -> Result<Vec<TRNA>, String> {
    let mut vec_of_trna = Vec::new();

    for stringified_trna in vec_of_stringified_trna {
        let codon = convert_rna_string_to_rna_codon(stringified_trna.codon)?;
        if stringified_trna.amino_acid.len() != 3 {
            return Err(String::from("Amino acid abreiviation not length 3"))
        }
        let mut abreiviation = ['\0';3];
        for (i, c) in stringified_trna.amino_acid.chars().enumerate() {
            abreiviation[i] = c;
        }
        let letter = stringified_trna.letter;
        let name = stringified_trna.full_name;
        
        let trna = TRNA {
            codon,
            amino_acid: AminoAcid {
                abreiviation,
                letter,
                name
            }
        };

        vec_of_trna.push(trna);
    }

    Ok(vec_of_trna)
}

fn convert_rna_string_to_rna_codon(rna_string: String) -> Result<[RNANucleotides;3], String> {
    let mut codon = [RNANucleotides::A;3];
    if rna_string.len() != 3 {
        return Err(String::from("RNA string not length 3"));
    }
    for (i, c) in rna_string.chars().enumerate() {
        codon[i] = match &c {
            'A' => RNANucleotides::A,
            'U' => RNANucleotides::U,
            'C' => RNANucleotides::C,
            'G' => RNANucleotides::G,
            _ => return Err(format!("Incorrect RNA nucleotide: {c}"))
        }
    }

    Ok(codon)
}
