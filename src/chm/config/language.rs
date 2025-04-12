macro_rules! define_langs {
    {
        $(
            [ $ident:ident, $hexcode:literal, $name:literal, $shortcode:literal ]
        ),+ $(,)?
    } => {
            /// The set of language codes accepted by the CHM compiler.  
            /// Each entry has a hex code, and a name ([`ChmLanguage::name`]).
            ///
            /// For the full list, see <https://www.w3.org/International/ms-lang.html>
            #[derive(Debug, Clone, Copy)]
            #[repr(u32)]
            pub enum ChmLanguage {
                $(
                    #[doc = concat!($name, "(", $shortcode, ")")]
                    $ident = $hexcode
                ),+
            }
            impl ChmLanguage {/// Returns the full name of the language
                #[must_use]
                pub fn name(&self) -> &'static str {
                    match self {
                        $(
                            Self::$ident => $name,
                        )+
                    }
                }

                /// Returns the language for the given shortcode. Case insensitive
                #[must_use]
                pub fn from_code(shortcode: &str) -> Option<Self> {
                    match shortcode.to_lowercase().as_str() {
                        $(
                            $shortcode => Some(Self::$ident),
                        )+

                        _ => None
                    }
                }
            }
            impl std::fmt::Display for ChmLanguage {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let code = *self as u32;
                    let name = self.name();
                    write!(f, "{code:0x} {name}")
                }
            }
            
        }
}

define_langs!{
    [Af, 0x436, "Afrikaans", "af"],
    [Sq, 0x41c, "Albanian", "sq"],
    [Ar, 0x1, "Arabic (Standard)", "ar"],
    [ArSa, 0x401, "Arabic (Saudi Arabia)", "ar-sa"],
    [ArIq, 0x801, "Arabic (Iraq)", "ar-iq"],
    [ArEg, 0x0c01, "Arabic (Egypt)", "ar-eg"],
    [ArLy, 0x1001, "Arabic (Libya)", "ar-ly"],
    [ArDz, 0x1401, "Arabic (Algeria)", "ar-dz"],
    [ArMa, 0x1801, "Arabic (Morocco)", "ar-ma"],
    [ArTn, 0x1c01, "Arabic (Tunisia)", "ar-tn"],
    [ArOm, 0x2001, "Arabic (Oman)", "ar-om"],
    [ArYe, 0x2401, "Arabic (Yemen)", "ar-ye"],
    [ArSy, 0x2801, "Arabic (Syria)", "ar-sy"],
    [ArJo, 0x2c01, "Arabic (Jordan)", "ar-jo"],
    [ArLb, 0x3001, "Arabic (Lebanon)", "ar-lb"],
    [ArKw, 0x3401, "Arabic (Kuwait)", "ar-kw"],
    [ArAe, 0x3801, "Arabic (U.A.E.)", "ar-ae"],
    [ArBh, 0x3c01, "Arabic (Bahrain)", "ar-bh"],
    [ArQa, 0x4001, "Arabic (Qatar)", "ar-qa"],
    [Eu, 0x42d, "Basque", "eu"],
    [Bg, 0x402, "Bulgarian", "bg"],
    [Be, 0x423, "Belarusian", "be"],
    [Ca, 0x403, "Catalan", "ca"],
    [Zh, 0x4, "Chinese", "zh"],
    [ZhTw, 0x404, "Chinese (Taiwan)", "zh-tw"],
    [ZhCn, 0x804, "Chinese (PRC)", "zh-cn"],
    [ZhHk, 0x0c04, "Chinese (Hong Kong)", "zh-hk"],
    [ZhSg, 0x1004, "Chinese (Singapore)", "zh-sg"],
    [Hr, 0x41a, "Croatian", "hr"],
    [Cs, 0x405, "Czech", "cs"],
    [Da, 0x406, "Danish", "da"],
    [Nl, 0x413, "Dutch (Standard)", "nl"],
    [NlBe, 0x813, "Dutch (Belgian)", "nl-be"],
    [En, 0x9, "English", "en"],
    [EnUs, 0x409, "English (United States)", "en-us"],
    [EnGb, 0x809, "English (British)", "en-gb"],
    [EnAu, 0x0c09, "English (Australian)", "en-au"],
    [EnCa, 0x1009, "English (Canadian)", "en-ca"],
    [EnNz, 0x1409, "English (New Zealand)", "en-nz"],
    [EnIe, 0x1809, "English (Ireland)", "en-ie"],
    [EnZa, 0x1c09, "English (South Africa)", "en-za"],
    [EnJm, 0x2009, "English (Jamaica)", "en-jm"],
    [EnBz, 0x2809, "English (Belize)", "en-bz"],
    [EnTt, 0x2c09, "English (Trinidad)", "en-tt"],
    [Et, 0x425, "Estonian", "et"],
    [Fo, 0x438, "Faeroese", "fo"],
    [Fa, 0x429, "Farsi", "fa"],
    [Fi, 0x40b, "Finnish", "fi"],
    [Fr, 0x40c, "French (Standard)", "fr"],
    [FrBe, 0x80c, "French (Belgian)", "fr-be"],
    [FrCa, 0x0c0c, "French (Canadian)", "fr-ca"],
    [FrCh, 0x100c, "French (Swiss)", "fr-ch"],
    [FrLu, 0x140c, "French (Luxembourg)", "fr-lu"],
    [Gd, 0x43c, "Gaelic (Scots)", "gd"],
    [De, 0x407, "German (Standard)", "de"],
    [DeCh, 0x807, "German (Swiss)", "de-ch"],
    [DeAt, 0x0c07, "German (Austrian)", "de-at"],
    [DeLu, 0x1007, "German (Luxembourg)", "de-lu"],
    [DeLi, 0x1407, "German (Liechtenstein)", "de-li"],
    [El, 0x408, "Greek", "el"],
    [He, 0x40d, "Hebrew", "he"],
    [Hi, 0x439, "Hindi", "hi"],
    [Hu, 0x40e, "Hungarian", "hu"],
    [Is, 0x40f, "Icelandic", "is"],
    [In, 0x421, "Indonesian", "id"],
    [It, 0x410, "Italian (Standard)", "it"],
    [ItCh, 0x810, "Italian (Swiss)", "it-ch"],
    [Ja, 0x411, "Japanese", "ja"],
    [Ko, 0x412, "Korean", "ko"],
    [Lv, 0x426, "Latvian", "lv"],
    [Lt, 0x427, "Lithuanian", "lt"],
    [Mk, 0x42f, "Macedonian", "mk"],
    [Ms, 0x43e, "Malaysian", "ms"],
    [Mt, 0x43a, "Maltese", "mt"],
    [No, 0x414, "Norwegian (Bokmal)", "nb"],
    [Pl, 0x415, "Polish", "pl"],
    [PtBr, 0x416, "Portuguese (Brazilian)", "pt-br"],
    [Pt, 0x816, "Portuguese (Standard)", "pt"],
    [Rm, 0x417, "Rhaeto-Romanic", "rm"],
    [Ro, 0x418, "Romanian", "ro"],
    [RoMo, 0x818, "Romanian (Moldavia)", "ro-mo"],
    [Ru, 0x419, "Russian", "ru"],
    [RuMo, 0x819, "Russian (Moldavia)", "ru-mo"],
    [Sr, 0x0c1a, "Serbian", "sr"],
    [Sk, 0x41b, "Slovak", "sk"],
    [Sl, 0x424, "Slovenian", "sl"],
    [Sb, 0x42e, "Sorbian", "sb"],
    [Es, 0x40a, "Spanish (Spain - Modern Sort)", "es"],
    [EsMx, 0x80a, "Spanish (Mexican)", "es-mx"],
    [EsGt, 0x100a, "Spanish (Guatemala)", "es-gt"],
    [EsCr, 0x140a, "Spanish (Costa Rica)", "es-cr"],
    [EsPa, 0x180a, "Spanish (Panama)", "es-pa"],
    [EsDo, 0x1c0a, "Spanish (Dominican Republic)", "es-do"],
    [EsVe, 0x200a, "Spanish (Venezuela)", "es-ve"],
    [EsCo, 0x240a, "Spanish (Colombia)", "es-co"],
    [EsPe, 0x280a, "Spanish (Peru)", "es-pe"],
    [EsAr, 0x2c0a, "Spanish (Argentina)", "es-ar"],
    [EsEc, 0x300a, "Spanish (Ecuador)", "es-ec"],
    [EsCl, 0x340a, "Spanish (Chile)", "es-cl"],
    [EsUy, 0x380a, "Spanish (Uruguay)", "es-uy"],
    [EsPy, 0x3c0a, "Spanish (Paraguay)", "es-py"],
    [EsBo, 0x400a, "Spanish (Bolivia)", "es-bo"],
    [EsSv, 0x440a, "Spanish (El Salvador)", "es-sv"],
    [EsHn, 0x480a, "Spanish (Honduras)", "es-hn"],
    [EsNi, 0x4c0a, "Spanish (Nicaragua)", "es-ni"],
    [EsPr, 0x500a, "Spanish (Puerto Rico)", "es-pr"],
    [Sx, 0x430, "Sutu", "sx"],
    [Sv, 0x41d, "Swedish", "sv"],
    [SvFi, 0x81d, "Swedish (Finland)", "sv-fi"],
    [Th, 0x41e, "Thai", "th"],
    [Ts, 0x431, "Tsonga", "ts"],
    [Tn, 0x432, "Tswana", "tn"],
    [Tr, 0x41f, "Turkish", "tr"],
    [Uk, 0x422, "Ukrainian", "uk"],
    [Ur, 0x420, "Urdu", "ur"],
    [Vi, 0x42a, "Vietnamese", "vi"],
    [Xh, 0x434, "Xhosa", "xh"],
    [Ji, 0x43d, "Yiddish", "ji"],
    [Zu, 0x435, "Zulu", "zu"],
}
impl Default for ChmLanguage {
    fn default() -> Self {
        Self::EnUs
    }
}