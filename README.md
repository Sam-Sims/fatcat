# fatcat

**FA**S**T**Q **cat**

A paged, pretty printer for FASTQ files.

Like `zcat reads.fastq.gz | less` but nicer.

Kind of a gimmick, mildly useful.

![fatcat](img/terminal2.png)

## Installation

### Cargo:

Requires [cargo](https://www.rust-lang.org/tools/install)

```
cargo install fatcat
```

### Build from source:

#### Install rust toolchain:

To install please refer to the rust documentation: [docs](https://www.rust-lang.org/tools/install)

#### Clone the repository:

```bash
git clone https://github.com/Sam-Sims/fatcat
```

#### Build and add to path:

```bash
cd fatcat
cargo build --release
export PATH=$PATH:$(pwd)/target/release
```

All executables will be in the directory fatcat/target/release.

## Usage

### Basic usage:

```bash
fatcat <fastq_file> [options]
```
