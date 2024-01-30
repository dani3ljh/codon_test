use crate::display::print_dna;
use crate::display::print_rna;
use crate::display::print_amino_acid_abreviations;
use crate::input::read_codons_from_file;
use crate::input::string_to_dna_genetic_code;
use crate::logic::TRNA;
use crate::logic::transcription;
use crate::logic::translation;

mod display;
mod input;
mod logic;

fn main() -> Result<(), String>{
    let trna = read_codons_from_file("codons.json")?;

    example1(&trna)?;

    Ok(())
}

fn example1(trna: &Vec<TRNA>) -> Result<(), String> {
    let dna_example1 = string_to_dna_genetic_code(&String::from("TACCTTGGGGAATATACACGCTGGCTTCGATGAATC"))?;
    let rna_example1 = transcription(&dna_example1)?;
    let amino_acids_example1 = translation(trna, &rna_example1)?;

    print!("Template DNA:\t");
    print_dna(&dna_example1);
    print!("mRNA:\t\t");
    print_rna(&rna_example1);
    print!("Amino Acids:\t");
    print_amino_acid_abreviations(&amino_acids_example1);

    Ok(())
}