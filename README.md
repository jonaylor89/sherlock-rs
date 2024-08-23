

<p align=center>
  <br>
  <a href="https://sherlock-project.github.io/" target="_blank"><img src="docs/images/sherlock-logo.png"/></a>
  <br>
  <span>Hunt down social media accounts by username across <a href="https://sherlockproject.xyz/sites">400+ social networks</a></span>
  <br>
</p>

<p align="center">
  <a href="https://sherlockproject.xyz/installation">Installation</a>
  &nbsp;&nbsp;&nbsp;•&nbsp;&nbsp;&nbsp;
  <a href="https://sherlockproject.xyz/usage">Usage</a>
  &nbsp;&nbsp;&nbsp;•&nbsp;&nbsp;&nbsp;
  <a href="https://sherlockproject.xyz/contribute">Contributing</a>
</p>

<p align="center">
<img width="70%" height="70%" src="docs/images/demo.png"/>
</a>
</p>


## Usage

```shell
sherlock user123
```
To search for more than one user:
```shell
sherlock user1 user2 user3
```
Accounts found will be stored in an individual text file with the corresponding username (e.g user123.txt).

```shell
$ sherlock --help

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

## Motivation

I strongly dislike deploying python applications and wanted a single binary. Go get's on my nerves, Swift doesn't have great tooling server-side and on linux, C++ isn't even a question, so Rust it is.

## Credits

Thank you to everyone who has contributed to Sherlock! ❤️

<a href="https://github.com/jonaylor89/sherlock-rs/graphs/contributors">
  <img src="https://contrib.rocks/image?&columns=25&max=10000&&repo=jonaylor89/sherlock-rs" noZoom />
</a>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=jonaylor89/sherlock-rs&type=Date&theme=dark" />
  <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=jonaylor89/sherlock-rs&type=Date" />
  <img alt="Sherlock Project Star History Chart" src="https://api.star-history.com/svg?repos=jonaylor89/sherlock-rs&type=Date" />
</picture>

## License

MIT © Johannes Naylor<br/>
Original Creator of Sherlock - [Siddharth Dushantha](https://github.com/sdushantha)

## Thanks

- [Siddharth Dushantha](https://github.com/sdushantha) for creating the original Sherlock
- [Eira Fransham](https://github.com/eira-fransham) for giving me the crate name
