# rentr
A utility for running arbitrary commands when files change.

## Installation
`cargo install rentr`

## Usage
`find . | grep \.rs | rentr -c cargo build`
`find . | grep \.rs | rentr -c echo "CHANGED"`