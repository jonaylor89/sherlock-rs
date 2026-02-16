
<p align=center>
  <br>
  <a href="https://sherlock-project.github.io/" target="_blank"><img src="docs/images/sherlock-logo.png"/></a>
  <br>
  <span>Найдите аккаунты в социальных сетях по имени пользователя в более чем <a href="https://sherlockproject.xyz/sites">400+ социальных сетях</a></span>
  <br>
</p>

<p align="center">
<img width="70%" height="70%" src="docs/images/demo.png"/>
</a>
</p>


## Установка

```shell
cargo install sherlock
```

## Использование

```shell
sherlock user123
```
Для поиска нескольких пользователей:
```shell
sherlock user1 user2 user3
```
Найденные аккаунты будут сохранены в отдельных текстовых файлах с соответствующим именем пользователя (например, user123.txt).

```shell
$ sherlock --help

Найдите аккаунты в социальных сетях по имени пользователя

Использование: sherlock-rs [OPTIONS] <usernames>...

Аргументы:
  <usernames>...  Одно или несколько имен пользователей для проверки в социальных сетях. Проверяйте похожие имена пользователей, используя {?} (замените на '_', '-', '.')

Опции:
  -v, --verbose                        Отображать дополнительную отладочную информацию и метрики
  -o, --output <OUTPUT_FILE>           Выходной файл для сохранения результатов
  -f, --output-folder <OUTPUT_FOLDER>  Если используется одно имя пользователя, результат будет сохранен в эту папку
  -c, --csv                            Создать файл значений, разделенных запятыми (CSV)
      --xlsx                           Создать стандартный файл для современной электронной таблицы Microsoft Excel (xlsx)
  -s, --site-list <SITE_LIST>          Ограничить анализ только перечисленными сайтами. Добавьте несколько опций, чтобы указать более одного сайта
  -p, --proxy <PROXY>
  -d, --dump-response                  Вывести HTTP-запрос в stdout для целевой отладки
  -j, --json <JSON_FILE>               Загрузить данные из файла JSON или из действительного онлайн-файла JSON
  -t, --timeout <TIMEOUT>              Время (в секундах) ожидания ответа на запросы [по умолчанию: 60]
      --print-all                      Вывести сайты, где имя пользователя не было найдено
      --print-found                    Вывести сайты, где имя пользователя было найдено
  -n, --no-color                       Не окрашивать вывод терминала
  -b, --browse                         Перейти ко всем результатам в браузере по умолчанию
  -l, --local                          Принудительно использовать локальный файл data.json
      --nsfw                           Включить проверку NSFW сайтов из списка по умолчанию
  -h, --help                           Вывести справку
  -V, --version                        Вывести версию
```

## Установка

```shell
cargo install sherlock
```

## Мотивация

Мне очень не нравится развертывать приложения на Python, и мне нужен был один бинарный файл. Go раздражает, Swift не имеет отличных инструментов на стороне сервера и в Linux, C++ даже не рассматривается, поэтому Rust — это то, что нужно.

<p align="center">
<img width="70%" height="70%" src="docs/images/reddit.png"/>
</a>
</p>

## Лицензия

MIT © Johannes Naylor<br/>
Оригинальный создатель Sherlock - [Siddharth Dushantha](https://github.com/sdushantha)

## Благодарности

- [Siddharth Dushantha](https://github.com/sdushantha) за создание оригинального Sherlock
- [Eira Fransham](https://github.com/eira-fransham) за предоставление имени крейта
