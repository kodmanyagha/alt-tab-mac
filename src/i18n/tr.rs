// SPDX-License-Identifier: GPL-3.0-or-later

use super::key::Key::{self, *};
use super::template::Template;

pub fn text(key: Key) -> &'static str {
    match key {
        MenuSettings => "Ayarlar…",
        MenuOpenConfig => "Ayar dosyasını aç",
        MenuReloadConfig => "Ayar dosyasını yeniden yükle",
        MenuQuit => "Çık",

        SettingsTitle => "alt-tab-mac Ayarları",
        TabGeneral => "Genel",
        TabShortcuts => "Kısayollar",
        TabAppearance => "Görünüm",
        TabWindows => "Pencereler",
        TabAbout => "Hakkında",

        Startup => "Başlangıç",
        LaunchAtLogin => "Oturum açılışında başlat",
        ShowStatusItem => "Menü çubuğunda simge göster",
        StatusItemNote => "Simge gizliyken ayarları açmak için uygulamayı tekrar açın (Finder veya Spotlight).",
        Mouse => "Fare",
        MouseSelection => "Panelde fareyle seçim",
        MouseSelectionNote => "Açıkken fareyle üzerine gelinen öğe seçilir, tıklanınca o pencereye geçilir.",
        Panel => "Panel",
        ShowDelay => "Panel gecikmesi",
        Immediately => "Hemen",
        ShowDelayNote => "Gecikme dolmadan kısayol bırakılırsa panel hiç görünmeden geçiş yapılır.",
        PanelScreen => "Panelin göründüğü ekran",
        ScreenFilter => "\"Mevcut ekran\" filtresi",
        ActiveWindowScreen => "Aktif pencerenin ekranı",
        CursorScreen => "Farenin olduğu ekran",
        PrimaryScreen => "Ana ekran",
        AllScreens => "Tüm ekranlar",
        Language => "Dil",
        LanguageSystem => "Sistem dili",
        Logs => "Loglar",
        LogLevel => "Log seviyesi",
        LogLevelError => "Yalnızca hatalar",
        LogLevelWarn => "Uyarılar ve hatalar",
        LogLevelInfo => "Bilgi",
        LogLevelDebug => "Ayrıntılı (hata ayıklama)",
        LogLevelTrace => "Çok ayrıntılı",
        LogsNote => {
            "Bir sorunu bildirirken seviyeyi \"Ayrıntılı\" yapın, sorunu tekrarlayın ve log dosyasını gönderin. Dosyada en son 2000 satır tutulur."
        }
        ShowLogFile => "Log dosyasını Finder'da göster",

        Theme => "Tema",
        ThemeSystem => "Sistem",
        ThemeLight => "Açık",
        ThemeDark => "Koyu",
        Orientation => "Yönelim",
        OrientationHorizontal => "Yatay (yan yana, başlık altta)",
        OrientationVertical => "Dikey (alt alta, başlık sağda)",
        Sort => "Sıralama",
        SortRecent => "Son kullanılan",
        SortAppName => "Uygulama adı",
        SortGrouped => "Uygulamalara göre gruplu",
        Items => "Öğeler",
        ItemSize => "Öğe boyutu",
        IconSize => "İkon boyutu",
        SizeSmall => "Küçük",
        SizeMedium => "Orta",
        SizeLarge => "Büyük",
        SizeExtraLarge => "Çok büyük",
        ItemPadding => "Öğe iç boşluğu",
        ItemSpacing => "Öğeler arası boşluk",
        StyleNote => "Görünüm stili (ikon / önizleme / liste) her kısayol için Kısayollar sekmesinden seçilir.",
        Text => "Yazı",
        FontFamily => "Yazı tipi",
        SystemFont => "Sistem yazı tipi",
        FontSize => "Yazı boyutu",
        TitleLines => "Başlık satırları",
        OneLine => "1 satır",
        TwoLines => "2 satır",
        ThreeLines => "3 satır",

        WindowKinds => "Listede gösterilecek pencere türleri",
        Dialogs => "Diyaloglar",
        DialogsNote => "\"Bilgi Ver\" gibi bağımsız küçük pencereler.",
        ModalDialogs => "Ana pencereyi kilitleyen diyaloglar",
        ModalDialogsNote => "Ör. PhpStorm Ayarlar. Kapalıyken ana pencereye geçmek diyaloğu da öne getirir.",
        FloatingPanels => "Yüzen paneller",
        FloatingPanelsNote => "Yazı Tipleri, Renkler, araç paletleri.",
        SheetsNote => "Pencereye yapışık ekranlar (Kaydet, Yazdır) hiçbir zaman ayrı öğe olarak görünmez.",
        Blacklist => "Kara liste",
        BlacklistNote => "Bu uygulamaların pencereleri hiç gösterilmez.",
        Remove => "Kaldır",
        AddApp => "Uygulama ekle",
        ChooseApp => "Uygulama seçin…",

        ShortcutsIntro => "Tuşları değiştirmek için \"Tuşlar\" düğmesine tıklayıp yeni kombinasyona basın.",
        Enabled => "Etkin",
        Delete => "Sil",
        Keys => "Tuşlar",
        Style => "Stil",
        StyleIcons => "İkon",
        StylePreviews => "Önizleme",
        StylePreviewsWithIcon => "Önizleme + köşede ikon",
        StyleList => "Liste",
        Apps => "Uygulamalar",
        AllApps => "Tüm uygulamalar",
        CurrentApp => "Mevcut uygulama",
        Spaces => "Masaüstleri",
        AllSpaces => "Tüm masaüstleri",
        CurrentSpace => "Mevcut masaüstü",
        Screens => "Ekranlar",
        CurrentScreen => "Mevcut ekran",
        MinimizedWindows => "Küçültülmüş pencereler",
        HiddenApps => "Gizlenmiş uygulamalar",
        Show => "Göster",
        ShowAtEnd => "Sona koy",
        Hide => "Gizle",
        AddShortcut => "+ Kısayol ekle",
        PanelKeys => "Panel açıkken tuşlar",
        CloseWindow => "Pencereyi kapat",
        Minimize => "Küçült",
        QuitApp => "Uygulamadan çık",
        HideApp => "Uygulamayı gizle",
        PressKeys => "Tuşlara basın… (Esc: iptal)",

        Donate => "Bağış",
        DonateNote => {
            "Alt-Tab Mac ücretsizdir ve her zaman ücretsiz kalacaktır. Ben açık kaynak ile başlayıp sonradan \
             ücretliye dönüştürülmesinden ve basit işler için ücret talep edilmesinden çok rahatsız olan bir \
             yazılım geliştirici olarak bu projeyi sizler için geliştirdim. Bununla birlikte eğer bağış yapmak ve \
             beni desteklemek isterseniz aşağıdaki kripto adreslerime istediğiniz miktarda bağış yapabilirsiniz. \
             Bundan ayrıca mutluluk duyacağım. İlerleyen zamanlarda bağış yapan kişileri burada göstereceğim. \
             Şimdiden teşekkürler."
        }
        Copy => "Kopyala",
        Copied => "Kopyalandı ✓",
        BinanceUserId => "Binance kullanıcı kimliği (UID)",
        BinanceUsername => "Binance kullanıcı adı",

        AppDescription => "macOS için hızlı, pencere tabanlı geçiş uygulaması.",
        License => "Lisans",
        LicenseText => {
            "GNU Genel Kamu Lisansı 3.0 (veya sonrası) ile dağıtılan özgür yazılımdır. \
                        Kapalı kaynak kullanım için ayrı ticari lisans gerekir."
        }
    }
}

pub fn template(template: Template, value: &str) -> String {
    match template {
        Template::ScreenName => format!("Ekran: {value}"),
        Template::ShortcutTitle => format!("Kısayol {value}"),
        Template::Version => format!("Sürüm {value}"),
        Template::ConfigFile => format!("Ayar dosyası: {value}"),
    }
}
