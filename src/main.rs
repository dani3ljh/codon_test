use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RNANucleotides {
    A,
    U,
    G,
    C
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DNANucleotides {
    A,
    T,
    G,
    C
}

#[derive(Debug, Deserialize)]
struct StringifiedTRNA {
    codon: String,
    amino_acid: String,
    letter: char,
    full_name: String
}

#[derive(Debug)]
struct TRNA {
    codon: [RNANucleotides;3],
    amino_acid: AminoAcid
}

#[derive(Debug, Clone)]
struct AminoAcid {
    abreiviation: [char;3],
    letter: char,
    name: String
}

fn read_codons_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<StringifiedTRNA>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as a vector of Codons.
    let codons = serde_json::from_reader(reader)?;

    Ok(codons)
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

fn transcription(dna_genetic_code: &Vec<DNANucleotides>) -> Result<Vec<RNANucleotides>, String> {
    let mut mrna = Vec::new();

    for nucleotide in dna_genetic_code {
        mrna.push(match &nucleotide {
            DNANucleotides::A => RNANucleotides::U,
            DNANucleotides::T => RNANucleotides::A,
            DNANucleotides::C => RNANucleotides::G,
            DNANucleotides::G => RNANucleotides::C,
        });
    }

    Ok(mrna)
}

fn translation(vec_of_trna: &Vec<TRNA>, mrna: &Vec<RNANucleotides>) -> Result<Vec<AminoAcid>, String> {
    if mrna.len() < 3 {
        return Err(String::from("Not enough codons for translation"));
    }

    let mut amino_acids = Vec::new();

    let mut i = 0;
    while i < mrna.len() - 2 {
        for trna in vec_of_trna {
            if trna.codon[0] != mrna[i] || trna.codon[1] != mrna[i + 1] || trna.codon[2] != mrna[i + 2] {
                continue;
            }
            amino_acids.push(trna.amino_acid.clone());
            break;
        }
        i += 3;
    }

    Ok(amino_acids)
}

fn string_to_dna_genetic_code(dna_string: &String) -> Result<Vec<DNANucleotides>, String> {
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

fn print_dna_mrna_and_amino_acids(trna: &Vec<TRNA>, dna: &Vec<DNANucleotides>) -> Result<(), String> {
    let mrna = transcription(dna)?;
    let amino_acids = translation(trna, &mrna)?;
    
    print!("Template dna:\t");
    let mut i = 0;
    for nucleotide in dna {
        print!("{nucleotide:?}");
        i += 1;
        if i == 3 {
            i = 0;
            print!("/");
        }
    }
    i = 0;
    print!("\nmRNA:\t\t");
    for nucleotide in mrna {
        print!("{nucleotide:?}");
        i += 1;
        if i == 3 {
            i = 0;
            print!("/");
        }
    }
    print!("\namino acids:\t");
    for amino_acid in amino_acids {
        for c in amino_acid.abreiviation {
            print!("{c}");
        }
        print!("-");
    }

    Ok(())
}

fn main() -> Result<(), String>{
    let stringified_trna = read_codons_from_file("src/codons.json")
        .expect("File Reading Error");
    let trna = convert_stringified_trna_to_trna(stringified_trna)?;

    let example1_dna = string_to_dna_genetic_code(&String::from("TACCTTGGGGAATATACACGCTGGCTTCGATGAATC"))?;
    print_dna_mrna_and_amino_acids(&trna, &example1_dna)?;

    Ok(())
}