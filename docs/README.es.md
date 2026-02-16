
<p align=center>
  <br>
  <a href="https://sherlock-project.github.io/" target="_blank"><img src="docs/images/sherlock-logo.png"/></a>
  <br>
  <span>Encuentra cuentas de redes sociales por nombre de usuario en más de <a href="https://sherlockproject.xyz/sites">400 redes sociales</a></span>
  <br>
</p>

<p align="center">
<img width="70%" height="70%" src="docs/images/demo.png"/>
</a>
</p>


## Instalación

```shell
cargo install sherlock
```

## Uso

```shell
sherlock user123
```
Para buscar más de un usuario:
```shell
sherlock user1 user2 user3
```
Las cuentas encontradas se almacenarán en un archivo de texto individual con el nombre de usuario correspondiente (por ejemplo, user123.txt).

```shell
$ sherlock --help

Busca cuentas de redes sociales por nombre de usuario

Uso: sherlock-rs [OPTIONS] <usernames>...

Argumentos:
  <usernames>...  Uno o más nombres de usuario para verificar con las redes sociales. Comprueba nombres de usuario similares usando {?} (reemplaza por '_', '-', '.')

Opciones:
  -v, --verbose                        Mostrar información de depuración y métricas adicionales
  -o, --output <OUTPUT_FILE>           El archivo de salida para guardar los resultados
  -f, --output-folder <OUTPUT_FOLDER>  Si se usa un solo nombre de usuario, la salida del resultado se guardará en esta carpeta
  -c, --csv                            Crear archivo de valores separados por comas (CSV)
      --xlsx                           Crear el archivo estándar para la hoja de cálculo moderna de Microsoft Excel (xlsx)
  -s, --site-list <SITE_LIST>          Limitar el análisis solo a los sitios listados. Agrega múltiples opciones para especificar más de un sitio
  -p, --proxy <PROXY>
  -d, --dump-response                  Volcar la solicitud HTTP a stdout para depuración dirigida
  -j, --json <JSON_FILE>               Cargar datos desde un archivo JSON o un archivo JSON válido en línea
  -t, --timeout <TIMEOUT>              Tiempo (en segundos) para esperar la respuesta a las solicitudes [predeterminado: 60]
      --print-all                      Mostrar sitios donde el nombre de usuario no fue encontrado
      --print-found                    Mostrar sitios donde el nombre de usuario fue encontrado
  -n, --no-color                       No colorear la salida de la terminal
  -b, --browse                         Navegar a todos los resultados en el navegador predeterminado
  -l, --local                          Forzar el uso del archivo data.json local
      --nsfw                           Incluir la verificación de sitios NSFW de la lista predeterminada
  -h, --help                           Mostrar ayuda
  -V, --version                        Mostrar versión
```

## Instalación

```shell
cargo install sherlock
```

## Motivación

Realmente no me gusta implementar aplicaciones Python y quería un único binario. Go me pone de los nervios, Swift no tiene excelentes herramientas del lado del servidor y en Linux, C++ ni siquiera es una opción, así que Rust es la solución.

<p align="center">
<img width="70%" height="70%" src="docs/images/reddit.png"/>
</a>
</p>

## Licencia

MIT © Johannes Naylor<br/>
Creador original de Sherlock - [Siddharth Dushantha](https://github.com/sdushantha)

## Agradecimientos

- [Siddharth Dushantha](https://github.com/sdushantha) por crear el Sherlock original
- [Eira Fransham](https://github.com/eira-fransham) por darme el nombre del crate
