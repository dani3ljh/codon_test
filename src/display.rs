use crate::logic::DNANucleotides;
use crate::logic::RNANucleotides;
use crate::logic::AminoAcid;

// prints dna in format ABC/DEF/GHI ...
pub fn print_dna(dna: &Vec<DNANucleotides>) {
    let mut i = 0;
    for nucleotide in dna {
        print!("{nucleotide:?}");
        i += 1;
        if i == 3 {
            i = 0;
            print!("/");
        }
    }
    print!("\n");
}

// prints rna in format: ABC/DEF/GHI ...
pub fn print_rna(rna: &Vec<RNANucleotides>) {
    let mut i = 0;
    for nucleotide in rna {
        print!("{nucleotide:?}");
        i += 1;
        if i == 3 {
            i = 0;
            print!("/");
        }
    }
    print!("\n");
}

pub fn print_amino_acid_abreviations(amino_acids: &Vec<AminoAcid>) {
    for amino_acid in amino_acids {
        for c in amino_acid.abreiviation {
            print!("{c}");
        }
        print!("-");
    }
    print!("\n");
}