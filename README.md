## Priorities:
- Linux: Created and firstly supported on linux. This means that linux will get the most development time and will have update priority.
- Windows: windows will be a run target so we will attempt to garantee builds and security
- Os X: probably works find (untested)

## dependecies:
- pdftoppm/poppler-utils (Might be switched out for [poppler (crate)](https://crates.io/crates/poppler) or [poppler-rs (crate)](https://crates.io/crates/poppler-rs) in the future
  - LINUX: preinstalled on most linux distros and installable as poppler-utils
  - MAC: [homebrew page](https://formulae.brew.sh/formula/poppler)
  - WINDOWS/MAC/LINUX: [xpdfreader.com](https://www.xpdfreader.com/download.html)
- tesseract ocr
  - [tesseract installation instructions](https://tesseract-ocr.github.io/tessdoc/Installation.html)


## goals:
- taking in multiple formats
    - coned
    - hyper
    - and other (add later)
- automatically generate xsl skelletons and pottentially other formats later

## documentation goals:
- document how to use a terminal and use basic commands like commands like cd
- document how to run checks
- document how to set up filestructure for checks

## Licencing: TBD
