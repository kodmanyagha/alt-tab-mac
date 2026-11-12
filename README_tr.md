# alt-tab-mac

[English](README.md) | **Türkçe**

macOS için hızlı, **pencere tabanlı** geçiş uygulaması. Rust ile sıfırdan yazıldı.
Ücretsiz ve açık kaynak; her zaman öyle kalacak.

- Uygulamalar arasında değil, **pencereler** arasında geçiş yapar (Windows'taki Alt-Tab
  gibi). Aynı uygulamanın üç penceresi üç ayrı öğedir.
- **Sıfır gecikme:** Pencere listesi arka planda sürekli güncel tutulur. Kısayola
  basıldığında sisteme hiçbir şey sorulmaz; panel hazır listeden çizilir. 50 ms'lik hızlı
  basışlar bile çalışır, yavaş uygulamalar (PhpStorm, Electron…) paneli hiç bekletmez.
- **Hafif:** Boştayken işlemci ≈ %0, bellek ~15 MB.
- Intel ve Apple Silicon tek uygulamada, macOS 11 Big Sur ve sonrası.
- Türkçe ve İngilizce arayüz.

## Özellikler

- Birden çok kısayol; her birinin kendi kapsamı ve stili var. Varsayılanlar:

  | Kısayol | Gösterilen pencereler |
  |---------|-----------------------|
  | `⌘ Tab` | Tüm uygulamalar, tüm masaüstleri, tüm ekranlar |
  | `⌘ "` | Mevcut uygulama, tüm masaüstleri, tüm ekranlar |
  | `⌥ Tab` | Tüm uygulamalar, mevcut masaüstü, mevcut ekran |
  | `⌥ "` | Mevcut uygulama, mevcut masaüstü, mevcut ekran |

  `"` Tab'ın üstündeki tuştur (ABD klavyede `` ` ``). Tuşlar fiziksel konumlarına göre
  eşleşir, yani her klavye düzeninde çalışır. `⌘ Tab` ve `⌘ "` uygulama çalışırken
  sistemin kısayollarını devralır, uygulama kapanınca geri verir.
- **Stiller:** ikon, önizleme, köşesinde uygulama ikonu olan önizleme, liste.
- **Kısayol başına kapsam:** tüm uygulamalar ya da mevcut uygulama, tüm ya da mevcut
  masaüstü, tüm ya da mevcut ekran; küçültülmüş pencereler ve gizli uygulamalar
  gösterilebilir, gizlenebilir ya da sona konabilir.
- **Panel:** yatay ya da dikey yerleşim, tema (sistem / açık / koyu), öğe ve ikon boyutu,
  iç boşluk ve aralık, yazı tipi ve boyutu, 1–3 satır başlık, sıralama (son kullanılan,
  uygulama adı, uygulamalara göre gruplu); aktif pencerenin ekranında, farenin olduğu
  ekranda, ana ekranda, seçilen bir ekranda ya da tüm ekranlarda gösterim.
- **Pencere türleri:** diyaloglar, ana pencereyi kilitleyen diyaloglar ve yüzen paneller
  (Yazı Tipleri, Renkler…) ayrı ayrı gösterilebilir ya da gizlenebilir. Pencereye yapışık
  ekranlar (Kaydet, Yazdır) hiçbir zaman ayrı öğe olarak görünmez.
- Hiç gösterilmeyecek uygulamalar için **kara liste**.
- Panel açıkken: `Tab` / `⇧Tab` / ok tuşları ile gezinme, modifier'ı bırakınca ya da
  `Enter` ile geçiş, `Esc` ile iptal, `W` pencereyi kapat, `M` küçült, `Q` uygulamadan
  çık, `H` uygulamayı gizle (tüm tuşlar değiştirilebilir). İsteğe bağlı fareyle seçim.
- Açılışta başlatma, isteğe bağlı menü çubuğu simgesi. Simge gizliyken ayarları açmak
  için uygulamayı tekrar açın (Finder veya Spotlight).
- Kısayollar çalışamıyorsa (Erişilebilirlik izni geri alındı, bir şifre alanı odakta)
  sistemin `⌘ Tab`'ı geçici olarak geri verilir.

## Kurulum

**Homebrew** (kendi tap'inizden):
```sh
brew install --cask <kullanıcı>/tap/alt-tab-mac
```

**Elle:** [Releases](../../releases) sayfasından `.dmg` ya da `.zip` indirin, uygulamayı
Uygulamalar klasörüne taşıyın.

Uygulama Apple tarafından onaylanmamıştır (notarize edilmemiş). İlk açılışta macOS
engellerse Finder'da sağ tık → **Aç** deyin ya da şunu çalıştırın:
```sh
xattr -dr com.apple.quarantine /Applications/alt-tab-mac.app
```

### İzinler

- **Erişilebilirlik** (zorunlu): Sistem Ayarları → Gizlilik ve Güvenlik → Erişilebilirlik.
  İzin verildiği anda uygulama çalışmaya başlar.
- **Ekran Kaydı** (sadece önizleme stilleri için): Sistem Ayarları → Gizlilik ve
  Güvenlik → Ekran ve Sistem Ses Kaydı. İzni verdikten sonra uygulamayı yeniden başlatın.
  İzin yoksa önizleme yerine uygulama ikonu gösterilir.

## Ayarlar

Menü çubuğu simgesi → **Ayarlar…** (`⌘,`). Her değişiklik anında uygulanır ve
`~/Library/Application Support/alt-tab-mac/config.toml` dosyasına kaydedilir. Bu dosya
elle de düzenlenip menüden yeniden yüklenebilir.

## Loglar

Log dosyası `~/Library/Logs/alt-tab-mac.log` konumuna yazılır. Bu, macOS'un uygulama
logları için kullandığı klasördür; Konsol (Console.app) uygulaması da burayı listeler.

- Varsayılan olarak yalnızca **hatalar** yazılır. Seviye istenildiği zaman
  **Ayarlar → Genel → Loglar** bölümünden değiştirilebilir (yalnızca hatalar, uyarılar,
  bilgi, ayrıntılı, çok ayrıntılı); değişiklik yeniden başlatmadan hemen etkili olur.
- Dosyada yalnızca en son **2000 satır** tutulur; eski satırlar kendiliğinden silinir.
- Satırları arka plandaki bir thread yazar, böylece loglama geçiş panelini hiç yavaşlatmaz.
- **Sorun bildirirken:** seviyeyi *Ayrıntılı* yapın, sorunu tekrarlayın, sonra
  **Log dosyasını Finder'da göster** butonuna tıklayıp dosyayı bildiriminize ekleyin.

## Derleme

```sh
scripts/create-dev-cert.sh    # bir kez: yerel imza sertifikası (izinler her derlemede sıfırlanmasın)
scripts/bundle.sh             # universal .app → target/alt-tab-mac.app
scripts/bundle.sh --native    # sadece bu makinenin mimarisi (daha hızlı)
cargo test                    # birim testleri
```

Hata ayıklama:
- Log: `~/Library/Logs/alt-tab-mac.log`; `ALT_TAB_LOG=error|warn|info|debug|trace` ortam
  değişkeni ayarlardaki seviyeyi geçersiz kılar
- `alt-tab-mac --diagnose`: izinler, masaüstleri, pencere türleri, önizleme yakalama testi
- `alt-tab-mac --render-settings <klasör>` / `--render-panel <klasör>`: ayarlar
  sekmelerini ya da paneli ekranda göstermeden PNG olarak kaydeder (`ALT_TAB_LANG=tr|en`
  dili seçer)

## Sürüm yayınlama

1. `Cargo.toml`'daki `version` değerini artırın (ör. `0.2.0`) ve commit'leyin.
2. `git tag v0.2.0 && git push origin v0.2.0`
3. GitHub Actions (`.github/workflows/release.yml`) testleri çalıştırır; universal zip,
   dmg, `SHA256SUMS` ve doldurulmuş Homebrew cask dosyasını (`alt-tab-mac.rb`) yeni bir
   GitHub Release'e ekler.
4. `alt-tab-mac.rb` dosyasını tap deponuzun `Casks/` klasörüne kopyalayın.

Yerelde: `scripts/release.sh` ve `scripts/update-cask.sh <kullanıcı/depo>` → `dist/`.
Her push'ta `.github/workflows/ci.yml` biçim, clippy, test ve universal derleme kontrolü
yapar.

## Lisans

[GPL-3.0-or-later](LICENSE). Kapalı kaynak kullanım için ayrı ticari lisans gerekir;
proje sahibiyle iletişime geçin.

## Bağış

Alt-Tab Mac ücretsizdir ve her zaman ücretsiz kalacaktır. Ben açık kaynak ile başlayıp
sonradan ücretliye dönüştürülmesinden ve basit işler için ücret talep edilmesinden çok
rahatsız olan bir yazılım geliştirici olarak bu projeyi sizler için geliştirdim. Bununla
birlikte eğer bağış yapmak ve beni desteklemek isterseniz aşağıdaki kripto adreslerime
istediğiniz miktarda bağış yapabilirsiniz. Bundan ayrıca mutluluk duyacağım. İlerleyen
zamanlarda bağış yapan kişileri burada göstereceğim. Şimdiden teşekkürler.

| Ağ | Adres |
|----|-------|
| Bitcoin (BTC) | `bc1qnac4z5363m5ke937gqlkn69jfhcqqz9uv85drv` |
| Ethereum (ETH, USDT...) | `0xa3Ecccc8A958309519659f0d317A0B5eEEd43a26` |
| Solana (SOL) | `A6Q6aYPihLpvZhBvgXbFhxeXVTVb9T5t5jnciJEJ5RXB` |
| TRON (TRX, USDT...) | `TXt2esaLDc7rebiPCC2SUPaqrbao1NnGab` |
| Binance kullanıcı kimliği (UID) | `22770019` |
| Binance kullanıcı adı | `rustdev` |

Aynı adresler, kopyalama butonlarıyla birlikte, uygulamada **Ayarlar → Hakkında**
bölümünde de var.

---

<p align="center">❤️ ve 🦀 ile yapıldı — <b>Rust'ı seviyorum!</b> (I love Rust!)</p>
