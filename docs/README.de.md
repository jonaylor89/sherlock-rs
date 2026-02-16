
<p align=center>
  <br>
  <a href="https://sherlock-project.github.io/" target="_blank"><img src="docs/images/sherlock-logo.png"/></a>
  <br>
  <span>Soziale Medienkonten anhand des Benutzernamens auf über <a href="https://sherlockproject.xyz/sites">400+ sozialen Netzwerken</a> finden</span>
  <br>
</p>

<p align="center">
<img width="70%" height="70%" src="docs/images/demo.png"/>
</a>
</p>


## Installation

```shell
cargo install sherlock
```

## Nutzung

```shell
sherlock user123
```
Um nach mehr als einem Benutzer zu suchen:
```shell
sherlock user1 user2 user3
```
Gefundene Konten werden in einer einzelnen Textdatei mit dem entsprechenden Benutzernamen (z.B. user123.txt) gespeichert.

```shell
$ sherlock --help

Soziale Medienkonten anhand des Benutzernamens finden

Verwendung: sherlock-rs [OPTIONS] <usernames>...

Argumente:
  <usernames>...  Ein oder mehrere Benutzernamen, die in sozialen Netzwerken überprüft werden sollen. Ähnliche Benutzernamen mit {?} überprüfen (ersetzen durch '_', '-', '.')

Optionen:
  -v, --verbose                        Zusätzliche Debugging-Informationen und Metriken anzeigen
  -o, --output <OUTPUT_FILE>           Die Ausgabedatei zum Speichern der Ergebnisse
  -f, --output-folder <OUTPUT_FOLDER>  Wenn ein einzelner Benutzername verwendet wird, wird die Ausgabe des Ergebnisses in diesem Ordner gespeichert
  -c, --csv                            Komma-getrennte Werte (CSV) Datei erstellen
      --xlsx                           Die Standarddatei für die moderne Microsoft Excel-Tabelle (xlsx) erstellen
  -s, --site-list <SITE_LIST>          Analyse auf die gelisteten Seiten beschränken. Mehrere Optionen hinzufügen, um mehr als eine Seite anzugeben
  -p, --proxy <PROXY>
  -d, --dump-response                  Die HTTP-Anfrage zur gezielten Fehlersuche in stdout ausgeben
  -j, --json <JSON_FILE>               Daten aus einer JSON-Datei oder einer gültigen Online-JSON-Datei laden
  -t, --timeout <TIMEOUT>              Zeit (in Sekunden) für die Antwort auf Anfragen warten [Standard: 60]
      --print-all                      Seiten ausgeben, auf denen der Benutzername nicht gefunden wurde
      --print-found                    Seiten ausgeben, auf denen der Benutzername gefunden wurde
  -n, --no-color                       Terminalausgabe nicht einfärben
  -b, --browse                         Alle Ergebnisse im Standardbrowser anzeigen
  -l, --local                          Erzwingt die Verwendung der lokalen data.json-Datei
      --nsfw                           Überprüfung von NSFW-Seiten aus der Standardliste einschließen
  -h, --help                           Hilfe anzeigen
  -V, --version                        Version anzeigen
```

## Installation

```shell
cargo install sherlock
```

## Motivation

Ich mag es überhaupt nicht, Python-Anwendungen bereitzustellen, und wollte ein einziges Binärprogramm. Go nervt mich, Swift hat keine großartigen Server-Side-Tools und unter Linux ist C++ keine Option, also ist Rust die Lösung.

<p align="center">
<img width="70%" height="70%" src="docs/images/reddit.png"/>
</a>
</p>

## Lizenz

MIT © Johannes Naylor<br/>
Originaler Ersteller von Sherlock - [Siddharth Dushantha](https://github.com/sdushantha)

## Danksagungen

- [Siddharth Dushantha](https://github.com/sdushantha) für die Erstellung des originalen Sherlock
- [Eira Fransham](https://github.com/eira-fransham) für den Crate-Namen
