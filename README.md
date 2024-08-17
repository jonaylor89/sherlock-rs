
# Sherlock

the sherlock tool but writen in rust because I wanted a single binary

## Usage

```shell
Hunt down social media accounts by username

Usage: sherlock-rs [OPTIONS] <usernames>...

Arguments:
  <usernames>...  One or more usernames to check with social networks. Check similar usernames using {?} (replace to '_', '-', '.')

Options:
  -v, --verbose                        Display extra debugging information and metrics
  -o, --output <OUTPUT_FILE>           The output file to save the results to
  -f, --output-folder <OUTPUT_FOLDER>  If using single username, the output of the result will be saved to this file
  -c, --csv                            Create Comma-Separated Values (CSV) File
      --xlsx                           Create the standard file for the modern Microsoft Excel spreadsheet (xlsx)
  -s, --site-list <SITE_LIST>          Limit analysis to just the listed sites. Add multiple options to specify more than one site
  -p, --proxy <PROXY>
  -d, --dump-response                  Dump the HTTP request to stdout for targeted debugging
  -j, --json <JSON_FILE>               Load data from a JSON file or an online, valid, JSON file
  -t, --timeout <TIMEOUT>              Time (in seconds) to wait for response to requests [default: 60]
      --print-all                      Output sites where the username was not found
      --print-found                    Output sites where the username was found
  -n, --no-color                       Don't color terminal output
  -b, --browse                         Browse to all results on default browser
  -l, --local                          Force the use of the local data.json file
      --nsfw                           Include checking of NSFW sites from default list
  -h, --help                           Print help
  -V, --version                        Print version
```

## Installation

```shell
cargo install sherlock
```
