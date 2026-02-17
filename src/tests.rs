use crate::Guess;

fn format(a: &str) -> String {
    Guess::new(a).to_string()
}

#[test]
fn win() {
    // Edge on Windows 11
    let edge = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0";
    assert_eq!(&format(edge), "Windows Chrome/144.0");

    // Firefox
    let firefox = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:147.0) Gecko/20100101 Firefox/147.0";
    assert_eq!(&format(firefox), "Windows Firefox/147.0");
}

#[test]
fn linux() {
    // Chromium on Arch Linux
    let chromium = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36";
    assert_eq!(&format(chromium), "Linux Chrome/144.0");

    // Firefox
    let firefox = "Mozilla/5.0 (X11; Linux x86_64; rv:147.0) Gecko/20100101 Firefox/147.0";
    assert_eq!(&format(firefox), "Linux Firefox/147.0");
}

#[test]
fn mac() {
    // Safari
    let safari = "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_4) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15";
    assert_eq!(&format(safari), "Mac Safari/26.0");

    // Firefox
    let firefox = "Mozilla/5.0 (Macintosh; Intel Mac OS X 15.7; rv:147.0) Gecko/20100101 Firefox/147.0";
    assert_eq!(&format(firefox), "Mac Firefox/147.0");

    let chromes = &[
        // Chrome
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36",
        // Vivaldi
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Vivaldi/7.8.3925.66",
        // Edge
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/145.0.3800.58",
    ];

    for chrome in chromes {
        assert_eq!(&format(chrome), "Mac Chrome/144.0");
    }
}

#[test]
fn android() {
    // BTW, brand info is available in Sec-CH-UA header - sent only over https
    let chromes = &[
        // DeepSeek app
        "Mozilla/5.0 (Linux; Android 16; M2007J17G Build/BP2A.250805.005; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/142.0.7444.138 Mobile Safari/537.36",
        // Mulch
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
        // Edge
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36 EdgA/142.0.0.0",
        // Brave
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Mobile Safari/537.36",
        // Vivaldi
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36",
        // Via
        "Mozilla/5.0 (Linux; Android 16; M2007J17G Build/BP2A.250805.005) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.7444.138 Mobile Safari/537.36",
        // Opera
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36 OPR/93.0.0.0",
        // Opera mini
        "Mozilla/5.0 (Linux; U; Android 16; M2007J17G Build/BP2A.250805.005; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/142.0.7444.138 Mobile Safari/537.36 OPR/96.1.2254.80015",
        // Yandex Start
        "Mozilla/5.0 (Linux; arm_64; Android 16; M2007J17G) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.7339.698 YaApp_Android/25.104.1 YaSearchBrowser/25.104.1 BroPP/1.0 SA/3 Mobile Safari/537.36",
        // Miui Browser
        "Mozilla/5.0 (Linux; Android 16; M2007J17G Build/BP2A.250805.005) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.6312.118 Mobile Safari/537.36 XiaoMi/MiuiBrowser/14.47.1-gn",
        // Yandex Browser
        "Mozilla/5.0 (Linux; arm_64; Android 16; M2007J17G) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.7339.40 YaBrowser/25.10.7.40.00 SA/3 Mobile Safari/537.36",
        // Yandex Lite
        "Mozilla/5.0 (Linux; Android 16; M2007J17G Build/BP2A.250805.005; wv) AppleWebKit/537.36 (KHTML, like Gecko)  Chrome/142.0.7444.138 YaBrowser/25.12.0.63 (lite) Mobile Safari/537.36",
        // Cromite
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Mobile Safari/537.36",
        // Telegram
        "Mozilla/5.0 (Linux; Android 13; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.7499.34 Mobile Safari/537.36",
    ];

    let expected_vers = &[
        "Android Chrome/142.0", // DeepSeek app
        "Android Chrome/131.0", // Mulch
        "Android Chrome/142.0", // Edge
        "Android Chrome/143.0", // Brave
        "Android Chrome/142.0", // Vivaldi
        "Android Chrome/142.0", // Via
        "Android Chrome/142.0", // Opera
        "Android Chrome/142.0", // Opera mini
        "Android Chrome/140.0", // Yandex Start
        "Android Chrome/123.0", // Miui Browser
        "Android Chrome/140.0", // Yandex Browser
        "Android Chrome/142.0", // Yandex Lite
        "Android Chrome/143.0", // Cromite
        "Android Chrome/143.0", // Telegram
    ];

    for (chrome, ver) in chromes.iter().zip(expected_vers.iter()) {
        assert_eq!(&format(chrome), ver);
    }

    // Firefox/Fennec
    let fox = "Mozilla/5.0 (Android 16; Mobile; rv:146.0) Gecko/146.0 Firefox/146.0";
    assert_eq!(&format(fox), "Android Firefox/146.0");
}

#[test]
fn ios() {
    let iphones = &[
        // Safari
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Mobile/15E148 Safari/604.1",
        // Chrome
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/145.0.7632.55 Mobile/15E148 Safari/604.1",
        // Firefox
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/147.0 Mobile/15E148 Safari/605.1.15",
        // iPad Safari
        "Mozilla/5.0 (iPad; CPU OS 9_3_5 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13G36 Safari/601.1",
        // iPhone Safari
        "Mozilla/5.0 (iPhone; CPU iPhone OS 7_1_2 like Mac OS X) AppleWebKit/537.51.2 (KHTML, like Gecko) Version/7.0 Mobile/11D257 Safari/9537.53",
    ];

    let expected_vers = &[
        "iOS Safari/26.0",
        "iOS Chrome/145.0",
        "iOS Firefox/147.0",
        "iOS Safari/9.0",
        "iOS Safari/7.0",
    ];

    for (iphone, ver) in iphones.iter().zip(expected_vers.iter()) {
        assert_eq!(&format(iphone), ver);
    }
}

#[test]
fn unknown() {
    let unknowns = &[
        "Mozilla/5.0",
        "curl/8.14.1",
        "Wget/1.25.0",
        "VLC/3.0.21 LibVLC/3.0.21",
        "python-requests/2.31.0",
        "ELinks/0.18.0 (textmode; Linux 4.19.127-g657526e18720-dirty armv8l; 70x45-2)",
    ];

    let expected_vers = &[
        "Unknown Mozilla/5.0",
        "Unknown curl/8.14.1",
        "Unknown Wget/1.25.0",
        "Unknown VLC/3.0.21",
        "Unknown python-requests/2.31.0",
        "Linux ELinks/0.18.0",
    ];

    for (unknown, ver) in unknowns.iter().zip(expected_vers.iter()) {
        assert_eq!(&format(unknown), ver);
    }
}

#[test]
fn dots() {
    let _ = format("Chrome/");
    let _ = format("Chrome/141");
    let _ = format("Chrome/141.");
    let _ = format("Chrome/141.0");
    let _ = format("Chrome/141.0.");
    let _ = format("Chrome/141.0.0");
    let _ = format("Chrome/141.0.0.");
    let _ = format("Chrome/141.0.0.0");
}
