
<p align=center>
  <br>
  <a href="https://sherlock-project.github.io/" target="_blank"><img src="docs/images/sherlock-logo.png"/></a>
  <br>
  <span><a href="https://sherlockproject.xyz/sites">400'den fazla sosyal ağda</a> kullanıcı adına göre sosyal medya hesaplarını bulun</span>
  <br>
</p>

<p align="center">
<img width="70%" height="70%" src="docs/images/demo.png"/>
</a>
</p>


## Kurulum

```shell
cargo install sherlock
```

## Kullanım

```shell
sherlock user123
```
Birden fazla kullanıcı aramak için:
```shell
sherlock user1 user2 user3
```
Bulunan hesaplar, ilgili kullanıcı adıyla ayrı bir metin dosyasına kaydedilecektir (örneğin user123.txt).

```shell
$ sherlock --help

Kullanıcı adına göre sosyal medya hesaplarını bulun

Kullanım: sherlock-rs [OPTIONS] <usernames>...

Argümanlar:
  <usernames>...  Sosyal ağlarda kontrol edilecek bir veya daha fazla kullanıcı adı. Benzer kullanıcı adlarını {?} kullanarak kontrol edin (yerine '_', '-', '.' kullanın)

Seçenekler:
  -v, --verbose                        Ekstra hata ayıklama bilgileri ve metrikleri göster
  -o, --output <OUTPUT_FILE>           Sonuçları kaydetmek için çıktı dosyası
  -f, --output-folder <OUTPUT_FOLDER>  Tek kullanıcı adı kullanılıyorsa, sonucun çıktısı bu klasöre kaydedilecektir
  -c, --csv                            Virgülle Ayrılmış Değerler (CSV) Dosyası Oluştur
      --xlsx                           Modern Microsoft Excel elektronik tablosu için standart dosyayı (xlsx) oluştur
  -s, --site-list <SITE_LIST>          Analizi yalnızca listelenen sitelerle sınırla. Birden fazla site belirtmek için birden fazla seçenek ekleyin
  -p, --proxy <PROXY>
  -d, --dump-response                  Hedefli hata ayıklama için HTTP isteğini stdout'a dök
  -j, --json <JSON_FILE>               Verileri bir JSON dosyasından veya çevrimiçi, geçerli bir JSON dosyasından yükle
  -t, --timeout <TIMEOUT>              İsteklere yanıt beklemek için süre (saniye olarak) [varsayılan: 60]
      --print-all                      Kullanıcı adının bulunamadığı siteleri çıktıla
      --print-found                    Kullanıcı adının bulunduğu siteleri çıktıla
  -n, --no-color                       Terminal çıktısını renklendirme
  -b, --browse                         Varsayılan tarayıcıda tüm sonuçlara göz at
  -l, --local                          Yerel data.json dosyasının kullanımını zorla
      --nsfw                           Varsayılan listeden NSFW sitelerinin kontrolünü dahil et
  -h, --help                           Yardımı yazdır
  -V, --version                        Sürümü yazdır
```

## Kurulum

```shell
cargo install sherlock
```

## Motivasyon

Python uygulamalarını dağıtmaktan hiç hoşlanmıyorum ve tek bir ikili dosya istiyordum. Go beni sinir ediyor, Swift'in sunucu tarafında ve Linux'ta harika araçları yok, C++ ise söz konusu bile değil, bu yüzden Rust tercih edildi.

<p align="center">
<img width="70%" height="70%" src="docs/images/reddit.png"/>
</a>
</p>

## Lisans

MIT © Johannes Naylor<br/>
Sherlock'un Orijinal Yaratıcısı - [Siddharth Dushantha](https://github.com/sdushantha)

## Teşekkürler

- Orijinal Sherlock'u yarattığı için [Siddharth Dushantha](https://github.com/sdushantha)'ya
- Crate adını verdiği için [Eira Fransham](https://github.com/eira-fransham)'a
