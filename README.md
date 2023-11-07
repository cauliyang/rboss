# Rust Bioinformatics Toolbox (rboss)

Rust Bioinformatics Toolbox, abbreviated as rboss, is a command-line tool designed to facilitate various operations on bioinformatics files.
It provides a range of commands to manipulate sequence data files commonly used in bioinformatics, such as BAM, FASTA, and FASTQ formats.

## Minimum Supported Rust Version (MSRV)

This project adheres to a Minimum Supported Rust Version (MSRV) policy.
The Minimum Supported Rust Version (MSRV) is 1.70.0.
We ensure that all code within the project is compatible with this version or newer to maintain stability and compatibility.

## Features

- **High Performance**: Take advantage of Rust's performance for heavy computational tasks.
- **Safety**: Rust's strong type system and ownership model ensure safety across bioinformatics operations.
- **Concurrency**: Utilize Rust's modern concurrency tools for parallel data processing.

## Installation

You can install `rboss` using Cargo, the Rust package manager. Ensure you have Rust and Cargo installed on your system, then run:

```sh
cargo install rboss
```

## Getting Started

To begin using `rboss`, you may invoke it with the following syntax:

```sh
rboss [OPTIONS] [COMMAND]
```

`rboss` provides several commands to perform different operations:

### Commands

- `extract` (aliases: `e`): Extract reads from a BAM file.

  Usage:

  ```sh
  rboss extract [OPTIONS] <BAM_FILE>
  ```

- `index`: Index a BAM file to speed up read access.

  Usage:

  ```sh
  rboss index <BAM_FILE>
  ```

- `fa2fq`: Convert a FASTA file to FASTQ format.

  Usage:

  ```sh
  rboss fa2fq <FASTA_FILE>
  ```

- `fq2fa`: Convert a FASTQ file to FASTA format.

  Usage:

  ```sh
  rboss fq2fa <FASTQ_FILE>
  ```

- `rsoft`: Create soft links to files with the same suffix in one directory recursively.

  Usage:

  ```sh
  rboss rsoft <TARGET_DIR> -s <SUFFIX>
  ```

- `help`: Print detailed help information for `rboss` or its subcommands.

  Usage:

  ```sh
  rboss help [COMMAND]
  ```

### Options

- `--generate <GENERATOR>`: Generate shell completions for `rboss` for the specified shell. Possible values include `bash`, `elvish`, `fish`, `powershell`, and `zsh`.

- `-v`, `--verbose`: Increase verbosity. The more occurrences of this flag, the more detailed the output.

- `-q`, `--quiet`: Decrease verbosity. The more occurrences of this flag, the less detailed the output.

- `-h`, `--help`: Print help information for `rboss` and its subcommands.

- `-V`, `--version`: Print the version information for `rboss`.

## Examples

Extracting reads from a BAM file based on `reads.txt`:

```sh
rboss extract reads.txt sample.bam
```

Indexing a BAM file:

```sh
rboss index sample.bam
```

Converting a FASTA file to FASTQ:

```sh
rboss fa2fq sample.fasta
```

Converting a FASTQ file to FASTA:

```sh
rboss fq2fa sample.fastq
```

Creating soft links for files with a `.txt` or `.csv` suffix:

```sh
rboss rsoft /path/to/directory -s txt csv
```

For further help on any specific command, you can use the `help` command:

```sh
rboss help extract
```

For the latest updates and more detailed documentation, please visit the official `rboss` repository.

## Contributing

Contributions to `rboss` are welcome. If you have suggestions for improvements or have identified issues, please open an issue or a pull request in the repository.

Thank you for using `rboss`, the versatile Rust-powered toolbox for bioinformatics data manipulation!

## License

rboss is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See LICENSE-APACHE and LICENSE-MIT for details.

## Community

Join us on [Discord/Gitter/Forum] to discuss rboss development, ask questions, and collaborate with other contributors.

## Credits

rboss is being actively developed and maintained by bioinformatics enthusiasts and the Rust community.
We thank all contributors for their efforts.
