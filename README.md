# Peptide Mass Scanner

A simple tool for generating and searching peptide mass fragments from a protein sequence written in **Rust**. This tool is indended to aid in analysis of protein mass-spectrometry results.

My motivation for making this tool was that, as a biochemist, I frequently want to check if a specific mass is possible to be generated as a substring of a protein sequence. Othertools do exist to perform this task, however, they are complex and not very convient to use from the command line, as such, I am writting this CLI tool.

This project is currently a work in progress.

## Example
``` bash
mass_scanner [OPTIONS] <TARGET_MASS> <SEQ_PATH>
```


