#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RNANucleotides {
    A,
    U,
    G,
    C
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DNANucleotides {
    A,
    T,
    G,
    C
}

#[derive(Debug)]
pub struct TRNA {
    pub codon: [RNANucleotides;3],
    pub amino_acid: AminoAcid
}

#[derive(Debug, Clone)]
pub struct AminoAcid {
    pub abreiviation: [char;3],
    pub letter: char,
    pub name: String
}

pub fn transcription(dna_genetic_code: &Vec<DNANucleotides>) -> Result<Vec<RNANucleotides>, String> {
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

pub fn translation(vec_of_trna: &Vec<TRNA>, mrna: &Vec<RNANucleotides>) -> Result<Vec<AminoAcid>, String> {
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

pub fn dna_complement(dna: &Vec<DNANucleotides>) -> Vec<DNANucleotides> {
    let mut new_dna = Vec::new();

    for nucleotide in dna {
        new_dna.push(match nucleotide {
            DNANucleotides::A => DNANucleotides::T,
            DNANucleotides::T => DNANucleotides::A,
            DNANucleotides::C => DNANucleotides::G,
            DNANucleotides::G => DNANucleotides::C
        })
    }

    new_dna
}