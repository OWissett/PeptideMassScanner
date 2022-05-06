# Peptide Mass Scanner - v 0.1.0

<div style='text-align: justify'>
A simple tool for generating and searching peptide mass fragments from a protein sequence written in <b>Rust</b>. This tool is intended to aid in analysis of protein mass-spectrometry results.
<br><hr>
My motivation for making this tool was that, as a biochemist, I frequently want to check if a specific mass is possible to be generated as a substring of a protein sequence. Other tools do exist to perform similar tasks, however, they are complex and not very convenient to use from the command line, as such, I am writing this CLI tool.

This project is currently a work in progress.
</div>
<hr>

## Installing
Since only the source is provided here in this repo, to use the tool you must build the program yourself. To build this program, you will require an installation of Rust and Cargo (see here for details https://doc.rust-lang.org/cargo/getting-started/installation.html).

Note: This software has been developed on MacOS Monterey 12.0.1 on arm64 (M1) and has not been tested on x86 architectures.

## CLI Usage
``` bash
mass_scanner [OPTIONS] <TARGET_MASS> <SEQ_PATH>
```
|Option|Description|
|------|-----------|

WIP



## Input

Currently, two file types can be used as an input for the protein sequence:

|Type|Description|Extension|
|----|-----------|---------|
|**FASTA**| FASTA files must only contain canonical amino acids and be correctly formatted. | .fasta|
|**Comma delimited values**| CSV files must contain the sequence name in the first column and the sequence in the second column. The file must also be headerless and be delimited by commas (not tabs or periods)|.csv|


