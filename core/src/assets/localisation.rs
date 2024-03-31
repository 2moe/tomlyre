// Version: 0.0.1-beta.2
#![allow(dead_code)]

use super::{lang_id_consts, HashMap, LangID};

pub(crate) type L10nMap = ::phf::Map<&'static str, &'static str>;
pub(crate) type SubLocaleMap = ::phf::Map<&'static str, fn() -> L10nMap>;
pub(crate) type LocaleMap = ::phf::Map<&'static str, fn() -> SubLocaleMap>;
pub(crate) type LocaleHashMap = HashMap<LangID, SubLocaleMap>;

/// Language ID: af;
/// Map name: "conversion";
/// Description: Afrikaans, Latyn, Suid-Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Kon nie die formaat outomaties opspoor nie.Spesifiseer asseblief handmatig.");
/// ```
pub(super) const fn get_af_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Die lêer is nie 'n geldige 'JSON 1.0' -formaat nie, en probeer om as 'JSON5' te ontleed ..."##),
        ("not-included", r##"Hierdie binêre ** sluit nie ** die omskakelingsfunksie vir die betrokke formaat in nie.
U moet die betrokke funksie in u 'vrag.toml' in staat stel en weer saamstaan.
As hierdie sagteware nie die ooreenstemmende funksionaliteit insluit nie, moet u 'n probleem indien."##),
        ("currently-supported", r##"Tans ondersteunde Formate -lys:"##),
        ("unsupported", r##"Omskakeling van onondersteunde formaat"##),
        ("auto-detection-failed", r##"Kon nie die formaat outomaties opspoor nie.Spesifiseer asseblief handmatig."##),
        ("not-support-deser-sexp", r##"** Nog nie ondersteun nie **: omskakeling van 'lisp s-expression' na 'ander formate'"##),
        ("unknown-fmt", r##"Onbekende lêerformaat"##),
        ("not-saved", r##"Die volgende inhoud ** sal nie ** gestoor word nie, omdat `--save` nie geroep is nie."##),
        ("dst", r##"bestemmingslêerpad"##),
        ("conv-error", r##"Omskakelingsfout"##),
    ],
}
}

/// Language ID: af;
/// Map name: "conversion_md";
/// Description: Afrikaans, Latyn, Suid-Afrika;
pub(super) const fn get_af_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mDie lêer is nie 'n geldige 'JSON 1.0' -formaat nie, en probeer om as 'JSON5' te ontleed ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mHierdie binêre ** sluit nie ** die omskakelingsfunksie vir die betrokke formaat in nie.
[48;2;34;34;34m[38;2;255;255;255mU moet die betrokke funksie in u 'vrag.toml' in staat stel en weer saamstaan.
[48;2;34;34;34m[38;2;255;255;255mAs hierdie sagteware nie die ooreenstemmende funksionaliteit insluit nie, moet u 'n probleem indien.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mTans ondersteunde Formate -lys:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mOmskakeling van onondersteunde formaat[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mKon nie die formaat outomaties opspoor nie.Spesifiseer asseblief handmatig.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Nog nie ondersteun nie [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: omskakeling van 'lisp s-expression' na 'ander formate'[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mOnbekende lêerformaat[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mDie volgende inhoud ** sal nie ** gestoor word nie, omdat [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nie geroep is nie.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mbestemmingslêerpad[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mOmskakelingsfout[0m"##),
    ],
}
}

/// Language ID: af;
/// Map name: "set";
/// Description: Afrikaans, Latyn, Suid-Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ongeldige lêerpad.");
/// ```
pub(super) const fn get_af_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Die gewysigde inhoud ** sal nie ** gestoor word nie omdat `--sv` nie genoem is nie."##),
        ("new-value", r##"Nuwe waarde"##),
        ("invalid-path", r##"Ongeldige lêerpad."##),
    ],
}
}

/// Language ID: af;
/// Map name: "set_md";
/// Description: Afrikaans, Latyn, Suid-Afrika;
pub(super) const fn get_af_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDie gewysigde inhoud ** sal nie ** gestoor word nie omdat [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nie genoem is nie.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNuwe waarde[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mOngeldige lêerpad.[0m"##),
    ],
}
}

/// Language ID: af;
/// Map name: "get";
/// Description: Afrikaans, Latyn, Suid-Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Bestemmingsformaat");
/// ```
pub(super) const fn get_af_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Bestemmingsformaat"##),
        ("src-fmt", r##"Bronlêerformaat"##),
    ],
}
}

/// af: Afrikaans, Latyn, Suid-Afrika
pub(super) const fn get_af_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_af_map_conversion),
        ("get", get_af_map_get),
        ("set_md", get_af_map_set_md),
        ("set", get_af_map_set),
        ("conversion_md", get_af_map_conversion_md),
    ],
}
}

/// Language ID: am;
/// Map name: "conversion";
/// Description: አማርኛ, ኢትዮፒክ, ኢትዮጵያ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ቅርጸት በራስ-ሰር ለመምረጥ አልተሳካም.እባክዎ እራስዎ ይጥቀሱ.");
/// ```
pub(super) const fn get_am_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"የሚከተለው ይዘት ** አይሰጥም *** አይቀመጥም *** መቀመጥ የለበትም."##),
        ("unknown-fmt", r##"ያልታወቀ ፋይል ቅርጸት"##),
        ("currently-supported", r##"በአሁኑ ጊዜ የተደገፉ ቅርፀቶች ዝርዝር:"##),
        ("auto-detection-failed", r##"ቅርጸት በራስ-ሰር ለመምረጥ አልተሳካም.እባክዎ እራስዎ ይጥቀሱ."##),
        ("not-included", r##"ይህ ሁለትዮሽ ** ተገቢውን ቅርጸት የውይይት ተግባሩን አያካትትም.
ተገቢውን ባህሪ በ `ኘካሪ.` `ዎ ውስጥ ባለው የጭነት መኪናዎ ውስጥ ማንቃት ያስፈልግዎታል.
ይህ ሶፍትዌር ተጓዳኝ ተግባሩን የማያካትት ከሆነ እባክዎን አንድ ጉዳይ ያስገቡ."##),
        ("conv-error", r##"የውድር ስህተት"##),
        ("dst", r##"የመድረሻ ፋይል መንገድ"##),
        ("unsupported", r##"የማይደገፍ ቅርጸት ልወጣ"##),
        ("invalid-json1.0", r##"ፋይሉ ትክክለኛ jdson 1.0 ቅርጸት, እንደ `json5` ወደታች ለመሞከር በመሞከር ላይ ..."##),
    ],
}
}

/// Language ID: am;
/// Map name: "conversion_md";
/// Description: አማርኛ, ኢትዮፒክ, ኢትዮጵያ;
pub(super) const fn get_am_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mየሚከተለው ይዘት ** አይሰጥም *** አይቀመጥም *** መቀመጥ የለበትም.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mያልታወቀ ፋይል ቅርጸት[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mበአሁኑ ጊዜ የተደገፉ ቅርፀቶች ዝርዝር:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mቅርጸት በራስ-ሰር ለመምረጥ አልተሳካም.እባክዎ እራስዎ ይጥቀሱ.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mይህ ሁለትዮሽ ** ተገቢውን ቅርጸት የውይይት ተግባሩን አያካትትም.
[48;2;34;34;34m[38;2;255;255;255mተገቢውን ባህሪ በ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mኘካሪ.[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mዎ ውስጥ ባለው የጭነት መኪናዎ ውስጥ ማንቃት ያስፈልግዎታል.
[48;2;34;34;34m[38;2;0;255;255mይህ ሶፍትዌር ተጓዳኝ ተግባሩን የማያካትት ከሆነ እባክዎን አንድ ጉዳይ ያስገቡ.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mየውድር ስህተት[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mየመድረሻ ፋይል መንገድ[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mየማይደገፍ ቅርጸት ልወጣ[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mፋይሉ ትክክለኛ jdson 1.0 ቅርጸት, እንደ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ወደታች ለመሞከር በመሞከር ላይ ...[0m"##),
    ],
}
}

/// Language ID: am;
/// Map name: "set";
/// Description: አማርኛ, ኢትዮፒክ, ኢትዮጵያ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ልክ ያልሆነ የፋይል ዱካ.");
/// ```
pub(super) const fn get_am_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("invalid-path", r##"ልክ ያልሆነ የፋይል ዱካ."##),
        ("new-value", r##"አዲስ እሴት"##),
    ],
}
}

/// Language ID: am;
/// Map name: "set_md";
/// Description: አማርኛ, ኢትዮፒክ, ኢትዮጵያ;
pub(super) const fn get_am_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mልክ ያልሆነ የፋይል ዱካ.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mአዲስ እሴት[0m"##),
    ],
}
}

/// Language ID: am;
/// Map name: "get";
/// Description: አማርኛ, ኢትዮፒክ, ኢትዮጵያ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "የመድረሻ ቅርጸት");
/// ```
pub(super) const fn get_am_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"የመድረሻ ቅርጸት"##),
        ("src-fmt", r##"ምንጭ ፋይል ቅርጸት"##),
    ],
}
}

/// am: አማርኛ, ኢትዮፒክ, ኢትዮጵያ
pub(super) const fn get_am_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_am_map_conversion),
        ("get", get_am_map_get),
        ("set_md", get_am_map_set_md),
        ("set", get_am_map_set),
        ("conversion_md", get_am_map_conversion_md),
    ],
}
}

/// Language ID: ar;
/// Map name: "conversion";
/// Description: العربية, العربية, مصر;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "فشل الكشف التلقائي عن التنسيق. يرجى تحديده يدوي\u{64b}ا.");
/// ```
pub(super) const fn get_ar_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"الملف ليس بتنسيق `json 1.0` صالح ، جاري محاولة التحويل إلى `json5` ..."##),
        ("not-included", r##"لا يتضمن هذا البرنامج الثنائي **عملية التحويل** للتنسيق ذي الصلة.
يجب تمكين الميزة المناسبة في ملف `Cargo.toml` الخاص بك وإعادة الترجمة.
إذا لم يتضمن هذا البرنامج الثنائي الوظائف المناسبة، يرجى إرسال طلب في القائمة."##),
        ("currently-supported", r##"قائمة التنسيقات المدعومة حاليًّا:"##),
        ("unsupported", r##"تحويل التنسيق غير المدعوم"##),
        ("auto-detection-failed", r##"فشل الكشف التلقائي عن التنسيق. يرجى تحديده يدويًا."##),
        ("not-support-deser-sexp", r##"**غير مدعوم حاليًا**: التحويل من تنسيق `Lisp S-Expression` إلى `تنسيقات أخرى`"##),
        ("unknown-fmt", r##"تنسيق الملف غير معروف"##),
        ("not-saved", r##"لن يتم حفظ المحتوى التالي **لأن `--save` لم يتم استدعاؤه**."##),
        ("dst", r##"مسار ملف الوجهة"##),
        ("conv-error", r##"خطأ في التحويل"##),
    ],
}
}

/// Language ID: ar;
/// Map name: "conversion_md";
/// Description: العربية, العربية, مصر;
pub(super) const fn get_ar_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mالملف ليس بتنسيق [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m صالح ، جاري محاولة التحويل إلى [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mلا يتضمن هذا البرنامج الثنائي [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mعملية التحويل[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m للتنسيق ذي الصلة.
[48;2;34;34;34m[38;2;255;255;255mيجب تمكين الميزة المناسبة في ملف [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m الخاص بك وإعادة الترجمة.
[48;2;34;34;34m[38;2;255;255;255mإذا لم يتضمن هذا البرنامج الثنائي الوظائف المناسبة، يرجى إرسال طلب في القائمة.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mقائمة التنسيقات المدعومة حاليًّا:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mتحويل التنسيق غير المدعوم[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mفشل الكشف التلقائي عن التنسيق. يرجى تحديده يدويًا.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mغير مدعوم حاليًا[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: التحويل من تنسيق [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m إلى [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mتنسيقات أخرى[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mتنسيق الملف غير معروف[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mلن يتم حفظ المحتوى التالي [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mلأن [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m لم يتم استدعاؤه[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mمسار ملف الوجهة[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mخطأ في التحويل[0m"##),
    ],
}
}

/// Language ID: ar;
/// Map name: "set";
/// Description: العربية, العربية, مصر;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "مسار الملف غير صالح.");
/// ```
pub(super) const fn get_ar_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"لن يتم حفظ المحتوى المعدل **لأن `--sv` لم يتم استدعاؤه**."##),
        ("new-value", r##"قيمة جديدة"##),
        ("invalid-path", r##"مسار الملف غير صالح."##),
    ],
}
}

/// Language ID: ar;
/// Map name: "set_md";
/// Description: العربية, العربية, مصر;
pub(super) const fn get_ar_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mلن يتم حفظ المحتوى المعدل [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mلأن [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m لم يتم استدعاؤه[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mقيمة جديدة[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mمسار الملف غير صالح.[0m"##),
    ],
}
}

/// Language ID: ar;
/// Map name: "get";
/// Description: العربية, العربية, مصر;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "تنسيق الوجهة");
/// ```
pub(super) const fn get_ar_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"تنسيق الوجهة"##),
        ("src-fmt", r##"تنسيق الملف المصدر"##),
    ],
}
}

/// ar: العربية, العربية, مصر
pub(super) const fn get_ar_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ar_map_conversion),
        ("get", get_ar_map_get),
        ("set_md", get_ar_map_set_md),
        ("set", get_ar_map_set),
        ("conversion_md", get_ar_map_conversion_md),
    ],
}
}

/// Language ID: az;
/// Map name: "conversion";
/// Description: azərbaycan, latın, Azərbaycan;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Formatı avtomatik aşkar edə bilmədi.Zəhmət olmasa əl ilə göstərin.");
/// ```
pub(super) const fn get_az_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Fayl, `JSON5` kimi təhlil etməyə çalışaraq, etibarlı bir `JSON 1.0` formatı deyil ..."##),
        ("not-included", r##"Bu ikili ** müvafiq format üçün dönüşüm funksionallığı daxil deyil.
'Cargo.toml' da müvafiq xüsusiyyətə imkan verməlisiniz və yenidən tərtib etməlisiniz.
Bu proqramın müvafiq funksionallıq daxil deyilsə, xahiş edirəm bir məsələ təqdim edin."##),
        ("currently-supported", r##"Hal-hazırda dəstəklənən formatlar siyahısı:"##),
        ("unsupported", r##"Dəstəklənməyən format dönüşüm"##),
        ("auto-detection-failed", r##"Formatı avtomatik aşkar edə bilmədi.Zəhmət olmasa əl ilə göstərin."##),
        ("not-support-deser-sexp", r##"** Dəstəklənməyib **: `Lisp S-ifadəsindən 'digər formatlara'"##),
        ("unknown-fmt", r##"Naməlum fayl formatı"##),
        ("not-saved", r##"Aşağıdakı məzmun qənaət olunmayacaq, çünki `--save` deyildi."##),
        ("dst", r##"Təyinat faylı yolu"##),
        ("conv-error", r##"Dönüşüm xətası"##),
    ],
}
}

/// Language ID: az;
/// Map name: "conversion_md";
/// Description: azərbaycan, latın, Azərbaycan;
pub(super) const fn get_az_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFayl, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m kimi təhlil etməyə çalışaraq, etibarlı bir [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m formatı deyil ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBu ikili ** müvafiq format üçün dönüşüm funksionallığı daxil deyil.
[48;2;34;34;34m[38;2;255;255;255m'Cargo.toml' da müvafiq xüsusiyyətə imkan verməlisiniz və yenidən tərtib etməlisiniz.
[48;2;34;34;34m[38;2;255;255;255mBu proqramın müvafiq funksionallıq daxil deyilsə, xahiş edirəm bir məsələ təqdim edin.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mHal-hazırda dəstəklənən formatlar siyahısı:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mDəstəklənməyən format dönüşüm[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mFormatı avtomatik aşkar edə bilmədi.Zəhmət olmasa əl ilə göstərin.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Dəstəklənməyib [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-ifadəsindən 'digər formatlara'[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNaməlum fayl formatı[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mAşağıdakı məzmun qənaət olunmayacaq, çünki [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m deyildi.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mTəyinat faylı yolu[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mDönüşüm xətası[0m"##),
    ],
}
}

/// Language ID: az;
/// Map name: "set";
/// Description: azərbaycan, latın, Azərbaycan;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Yanlış fayl yolu.");
/// ```
pub(super) const fn get_az_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Dəyişdirilmiş məzmun ** ** qənaət olunmayacaq, çünki '-' adlandırılmadı."##),
        ("new-value", r##"Yeni dəyər"##),
        ("invalid-path", r##"Yanlış fayl yolu."##),
    ],
}
}

/// Language ID: az;
/// Map name: "set_md";
/// Description: azərbaycan, latın, Azərbaycan;
pub(super) const fn get_az_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDəyişdirilmiş məzmun ** ** qənaət olunmayacaq, çünki '-' adlandırılmadı.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mYeni dəyər[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mYanlış fayl yolu.[0m"##),
    ],
}
}

/// Language ID: az;
/// Map name: "get";
/// Description: azərbaycan, latın, Azərbaycan;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Təyinat formatı");
/// ```
pub(super) const fn get_az_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Təyinat formatı"##),
        ("src-fmt", r##"Mənbə faylı formatı"##),
    ],
}
}

/// az: azərbaycan, latın, Azərbaycan
pub(super) const fn get_az_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_az_map_conversion),
        ("get", get_az_map_get),
        ("set_md", get_az_map_set_md),
        ("set", get_az_map_set),
        ("conversion_md", get_az_map_conversion_md),
    ],
}
}

/// Language ID: be;
/// Map name: "conversion";
/// Description: беларуская, кірыліца, Беларусь;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Не атрымалася аўтаматычна выявіць фармат.Калі ласка, укажыце ўручную.");
/// ```
pub(super) const fn get_be_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Файл не з'яўляецца сапраўдным фарматам `json 1.0`, спрабуючы разбіваць як `json5` ..."##),
        ("not-included", r##"Гэты двайковы ** не ўключае ** функцыянальнасць пераўтварэння для адпаведнага фармату.
Вам неабходна ўключыць адпаведную функцыю ў вашым `chargo.toml` і Recompile.
Калі гэта праграмнае забеспячэнне не ўключае адпаведную функцыянальнасць, калі ласка, адпраўце праблему."##),
        ("currently-supported", r##"У цяперашні час падтрымліваецца спіс фарматаў:"##),
        ("unsupported", r##"Перакладзенае пераўтварэнне фармату"##),
        ("auto-detection-failed", r##"Не атрымалася аўтаматычна выявіць фармат.Калі ласка, укажыце ўручную."##),
        ("not-support-deser-sexp", r##"** Пакуль яшчэ не падтрымліваецца **: пераўтварэнне з `Lisp S-Expression` да` іншых фарматаў '"##),
        ("unknown-fmt", r##"Невядомы фармат файла"##),
        ("not-saved", r##"Наступны змест ** не будзе захаваны **, таму што `--save` не быў выкліканы."##),
        ("dst", r##"шлях файла прызначэння"##),
        ("conv-error", r##"Памылка пераўтварэння"##),
    ],
}
}

/// Language ID: be;
/// Map name: "conversion_md";
/// Description: беларуская, кірыліца, Беларусь;
pub(super) const fn get_be_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mФайл не з'яўляецца сапраўдным фарматам [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, спрабуючы разбіваць як [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mГэты двайковы ** не ўключае ** функцыянальнасць пераўтварэння для адпаведнага фармату.
[48;2;34;34;34m[38;2;255;255;255mВам неабходна ўключыць адпаведную функцыю ў вашым [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mchargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m і Recompile.
[48;2;34;34;34m[38;2;255;255;255mКалі гэта праграмнае забеспячэнне не ўключае адпаведную функцыянальнасць, калі ласка, адпраўце праблему.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mУ цяперашні час падтрымліваецца спіс фарматаў:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mПеракладзенае пераўтварэнне фармату[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mНе атрымалася аўтаматычна выявіць фармат.Калі ласка, укажыце ўручную.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Пакуль яшчэ не падтрымліваецца [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: пераўтварэнне з [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m да[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m іншых фарматаў '[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mНевядомы фармат файла[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mНаступны змест ** не будзе захаваны [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m, таму што [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m не быў выкліканы.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mшлях файла прызначэння[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mПамылка пераўтварэння[0m"##),
    ],
}
}

/// Language ID: be;
/// Map name: "set";
/// Description: беларуская, кірыліца, Беларусь;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Несапраўдны шлях файла.");
/// ```
pub(super) const fn get_be_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Зменены змест ** не будзе захаваны, таму што `--sv` не быў выкліканы."##),
        ("new-value", r##"Новае значэнне"##),
        ("invalid-path", r##"Несапраўдны шлях файла."##),
    ],
}
}

/// Language ID: be;
/// Map name: "set_md";
/// Description: беларуская, кірыліца, Беларусь;
pub(super) const fn get_be_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mЗменены змест ** не будзе захаваны, таму што [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m не быў выкліканы.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mНовае значэнне[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mНесапраўдны шлях файла.[0m"##),
    ],
}
}

/// Language ID: be;
/// Map name: "get";
/// Description: беларуская, кірыліца, Беларусь;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Фармат прызначэння");
/// ```
pub(super) const fn get_be_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Фармат прызначэння"##),
        ("src-fmt", r##"Фармат зыходнага файла"##),
    ],
}
}

/// be: беларуская, кірыліца, Беларусь
pub(super) const fn get_be_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_be_map_conversion),
        ("get", get_be_map_get),
        ("set_md", get_be_map_set_md),
        ("set", get_be_map_set),
        ("conversion_md", get_be_map_conversion_md),
    ],
}
}

/// Language ID: bg;
/// Map name: "conversion";
/// Description: български, кирилица, България;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Неуспешно откриване на автоматично формата.Моля, посочете ръчно.");
/// ```
pub(super) const fn get_bg_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Файлът не е валиден формат `json 1.0`, опитвайки се да анализира като `json5` ..."##),
        ("not-included", r##"Този двоичен ** не включва ** функционалността за преобразуване за съответния формат.
Трябва да активирате съответната функция във вашия `Cargo.toml` и да се прекомпилирате.
Ако този софтуер не включва съответната функционалност, моля, изпратете проблем."##),
        ("currently-supported", r##"Списък с поддръжка на формати в момента:"##),
        ("unsupported", r##"Непредвечено преобразуване на формат"##),
        ("auto-detection-failed", r##"Неуспешно откриване на автоматично формата.Моля, посочете ръчно."##),
        ("not-support-deser-sexp", r##"** Все още не е поддържан **: Конвертиране от `Lisp S-Expression` в` други формати`"##),
        ("unknown-fmt", r##"Неизвестен формат на файла"##),
        ("not-saved", r##"Следното съдържание ** няма да бъде запазено, защото `--save` не е извикано."##),
        ("dst", r##"Пътят на файла на дестинацията"##),
        ("conv-error", r##"Грешка за преобразуване"##),
    ],
}
}

/// Language ID: bg;
/// Map name: "conversion_md";
/// Description: български, кирилица, България;
pub(super) const fn get_bg_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mФайлът не е валиден формат [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, опитвайки се да анализира като [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mТози двоичен ** не включва ** функционалността за преобразуване за съответния формат.
[48;2;34;34;34m[38;2;255;255;255mТрябва да активирате съответната функция във вашия [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m и да се прекомпилирате.
[48;2;34;34;34m[38;2;255;255;255mАко този софтуер не включва съответната функционалност, моля, изпратете проблем.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mСписък с поддръжка на формати в момента:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mНепредвечено преобразуване на формат[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mНеуспешно откриване на автоматично формата.Моля, посочете ръчно.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Все още не е поддържан [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Конвертиране от [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m в[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m други формати[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mНеизвестен формат на файла[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mСледното съдържание ** няма да бъде запазено, защото [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m не е извикано.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mПътят на файла на дестинацията[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mГрешка за преобразуване[0m"##),
    ],
}
}

/// Language ID: bg;
/// Map name: "set";
/// Description: български, кирилица, България;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Невалиден път на файла.");
/// ```
pub(super) const fn get_bg_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Модифицираното съдържание ** няма да бъде запазено, защото `--sv` не е извикано."##),
        ("new-value", r##"Нова стойност"##),
        ("invalid-path", r##"Невалиден път на файла."##),
    ],
}
}

/// Language ID: bg;
/// Map name: "set_md";
/// Description: български, кирилица, България;
pub(super) const fn get_bg_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mМодифицираното съдържание ** няма да бъде запазено, защото [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m не е извикано.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mНова стойност[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mНевалиден път на файла.[0m"##),
    ],
}
}

/// Language ID: bg;
/// Map name: "get";
/// Description: български, кирилица, България;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Формат на местоназначение");
/// ```
pub(super) const fn get_bg_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Формат на местоназначение"##),
        ("src-fmt", r##"Формат на изходния файл"##),
    ],
}
}

/// bg: български, кирилица, България
pub(super) const fn get_bg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_bg_map_conversion),
        ("get", get_bg_map_get),
        ("set_md", get_bg_map_set_md),
        ("set", get_bg_map_set),
        ("conversion_md", get_bg_map_conversion_md),
    ],
}
}

/// Language ID: bn;
/// Map name: "conversion";
/// Description: বাংলা, বাংলা, বাংলাদেশ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "স\u{9cd}বয\u{9bc}ংক\u{9cd}রিয\u{9bc} আক\u{9cd}রমণ সন\u{9cd}দেহজনক হয\u{9bc}েছে। দয\u{9bc}\u{9be} করে ম\u{9cd}য\u{9be}ন\u{9c1}য\u{9bc}\u{9be}লি উল\u{9cd}লেখ কর\u{9c1}ন।");
/// ```
pub(super) const fn get_bn_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"ফাইলটি বৈধ নয় `json 1.0` ফরম্যাট, চেষ্টা করে `json5` হিসাবে পার্স করা হচ্ছে..."##),
        ("not-included", r##"এই বাইনারি **বিষয়বস্তু হিসাবে অন্তর্ভুক্ত নয়**।
আপনাকে আপনার `Cargo.toml` -এ সম্পর্কিত বৈশিষ্ট্যটি সক্ষম করতে এবং পুনরায় সংমিশ্রণ করতে হবে।
যদি এই সফটওয়্যারটি উপযুক্ত কার্যক্রমটি অন্তর্ভুক্ত না করে, তবে একটি সমস্যা জমা দিয়ে দিন।"##),
        ("currently-supported", r##"বর্তমানে সমর্থিত ফরম্যাট তালিকা:"##),
        ("unsupported", r##"সমর্থিত নয় ফরম্যাটের রূপান্তর"##),
        ("auto-detection-failed", r##"স্বয়ংক্রিয় আক্রমণ সন্দেহজনক হয়েছে। দয়া করে ম্যানুয়ালি উল্লেখ করুন।"##),
        ("not-support-deser-sexp", r##"**এখনো সমর্থিত নয়**: `Lisp S-Expression` থেকে `অন্যান্য ফরম্যাট` -এ রূপান্তর করা"##),
        ("unknown-fmt", r##"অজানা ফাইল ফরম্যাট"##),
        ("not-saved", r##"`--save` কল করা না হওয়ার কারণে নিম্নলিখিত বিষয়বস্তু **সংরক্ষিত হবে না**।"##),
        ("dst", r##"গন্তব্য ফাইল পথ"##),
        ("conv-error", r##"রূপান্তর ত্রুটি"##),
    ],
}
}

/// Language ID: bn;
/// Map name: "conversion_md";
/// Description: বাংলা, বাংলা, বাংলাদেশ;
pub(super) const fn get_bn_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mফাইলটি বৈধ নয় [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ফরম্যাট, চেষ্টা করে [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m হিসাবে পার্স করা হচ্ছে...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mএই বাইনারি [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mবিষয়বস্তু হিসাবে অন্তর্ভুক্ত নয়[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m।
[48;2;34;34;34m[38;2;255;255;255mআপনাকে আপনার [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m -এ সম্পর্কিত বৈশিষ্ট্যটি সক্ষম করতে এবং পুনরায় সংমিশ্রণ করতে হবে।
[48;2;34;34;34m[38;2;255;255;255mযদি এই সফটওয়্যারটি উপযুক্ত কার্যক্রমটি অন্তর্ভুক্ত না করে, তবে একটি সমস্যা জমা দিয়ে দিন।[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mবর্তমানে সমর্থিত ফরম্যাট তালিকা:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mসমর্থিত নয় ফরম্যাটের রূপান্তর[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mস্বয়ংক্রিয় আক্রমণ সন্দেহজনক হয়েছে। দয়া করে ম্যানুয়ালি উল্লেখ করুন।[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mএখনো সমর্থিত নয়[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m থেকে [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mঅন্যান্য ফরম্যাট[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m -এ রূপান্তর করা[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mঅজানা ফাইল ফরম্যাট[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m কল করা না হওয়ার কারণে নিম্নলিখিত বিষয়বস্তু [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mসংরক্ষিত হবে না[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m।[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mগন্তব্য ফাইল পথ[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mরূপান্তর ত্রুটি[0m"##),
    ],
}
}

/// Language ID: bn;
/// Map name: "set";
/// Description: বাংলা, বাংলা, বাংলাদেশ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "অবৈধ ফ\u{9be}ইল প\u{9be}থ।");
/// ```
pub(super) const fn get_bn_map_set() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"অবৈধ ফাইল পাথ।"##),
        ("new-value", r##"নতুন মান"##),
        ("unsave-warn", r##"পরিবর্তিত সামগ্রী ** ** সংরক্ষণ করা হবে না কারণ `-এসভি` কল করা হয়নি।"##),
        ("type", r##"প্রকার"##),
    ],
}
}

/// Language ID: bn;
/// Map name: "set_md";
/// Description: বাংলা, বাংলা, বাংলাদেশ;
pub(super) const fn get_bn_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mঅবৈধ ফাইল পাথ।[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mনতুন মান[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mপরিবর্তিত সামগ্রী ** ** সংরক্ষণ করা হবে না কারণ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m-এসভি[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m কল করা হয়নি।[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255mপ্রকার[0m"##),
    ],
}
}

/// Language ID: bn;
/// Map name: "get";
/// Description: বাংলা, বাংলা, বাংলাদেশ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "গন\u{9cd}তব\u{9cd}য ফর\u{9cd}ম\u{9cd}য\u{9be}ট");
/// ```
pub(super) const fn get_bn_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"গন্তব্য ফর্ম্যাট"##),
        ("src-fmt", r##"উত্স ফাইল ফর্ম্যাট"##),
    ],
}
}

/// bn: বাংলা, বাংলা, বাংলাদেশ
pub(super) const fn get_bn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_bn_map_conversion),
        ("get", get_bn_map_get),
        ("set_md", get_bn_map_set_md),
        ("set", get_bn_map_set),
        ("conversion_md", get_bn_map_conversion_md),
    ],
}
}

/// Language ID: bs;
/// Map name: "conversion";
/// Description: bosanski, latinica, Bosna i Hercegovina;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Nije uspjelo automatski otkriti format.Molimo navedite ručno.");
/// ```
pub(super) const fn get_bs_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Datoteka nije važeći `JSON 1.0` format, pokušavajući da se raščlanjuje kao `json5` ..."##),
        ("not-included", r##"Ovaj binarni ** ne uključuje ** funkcionalnost konverzije za relevantni format.
Morate omogućiti relevantnu značajku u svom `Cargo.Toml` i Recompile.
Ako ovaj softver ne uključuje odgovarajuću funkcionalnost, molimo pošaljite problem."##),
        ("currently-supported", r##"Trenutno podržane liste formata:"##),
        ("unsupported", r##"Nepodržana konverzija formata"##),
        ("auto-detection-failed", r##"Nije uspjelo automatski otkriti format.Molimo navedite ručno."##),
        ("not-support-deser-sexp", r##"** Još nije podržana **: Pretvaranje iz `Lisp S-Expression` na` Ostale formate`"##),
        ("unknown-fmt", r##"Nepoznati format datoteke"##),
        ("not-saved", r##"Sljedeći sadržaj ** neće biti spremljen jer `--save` nije zvao."##),
        ("dst", r##"odredišna staza datoteke"##),
        ("conv-error", r##"Pogreška konverzije"##),
    ],
}
}

/// Language ID: bs;
/// Map name: "conversion_md";
/// Description: bosanski, latinica, Bosna i Hercegovina;
pub(super) const fn get_bs_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mDatoteka nije važeći [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m format, pokušavajući da se raščlanjuje kao [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mOvaj binarni ** ne uključuje ** funkcionalnost konverzije za relevantni format.
[48;2;34;34;34m[38;2;255;255;255mMorate omogućiti relevantnu značajku u svom [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m i Recompile.
[48;2;34;34;34m[38;2;255;255;255mAko ovaj softver ne uključuje odgovarajuću funkcionalnost, molimo pošaljite problem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mTrenutno podržane liste formata:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNepodržana konverzija formata[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNije uspjelo automatski otkriti format.Molimo navedite ručno.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Još nije podržana [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Pretvaranje iz [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m na[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m Ostale formate[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNepoznati format datoteke[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mSljedeći sadržaj ** neće biti spremljen jer [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nije zvao.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255modredišna staza datoteke[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mPogreška konverzije[0m"##),
    ],
}
}

/// Language ID: bs;
/// Map name: "set";
/// Description: bosanski, latinica, Bosna i Hercegovina;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Nevažeća staza datoteke.");
/// ```
pub(super) const fn get_bs_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Modificirani sadržaj ** neće biti spremljen jer `--sv` nije zvao."##),
        ("new-value", r##"Nova vrijednost"##),
        ("invalid-path", r##"Nevažeća staza datoteke."##),
    ],
}
}

/// Language ID: bs;
/// Map name: "set_md";
/// Description: bosanski, latinica, Bosna i Hercegovina;
pub(super) const fn get_bs_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mModificirani sadržaj ** neće biti spremljen jer [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nije zvao.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNova vrijednost[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNevažeća staza datoteke.[0m"##),
    ],
}
}

/// Language ID: bs;
/// Map name: "get";
/// Description: bosanski, latinica, Bosna i Hercegovina;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Destinacijski format");
/// ```
pub(super) const fn get_bs_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Destinacijski format"##),
        ("src-fmt", r##"Izvor Format datoteke"##),
    ],
}
}

/// bs: bosanski, latinica, Bosna i Hercegovina
pub(super) const fn get_bs_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_bs_map_conversion),
        ("get", get_bs_map_get),
        ("set_md", get_bs_map_set_md),
        ("set", get_bs_map_set),
        ("conversion_md", get_bs_map_conversion_md),
    ],
}
}

/// Language ID: ca;
/// Map name: "conversion";
/// Description: català, llatí, Espanya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "No s'ha pogut detectar automàticament el format.Especifiqueu manualment.");
/// ```
pub(super) const fn get_ca_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"El fitxer no és un format `json 1.0` vàlid, intentant analitzar com a `json5` ..."##),
        ("not-included", r##"Aquest binari ** no inclou ** la funcionalitat de conversió del format rellevant.
Heu d'activar la funció rellevant al vostre `Cargo.toml` i recompilar.
Si aquest programari no inclou la funcionalitat corresponent, envieu un problema."##),
        ("currently-supported", r##"Llista de formats actualment compatibles:"##),
        ("unsupported", r##"Conversió de format no suport"##),
        ("auto-detection-failed", r##"No s'ha pogut detectar automàticament el format.Especifiqueu manualment."##),
        ("not-support-deser-sexp", r##"** encara no és compatible **: convertint de `lisp s-expression` a` altres formats '"##),
        ("unknown-fmt", r##"Format de fitxer desconegut"##),
        ("not-saved", r##"El contingut següent ** no es desarà perquè no es va trucar a `--save`."##),
        ("dst", r##"ruta del fitxer de destinació"##),
        ("conv-error", r##"error de conversió"##),
    ],
}
}

/// Language ID: ca;
/// Map name: "conversion_md";
/// Description: català, llatí, Espanya;
pub(super) const fn get_ca_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mEl fitxer no és un format [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m vàlid, intentant analitzar com a [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mAquest binari ** no inclou ** la funcionalitat de conversió del format rellevant.
[48;2;34;34;34m[38;2;255;255;255mHeu d'activar la funció rellevant al vostre [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m i recompilar.
[48;2;34;34;34m[38;2;255;255;255mSi aquest programari no inclou la funcionalitat corresponent, envieu un problema.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLlista de formats actualment compatibles:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mConversió de format no suport[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNo s'ha pogut detectar automàticament el format.Especifiqueu manualment.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** encara no és compatible [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: convertint de [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m a[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m altres formats '[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormat de fitxer desconegut[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mEl contingut següent ** no es desarà perquè no es va trucar a [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mruta del fitxer de destinació[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255merror de conversió[0m"##),
    ],
}
}

/// Language ID: ca;
/// Map name: "set";
/// Description: català, llatí, Espanya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ruta del fitxer no vàlida.");
/// ```
pub(super) const fn get_ca_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"El contingut modificat ** no es desarà perquè no es va trucar a `--sv`."##),
        ("new-value", r##"valor nou"##),
        ("invalid-path", r##"ruta del fitxer no vàlida."##),
    ],
}
}

/// Language ID: ca;
/// Map name: "set_md";
/// Description: català, llatí, Espanya;
pub(super) const fn get_ca_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mEl contingut modificat ** no es desarà perquè no es va trucar a [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mvalor nou[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mruta del fitxer no vàlida.[0m"##),
    ],
}
}

/// Language ID: ca;
/// Map name: "get";
/// Description: català, llatí, Espanya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format de destinació");
/// ```
pub(super) const fn get_ca_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format de destinació"##),
        ("src-fmt", r##"Format del fitxer font"##),
    ],
}
}

/// ca: català, llatí, Espanya
pub(super) const fn get_ca_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ca_map_conversion),
        ("get", get_ca_map_get),
        ("set_md", get_ca_map_set_md),
        ("set", get_ca_map_set),
        ("conversion_md", get_ca_map_conversion_md),
    ],
}
}

/// Language ID: ceb;
/// Map name: "conversion";
/// Description: Cebuano, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Napakyas sa awtomatikong makit-an ang format.Palihug ipasabut sa mano-mano.");
/// ```
pub(super) const fn get_ceb_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Ang file dili usa ka balido nga format nga `JSON 1.0`, nga pagsulay sa pagpirma ingon nga `JSON5` ..."##),
        ("not-included", r##"Kini nga binary ** wala maglakip sa ** ang pag-andar sa pagkakabig alang sa may kalabutan nga format.
Kinahanglan nimo nga hatagan ang may kalabutan nga bahin sa imong `Cargo.toml 'ug balus.
Kung ang kini nga software wala maglakip sa katugbang nga pag-andar, palihug isumite ang usa ka isyu."##),
        ("currently-supported", r##"Gisuportahan karon ang Lista sa Formats:"##),
        ("unsupported", r##"Wala gisuportahan ang pagkakabig sa format"##),
        ("auto-detection-failed", r##"Napakyas sa awtomatikong makit-an ang format.Palihug ipasabut sa mano-mano."##),
        ("not-support-deser-sexp", r##"** Wala gisuportahan pa **: Pagbalhin gikan sa `Lisp S-Pahayag 'sa ubang mga formats`"##),
        ("unknown-fmt", r##"Wala mailhi nga Format File"##),
        ("not-saved", r##"Ang mosunud nga sulud dili maluwas tungod kay `--save` wala gitawag."##),
        ("dst", r##"Destination File Dalan"##),
        ("conv-error", r##"Sayup sa Pagbag-o"##),
    ],
}
}

/// Language ID: ceb;
/// Map name: "conversion_md";
/// Description: Cebuano, Latin, Pilipinas;
pub(super) const fn get_ceb_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mAng file dili usa ka balido nga format nga [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, nga pagsulay sa pagpirma ingon nga [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mKini nga binary ** wala maglakip sa ** ang pag-andar sa pagkakabig alang sa may kalabutan nga format.
[48;2;34;34;34m[38;2;255;255;255mKinahanglan nimo nga hatagan ang may kalabutan nga bahin sa imong [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml 'ug balus.
[48;2;34;34;34m[38;2;0;255;255mKung ang kini nga software wala maglakip sa katugbang nga pag-andar, palihug isumite ang usa ka isyu.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mGisuportahan karon ang Lista sa Formats:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mWala gisuportahan ang pagkakabig sa format[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNapakyas sa awtomatikong makit-an ang format.Palihug ipasabut sa mano-mano.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Wala gisuportahan pa [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Pagbalhin gikan sa [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Pahayag 'sa ubang mga formats[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mWala mailhi nga Format File[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mAng mosunud nga sulud dili maluwas tungod kay [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m wala gitawag.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestination File Dalan[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mSayup sa Pagbag-o[0m"##),
    ],
}
}

/// Language ID: ceb;
/// Map name: "set";
/// Description: Cebuano, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Dili husto nga agianan sa file.");
/// ```
pub(super) const fn get_ceb_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Ang giusab nga sulud * dili maluwas tungod kay '- "wala gitawag."##),
        ("new-value", r##"Bag-ong kantidad"##),
        ("invalid-path", r##"Dili husto nga agianan sa file."##),
    ],
}
}

/// Language ID: ceb;
/// Map name: "set_md";
/// Description: Cebuano, Latin, Pilipinas;
pub(super) const fn get_ceb_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mAng giusab nga sulud * dili maluwas tungod kay '- "wala gitawag.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mBag-ong kantidad[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mDili husto nga agianan sa file.[0m"##),
    ],
}
}

/// Language ID: ceb;
/// Map name: "get";
/// Description: Cebuano, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format sa destinasyon");
/// ```
pub(super) const fn get_ceb_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format sa destinasyon"##),
        ("src-fmt", r##"Format nga File Format"##),
    ],
}
}

/// ceb: Cebuano, Latin, Pilipinas
pub(super) const fn get_ceb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ceb_map_conversion),
        ("get", get_ceb_map_get),
        ("set_md", get_ceb_map_set_md),
        ("set", get_ceb_map_set),
        ("conversion_md", get_ceb_map_conversion_md),
    ],
}
}

/// Language ID: co;
/// Map name: "conversion";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Fallu automaticamente u furmatu.Per piacè specificà manualmente.");
/// ```
pub(super) const fn get_co_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"Errore di cunversione"##),
        ("not-support-deser-sexp", r##"** micca sustinutu ancu **: Cunverte da 'Lisp S-espressione` à `altri Formati`"##),
        ("dst", r##"PLAY DI FILE"##),
        ("unsupported", r##"Conversione di Format Inutile"##),
        ("auto-detection-failed", r##"Fallu automaticamente u furmatu.Per piacè specificà manualmente."##),
        ("currently-supported", r##"Lista di formati attualmente supportatu:"##),
        ("not-included", r##"Questu binariu ** Ùn include micca ** A funziunalità di cunversione per u furmatu pertinente.
Avete bisognu à attivà a funzione pertinente in u vostru 'cargu.toml` è recompile.
Se stu prugrammu ùn include micca a funziunalità currispondente, mandate un prublema."##),
        ("unknown-fmt", r##"Formatu di File Unknown"##),
    ],
}
}

/// Language ID: co;
/// Map name: "conversion_md";
pub(super) const fn get_co_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mErrore di cunversione[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** micca sustinutu ancu [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Cunverte da 'Lisp S-espressione[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m à [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114maltri Formati[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mPLAY DI FILE[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mConversione di Format Inutile[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mFallu automaticamente u furmatu.Per piacè specificà manualmente.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLista di formati attualmente supportatu:[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mQuestu binariu ** Ùn include micca ** A funziunalità di cunversione per u furmatu pertinente.
[48;2;34;34;34m[38;2;255;255;255mAvete bisognu à attivà a funzione pertinente in u vostru 'cargu.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m è recompile.
[48;2;34;34;34m[38;2;0;255;255mSe stu prugrammu ùn include micca a funziunalità currispondente, mandate un prublema.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormatu di File Unknown[0m"##),
    ],
}
}

/// Language ID: co;
/// Map name: "set";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "PIAZZA DI FILE INVALIDU.");
/// ```
pub(super) const fn get_co_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"U cuntenutu mudificatu ** ùn sarà micca sviatu perchè `--sv` ùn hè micca chjamatu."##),
        ("new-value", r##"Novu Valore"##),
        ("invalid-path", r##"PIAZZA DI FILE INVALIDU."##),
    ],
}
}

/// Language ID: co;
/// Map name: "set_md";
pub(super) const fn get_co_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mU cuntenutu mudificatu ** ùn sarà micca sviatu perchè [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ùn hè micca chjamatu.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNovu Valore[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mPIAZZA DI FILE INVALIDU.[0m"##),
    ],
}
}

/// Language ID: co;
/// Map name: "get";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Furmatu di destinazione");
/// ```
pub(super) const fn get_co_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Furmatu di destinazione"##),
        ("src-fmt", r##"Formatu di File Fonte"##),
    ],
}
}

/// co: co-Latn-FR
pub(super) const fn get_co_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_co_map_conversion),
        ("get", get_co_map_get),
        ("set_md", get_co_map_set_md),
        ("set", get_co_map_set),
        ("conversion_md", get_co_map_conversion_md),
    ],
}
}

/// Language ID: cs;
/// Map name: "conversion";
/// Description: čeština, latinka, Česko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Automatické rozpoznání formátu selhalo. Prosím, specifikujte ho ručně.");
/// ```
pub(super) const fn get_cs_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Soubor není platný formát `json 1.0`, pokus o parsování jako `json5`..."##),
        ("not-included", r##"Tento binární soubor **neobsahuje** funkcionalitu pro konverzi příslušného formátu.
Je nutné zapnout odpovídající funkci v `Cargo.toml` a znovu zkompilovat.
Pokud toto softwaru nezahrnuje odpovídající funkčnost, neváhejte nám poslat issue."##),
        ("currently-supported", r##"Aktuálně podporované formáty:"##),
        ("unsupported", r##"Konverze nepodporována"##),
        ("auto-detection-failed", r##"Automatické rozpoznání formátu selhalo. Prosím, specifikujte ho ručně."##),
        ("not-support-deser-sexp", r##"**Zatím nepodporováno**: konverze z `Lisp S-Expression` na `ostatní formáty`"##),
        ("unknown-fmt", r##"Neznámý formát souboru"##),
        ("not-saved", r##"Následující obsah **nebude** uložen, protože nebylo použito `--save`."##),
        ("dst", r##"Cílová cesta k výstupnímu souboru"##),
        ("conv-error", r##"Chyba konverze"##),
    ],
}
}

/// Language ID: cs;
/// Map name: "conversion_md";
/// Description: čeština, latinka, Česko;
pub(super) const fn get_cs_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mSoubor není platný formát [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, pokus o parsování jako [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mTento binární soubor [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mneobsahuje[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m funkcionalitu pro konverzi příslušného formátu.
[48;2;34;34;34m[38;2;255;255;255mJe nutné zapnout odpovídající funkci v [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m a znovu zkompilovat.
[48;2;34;34;34m[38;2;255;255;255mPokud toto softwaru nezahrnuje odpovídající funkčnost, neváhejte nám poslat issue.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mAktuálně podporované formáty:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mKonverze nepodporována[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mAutomatické rozpoznání formátu selhalo. Prosím, specifikujte ho ručně.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mZatím nepodporováno[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: konverze z [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m na [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mostatní formáty[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNeznámý formát souboru[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNásledující obsah [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnebude[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m uložen, protože nebylo použito [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mCílová cesta k výstupnímu souboru[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mChyba konverze[0m"##),
    ],
}
}

/// Language ID: cs;
/// Map name: "set";
/// Description: čeština, latinka, Česko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Neplatná cesta souboru.");
/// ```
pub(super) const fn get_cs_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Upravený obsah ** nebude ** uložen, protože `--sv` nebyl volán."##),
        ("new-value", r##"Nová hodnota"##),
        ("invalid-path", r##"Neplatná cesta souboru."##),
    ],
}
}

/// Language ID: cs;
/// Map name: "set_md";
/// Description: čeština, latinka, Česko;
pub(super) const fn get_cs_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mUpravený obsah ** nebude ** uložen, protože [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nebyl volán.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNová hodnota[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNeplatná cesta souboru.[0m"##),
    ],
}
}

/// Language ID: cs;
/// Map name: "get";
/// Description: čeština, latinka, Česko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Formát cíle");
/// ```
pub(super) const fn get_cs_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Formát cíle"##),
        ("src-fmt", r##"Formát zdrojového souboru"##),
    ],
}
}

/// cs: čeština, latinka, Česko
pub(super) const fn get_cs_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_cs_map_conversion),
        ("get", get_cs_map_get),
        ("set_md", get_cs_map_set_md),
        ("set", get_cs_map_set),
        ("conversion_md", get_cs_map_conversion_md),
    ],
}
}

/// Language ID: cy;
/// Map name: "conversion";
/// Description: Cymraeg, Lladin, Y Deyrnas Unedig;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "wedi methu â chanfod y fformat yn awtomatig.Nodwch â llaw.");
/// ```
pub(super) const fn get_cy_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Nid yw'r ffeil yn fformat `json 1.0` dilys, yn ceisio dosrannu fel `json5` ..."##),
        ("not-included", r##"Nid yw'r deuaidd ** hwn yn cynnwys ** ymarferoldeb trosi ar gyfer y fformat perthnasol.
Mae angen i chi alluogi'r nodwedd berthnasol yn eich `Cargo.toml` ac ail -grynhoi.
Os nad yw'r feddalwedd hon yn cynnwys yr ymarferoldeb cyfatebol, cyflwynwch fater."##),
        ("currently-supported", r##"ar hyn o bryd yn cefnogi fformatau: Rhestr Fformatau:"##),
        ("unsupported", r##"trosi fformat heb gefnogaeth"##),
        ("auto-detection-failed", r##"wedi methu â chanfod y fformat yn awtomatig.Nodwch â llaw."##),
        ("not-support-deser-sexp", r##"** heb ei gefnogi eto **: trosi o `lisp s-fynegiant` i` fformatau eraill`"##),
        ("unknown-fmt", r##"fformat ffeil anhysbys"##),
        ("not-saved", r##"Ni fydd y cynnwys canlynol ** yn cael ei arbed ** oherwydd ni alwyd `--save`."##),
        ("dst", r##"llwybr ffeil cyrchfan"##),
        ("conv-error", r##"gwall trosi"##),
    ],
}
}

/// Language ID: cy;
/// Map name: "conversion_md";
/// Description: Cymraeg, Lladin, Y Deyrnas Unedig;
pub(super) const fn get_cy_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mNid yw'r ffeil yn fformat [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m dilys, yn ceisio dosrannu fel [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mNid yw'r deuaidd ** hwn yn cynnwys ** ymarferoldeb trosi ar gyfer y fformat perthnasol.
[48;2;34;34;34m[38;2;255;255;255mMae angen i chi alluogi'r nodwedd berthnasol yn eich [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ac ail -grynhoi.
[48;2;34;34;34m[38;2;255;255;255mOs nad yw'r feddalwedd hon yn cynnwys yr ymarferoldeb cyfatebol, cyflwynwch fater.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mar hyn o bryd yn cefnogi fformatau: Rhestr Fformatau:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mtrosi fformat heb gefnogaeth[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mwedi methu â chanfod y fformat yn awtomatig.Nodwch â llaw.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** heb ei gefnogi eto [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: trosi o [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-fynegiant[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m i[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m fformatau eraill[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mfformat ffeil anhysbys[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNi fydd y cynnwys canlynol ** yn cael ei arbed ** oherwydd ni alwyd [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mllwybr ffeil cyrchfan[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mgwall trosi[0m"##),
    ],
}
}

/// Language ID: cy;
/// Map name: "set";
/// Description: Cymraeg, Lladin, Y Deyrnas Unedig;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "llwybr ffeil annilys.");
/// ```
pub(super) const fn get_cy_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ni fydd y cynnwys wedi'i addasu ** yn cael ei arbed ** oherwydd na elwir `--sv`."##),
        ("new-value", r##"gwerth newydd"##),
        ("invalid-path", r##"llwybr ffeil annilys."##),
    ],
}
}

/// Language ID: cy;
/// Map name: "set_md";
/// Description: Cymraeg, Lladin, Y Deyrnas Unedig;
pub(super) const fn get_cy_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mni fydd y cynnwys wedi'i addasu ** yn cael ei arbed ** oherwydd na elwir [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mgwerth newydd[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mllwybr ffeil annilys.[0m"##),
    ],
}
}

/// Language ID: cy;
/// Map name: "get";
/// Description: Cymraeg, Lladin, Y Deyrnas Unedig;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Fformat cyrchfan");
/// ```
pub(super) const fn get_cy_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Fformat cyrchfan"##),
        ("src-fmt", r##"Fformat Ffeil Ffynhonnell"##),
    ],
}
}

/// cy: Cymraeg, Lladin, Y Deyrnas Unedig
pub(super) const fn get_cy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_cy_map_conversion),
        ("get", get_cy_map_get),
        ("set_md", get_cy_map_set_md),
        ("set", get_cy_map_set),
        ("conversion_md", get_cy_map_conversion_md),
    ],
}
}

/// Language ID: da;
/// Map name: "conversion";
/// Description: dansk, latinsk, Danmark;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "kunne ikke automatisk registrere formatet.Angiv venligst manuelt.");
/// ```
pub(super) const fn get_da_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Filen er ikke et gyldigt `JSON 1.0 'format, der prøver at analysere som `JSON5` ..."##),
        ("not-included", r##"Denne binære ** inkluderer ikke ** konverteringsfunktionaliteten for det relevante format.
Du skal aktivere den relevante funktion i din `last.toml` og omkompilere.
Hvis denne software ikke inkluderer den tilsvarende funktionalitet, skal du indsende et problem."##),
        ("currently-supported", r##"Liste i aktuelt understøttede formater:"##),
        ("unsupported", r##"Ikke -understøttet formatkonvertering"##),
        ("auto-detection-failed", r##"kunne ikke automatisk registrere formatet.Angiv venligst manuelt."##),
        ("not-support-deser-sexp", r##"** understøttes endnu ikke **: konvertering fra `lisp s-ekspression 'til` andre formater`"##),
        ("unknown-fmt", r##"Ukendt filformat"##),
        ("not-saved", r##"Følgende indhold ** gemmes ikke **, fordi `--save` ikke blev kaldt."##),
        ("dst", r##"destinationsfilsti"##),
        ("conv-error", r##"Konverteringsfejl"##),
    ],
}
}

/// Language ID: da;
/// Map name: "conversion_md";
/// Description: dansk, latinsk, Danmark;
pub(super) const fn get_da_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFilen er ikke et gyldigt [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0 'format, der prøver at analysere som [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDenne binære ** inkluderer ikke ** konverteringsfunktionaliteten for det relevante format.
[48;2;34;34;34m[38;2;255;255;255mDu skal aktivere den relevante funktion i din [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlast.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m og omkompilere.
[48;2;34;34;34m[38;2;255;255;255mHvis denne software ikke inkluderer den tilsvarende funktionalitet, skal du indsende et problem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mListe i aktuelt understøttede formater:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mIkke -understøttet formatkonvertering[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mkunne ikke automatisk registrere formatet.Angiv venligst manuelt.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** understøttes endnu ikke [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: konvertering fra [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-ekspression 'til[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m andre formater[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mUkendt filformat[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mFølgende indhold ** gemmes ikke [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m, fordi [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ikke blev kaldt.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mdestinationsfilsti[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonverteringsfejl[0m"##),
    ],
}
}

/// Language ID: da;
/// Map name: "set";
/// Description: dansk, latinsk, Danmark;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ugyldig filsti.");
/// ```
pub(super) const fn get_da_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Det ændrede indhold ** gemmes ikke **, fordi `-SV` ikke blev kaldt."##),
        ("new-value", r##"Ny værdi"##),
        ("invalid-path", r##"Ugyldig filsti."##),
    ],
}
}

/// Language ID: da;
/// Map name: "set_md";
/// Description: dansk, latinsk, Danmark;
pub(super) const fn get_da_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDet ændrede indhold ** gemmes ikke [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m, fordi [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m-SV[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ikke blev kaldt.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNy værdi[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mUgyldig filsti.[0m"##),
    ],
}
}

/// Language ID: da;
/// Map name: "get";
/// Description: dansk, latinsk, Danmark;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Destinationsformat");
/// ```
pub(super) const fn get_da_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Destinationsformat"##),
        ("src-fmt", r##"Kildefilformat"##),
    ],
}
}

/// da: dansk, latinsk, Danmark
pub(super) const fn get_da_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_da_map_conversion),
        ("get", get_da_map_get),
        ("set_md", get_da_map_set_md),
        ("set", get_da_map_set),
        ("conversion_md", get_da_map_conversion_md),
    ],
}
}

/// Language ID: de;
/// Map name: "conversion";
/// Description: Deutsch, Lateinisch, Deutschland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Automatische Erkennung des Formats fehlgeschlagen, bitte manuell angeben.");
/// ```
pub(super) const fn get_de_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Die Datei hat kein gültiges `json 1.0` Format, versuche es als `json5` zu interpretieren..."##),
        ("not-included", r##"Diese Binärdatei enthält **nicht** die Funktionen zur Konvertierung in das entsprechende Format.
Sie müssen die relevanten Funktionen in Ihrer `Cargo.toml` aktivieren und erneut kompilieren.
Bitte erstellen Sie einen Fehlerbericht, wenn diese Software keine zugehörigen Funktionen enthält."##),
        ("currently-supported", r##"Aktuell unterstützte Formate"##),
        ("unsupported", r##"Nicht unterstützte Formatkonvertierung"##),
        ("auto-detection-failed", r##"Automatische Erkennung des Formats fehlgeschlagen, bitte manuell angeben."##),
        ("not-support-deser-sexp", r##"**Das Konvertieren von `Lisp S-Expression` in andere Formate ist vorübergehend nicht möglich.**"##),
        ("unknown-fmt", r##"Unbekanntes Dateiformat"##),
        ("not-saved", r##"Da `--save` nicht aufgerufen wurde, werden die folgenden Inhalte **nicht** gespeichert:"##),
        ("dst", r##"Zielpfad"##),
        ("conv-error", r##"Konvertierungsfehler"##),
    ],
}
}

/// Language ID: de;
/// Map name: "conversion_md";
/// Description: Deutsch, Lateinisch, Deutschland;
pub(super) const fn get_de_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mDie Datei hat kein gültiges [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m Format, versuche es als [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m zu interpretieren...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDiese Binärdatei enthält [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnicht[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m die Funktionen zur Konvertierung in das entsprechende Format.
[48;2;34;34;34m[38;2;255;255;255mSie müssen die relevanten Funktionen in Ihrer [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m aktivieren und erneut kompilieren.
[48;2;34;34;34m[38;2;255;255;255mBitte erstellen Sie einen Fehlerbericht, wenn diese Software keine zugehörigen Funktionen enthält.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mAktuell unterstützte Formate[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNicht unterstützte Formatkonvertierung[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mAutomatische Erkennung des Formats fehlgeschlagen, bitte manuell angeben.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mDas Konvertieren von [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m in andere Formate ist vorübergehend nicht möglich.[48;2;34;34;34m[38;2;249;38;114m**[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mUnbekanntes Dateiformat[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mDa [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nicht aufgerufen wurde, werden die folgenden Inhalte [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnicht[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m gespeichert:[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mZielpfad[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonvertierungsfehler[0m"##),
    ],
}
}

/// Language ID: de;
/// Map name: "get";
/// Description: Deutsch, Lateinisch, Deutschland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Zielformat");
/// ```
pub(super) const fn get_de_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Zielformat"##),
        ("src-fmt", r##"Quellformat"##),
    ],
}
}

/// Language ID: de;
/// Map name: "set";
/// Description: Deutsch, Lateinisch, Deutschland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ungültiger Dateipfad");
/// ```
pub(super) const fn get_de_map_set() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"Ungültiger Dateipfad"##),
        ("new-value", r##"Neuer Wert"##),
        ("unsave-warn", r##"Die geänderten Inhalte werden **nicht** gespeichert, da `--save` nicht aufgerufen wurde."##),
        ("type", r##"Typ"##),
    ],
}
}

/// Language ID: de;
/// Map name: "set_md";
/// Description: Deutsch, Lateinisch, Deutschland;
pub(super) const fn get_de_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mUngültiger Dateipfad[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNeuer Wert[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDie geänderten Inhalte werden [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnicht[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m gespeichert, da [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nicht aufgerufen wurde.[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255mTyp[0m"##),
    ],
}
}

/// de: Deutsch, Lateinisch, Deutschland
pub(super) const fn get_de_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_de_map_conversion),
        ("get", get_de_map_get),
        ("set_md", get_de_map_set_md),
        ("set", get_de_map_set),
        ("conversion_md", get_de_map_conversion_md),
    ],
}
}

/// Language ID: el;
/// Map name: "conversion";
/// Description: Ελληνικά, Ελληνικό, Ελλάδα;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Η αυτόματη ανίχνευση μορφής απέτυχε, παρακαλώ καθορίστε τη μορφή χειροκίνητα.");
/// ```
pub(super) const fn get_el_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Το αρχείο δεν είναι έγκυρη μορφή `json 1.0`, προσπαθεί να γίνει ανάλυση ως `json5`..."##),
        ("not-included", r##"Αυτό το δυαδικό αρχείο δεν **περιλαμβάνει** λειτουργίες μετατροπής για τη συγκεκριμένη μορφή.
Θα πρέπει να ενεργοποιήσετε τη σχετική λειτουργία στο αρχείο `Cargo.toml` και να ξανακάνετε τη μεταγλώττιση.
Εάν αυτό το λογισμικό δεν περιλαμβάνει τη σχετική λειτουργία, υποβάλετε ένα αίτημα (issue)."##),
        ("currently-supported", r##"Λίστα των τρέχοντων υποστηριζόμενων μορφών"##),
        ("unsupported", r##"Μη υποστηριζόμενη μορφή μετατροπής"##),
        ("auto-detection-failed", r##"Η αυτόματη ανίχνευση μορφής απέτυχε, παρακαλώ καθορίστε τη μορφή χειροκίνητα."##),
        ("not-support-deser-sexp", r##"**Δεν υποστηρίζεται προς το παρόν** η μετατροπή από `Lisp S-Expression` σε `άλλες μορφές`"##),
        ("unknown-fmt", r##"Άγνωστη μορφή αρχείου"##),
        ("not-saved", r##"Δεδομένου ότι δεν κλήθηκε η παράμετρος `--save`, τα παρακάτω περιεχόμενα **δεν θα** αποθηκευτούν."##),
        ("dst", r##"Διαδρομή αρχείου προορισμού"##),
        ("conv-error", r##"Σφάλμα μετατροπής"##),
    ],
}
}

/// Language ID: el;
/// Map name: "conversion_md";
/// Description: Ελληνικά, Ελληνικό, Ελλάδα;
pub(super) const fn get_el_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mΤο αρχείο δεν είναι έγκυρη μορφή [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, προσπαθεί να γίνει ανάλυση ως [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mΑυτό το δυαδικό αρχείο δεν [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mπεριλαμβάνει[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m λειτουργίες μετατροπής για τη συγκεκριμένη μορφή.
[48;2;34;34;34m[38;2;255;255;255mΘα πρέπει να ενεργοποιήσετε τη σχετική λειτουργία στο αρχείο [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m και να ξανακάνετε τη μεταγλώττιση.
[48;2;34;34;34m[38;2;255;255;255mΕάν αυτό το λογισμικό δεν περιλαμβάνει τη σχετική λειτουργία, υποβάλετε ένα αίτημα (issue).[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mΛίστα των τρέχοντων υποστηριζόμενων μορφών[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mΜη υποστηριζόμενη μορφή μετατροπής[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mΗ αυτόματη ανίχνευση μορφής απέτυχε, παρακαλώ καθορίστε τη μορφή χειροκίνητα.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mΔεν υποστηρίζεται προς το παρόν[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m η μετατροπή από [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m σε [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mάλλες μορφές[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mΆγνωστη μορφή αρχείου[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mΔεδομένου ότι δεν κλήθηκε η παράμετρος [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, τα παρακάτω περιεχόμενα [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mδεν θα[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m αποθηκευτούν.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mΔιαδρομή αρχείου προορισμού[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mΣφάλμα μετατροπής[0m"##),
    ],
}
}

/// Language ID: el;
/// Map name: "get";
/// Description: Ελληνικά, Ελληνικό, Ελλάδα;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Μορφή προορισμού");
/// ```
pub(super) const fn get_el_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Μορφή προορισμού"##),
        ("src-fmt", r##"Μορφή πηγαίου αρχείου"##),
    ],
}
}

/// Language ID: el;
/// Map name: "set";
/// Description: Ελληνικά, Ελληνικό, Ελλάδα;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Μη έγκυρη διαδρομή αρχείου");
/// ```
pub(super) const fn get_el_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Δεδομένου ότι δεν κλήθηκε η παράμετρος `--save`, οι τροποποιημένες πληροφορίες **δεν θα** αποθηκευτούν."##),
        ("new-value", r##"Νέα τιμή (value)"##),
        ("invalid-path", r##"Μη έγκυρη διαδρομή αρχείου"##),
    ],
}
}

/// Language ID: el;
/// Map name: "set_md";
/// Description: Ελληνικά, Ελληνικό, Ελλάδα;
pub(super) const fn get_el_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mΔεδομένου ότι δεν κλήθηκε η παράμετρος [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, οι τροποποιημένες πληροφορίες [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mδεν θα[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m αποθηκευτούν.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mΝέα τιμή (value)[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mΜη έγκυρη διαδρομή αρχείου[0m"##),
    ],
}
}

/// el: Ελληνικά, Ελληνικό, Ελλάδα
pub(super) const fn get_el_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_el_map_conversion),
        ("get", get_el_map_get),
        ("set_md", get_el_map_set_md),
        ("set", get_el_map_set),
        ("conversion_md", get_el_map_conversion_md),
    ],
}
}

/// Language ID: en;
/// Map name: "conversion";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Failed to automatically detect the format. Please specify manually.");
/// ```
pub(super) const fn get_en_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"The file is not a valid `json 1.0` format, trying to parse as `json5`..."##),
        ("not-included", r##"This binary **does not include** the conversion functionality for the relevant format.
You need to enable the relevant feature in your `Cargo.toml` and recompile.
If this software does not include the corresponding functionality, please submit an issue."##),
        ("currently-supported", r##"Currently supported formats list:"##),
        ("unsupported", r##"Unsupported format conversion"##),
        ("auto-detection-failed", r##"Failed to automatically detect the format. Please specify manually."##),
        ("not-support-deser-sexp", r##"**Not supported yet**: converting from `Lisp S-Expression` to `other formats`"##),
        ("unknown-fmt", r##"Unknown file format"##),
        ("not-saved", r##"The following content **will not** be saved because `--save` was not called."##),
        ("dst", r##"Destination file path"##),
        ("conv-error", r##"Conversion error"##),
    ],
}
}

/// Language ID: en;
/// Map name: "conversion_md";
/// Description: English, Latin, United States;
pub(super) const fn get_en_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mThe file is not a valid [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m format, trying to parse as [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mThis binary [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mdoes not include[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m the conversion functionality for the relevant format.
[48;2;34;34;34m[38;2;255;255;255mYou need to enable the relevant feature in your [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m and recompile.
[48;2;34;34;34m[38;2;255;255;255mIf this software does not include the corresponding functionality, please submit an issue.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mCurrently supported formats list:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mUnsupported format conversion[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mFailed to automatically detect the format. Please specify manually.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mNot supported yet[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: converting from [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m to [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mother formats[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mUnknown file format[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mThe following content [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mwill not[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m be saved because [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m was not called.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestination file path[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mConversion error[0m"##),
    ],
}
}

/// Language ID: en;
/// Map name: "get";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Destination format");
/// ```
pub(super) const fn get_en_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Destination format"##),
        ("src-fmt", r##"Source file format"##),
    ],
}
}

/// Language ID: en;
/// Map name: "set";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Invalid file path.");
/// ```
pub(super) const fn get_en_map_set() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"Invalid file path."##),
        ("new-value", r##"New value"##),
        ("unsave-warn", r##"The modified content **will not** be saved because `--sv` was not called."##),
        ("type", r##"Type"##),
    ],
}
}

/// Language ID: en;
/// Map name: "set_md";
/// Description: English, Latin, United States;
pub(super) const fn get_en_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mInvalid file path.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNew value[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mThe modified content [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mwill not[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m be saved because [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m was not called.[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255mType[0m"##),
    ],
}
}

/// en: English, Latin, United States
pub(super) const fn get_en_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_en_map_conversion),
        ("get", get_en_map_get),
        ("set_md", get_en_map_set_md),
        ("set", get_en_map_set),
        ("conversion_md", get_en_map_conversion_md),
    ],
}
}

/// Language ID: eo;
/// Map name: "conversion";
/// Description: esperanto, Latn, Mondo;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Malsukcesis aŭtomate detekti la formaton.Bonvolu specifi permane.");
/// ```
pub(super) const fn get_eo_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"La dosiero ne estas valida formato 'JSON 1.0`, provante analizi kiel `json5` ..."##),
        ("not-included", r##"Ĉi tiu binara ** ne inkluzivas ** la konvertan funkciecon por la koncernata formato.
Vi devas ebligi la koncernan funkcion en via `Cargo.toml` kaj rekompiri.
Se ĉi tiu programaro ne inkluzivas la respondan funkciecon, bonvolu sendi problemon."##),
        ("currently-supported", r##"Aktuale subtenataj formataj listoj:"##),
        ("unsupported", r##"Nepruvitan formatan konvertiĝon"##),
        ("auto-detection-failed", r##"Malsukcesis aŭtomate detekti la formaton.Bonvolu specifi permane."##),
        ("not-support-deser-sexp", r##"** ankoraŭ ne subtenata **: konverti de `lisp s-esprimo` al` aliaj formatoj`"##),
        ("unknown-fmt", r##"nekonatan dosierformaton"##),
        ("not-saved", r##"La sekva enhavo ** ne estos konservita ĉar `--save` ne estis nomita."##),
        ("dst", r##"Destina Dosiera Vojo"##),
        ("conv-error", r##"Konverta Eraro"##),
    ],
}
}

/// Language ID: eo;
/// Map name: "conversion_md";
/// Description: esperanto, Latn, Mondo;
pub(super) const fn get_eo_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mLa dosiero ne estas valida formato 'JSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m, provante analizi kiel [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mĈi tiu binara ** ne inkluzivas ** la konvertan funkciecon por la koncernata formato.
[48;2;34;34;34m[38;2;255;255;255mVi devas ebligi la koncernan funkcion en via [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m kaj rekompiri.
[48;2;34;34;34m[38;2;255;255;255mSe ĉi tiu programaro ne inkluzivas la respondan funkciecon, bonvolu sendi problemon.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mAktuale subtenataj formataj listoj:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNepruvitan formatan konvertiĝon[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mMalsukcesis aŭtomate detekti la formaton.Bonvolu specifi permane.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ankoraŭ ne subtenata [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: konverti de [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-esprimo[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m al[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m aliaj formatoj[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mnekonatan dosierformaton[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mLa sekva enhavo ** ne estos konservita ĉar [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ne estis nomita.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestina Dosiera Vojo[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonverta Eraro[0m"##),
    ],
}
}

/// Language ID: eo;
/// Map name: "set";
/// Description: esperanto, Latn, Mondo;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Nevalidan dosieran vojon.");
/// ```
pub(super) const fn get_eo_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"La modifita enhavo ** ne estos konservita ĉar `--sv` ne estis vokita."##),
        ("new-value", r##"Nova valoro"##),
        ("invalid-path", r##"Nevalidan dosieran vojon."##),
    ],
}
}

/// Language ID: eo;
/// Map name: "set_md";
/// Description: esperanto, Latn, Mondo;
pub(super) const fn get_eo_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mLa modifita enhavo ** ne estos konservita ĉar [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ne estis vokita.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNova valoro[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNevalidan dosieran vojon.[0m"##),
    ],
}
}

/// Language ID: eo;
/// Map name: "get";
/// Description: esperanto, Latn, Mondo;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Celloka formato");
/// ```
pub(super) const fn get_eo_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Celloka formato"##),
        ("src-fmt", r##"Fonta dosierformato"##),
    ],
}
}

/// eo: esperanto, Latn, Mondo
pub(super) const fn get_eo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_eo_map_conversion),
        ("get", get_eo_map_get),
        ("set_md", get_eo_map_set_md),
        ("set", get_eo_map_set),
        ("conversion_md", get_eo_map_conversion_md),
    ],
}
}

/// Language ID: es;
/// Map name: "conversion";
/// Description: español, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "La detección automática de formato falló, por favor especifique manualmente.");
/// ```
pub(super) const fn get_es_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"El archivo no es un formato JSON 1.0 válido, intentando analizar como `json5`..."##),
        ("not-included", r##"Este archivo binario no contiene funciones de conversión para este formato.
Debe habilitar la función correspondiente en su archivo `Cargo.toml` y volver a compilar.
Si este software no contiene esta función, presente un problema (issue)."##),
        ("currently-supported", r##"Lista actual de formatos soportados"##),
        ("unsupported", r##"Conversión de formato no soportada"##),
        ("auto-detection-failed", r##"La detección automática de formato falló, por favor especifique manualmente."##),
        ("not-support-deser-sexp", r##"**Actualmente no compatible** convertir `Lisp S-Expression` a `otros formatos`"##),
        ("unknown-fmt", r##"Formato de archivo desconocido"##),
        ("not-saved", r##"Dado que no se llamó a la opción `--save`, el siguiente contenido **no será** guardado."##),
        ("dst", r##"Ruta del archivo de destino"##),
        ("conv-error", r##"Error de conversión"##),
    ],
}
}

/// Language ID: es;
/// Map name: "conversion_md";
/// Description: español, latino, España;
pub(super) const fn get_es_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mEl archivo no es un formato JSON 1.0 válido, intentando analizar como [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mEste archivo binario no contiene funciones de conversión para este formato.
[48;2;34;34;34m[38;2;255;255;255mDebe habilitar la función correspondiente en su archivo [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m y volver a compilar.
[48;2;34;34;34m[38;2;255;255;255mSi este software no contiene esta función, presente un problema (issue).[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLista actual de formatos soportados[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mConversión de formato no soportada[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mLa detección automática de formato falló, por favor especifique manualmente.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mActualmente no compatible[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m convertir [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m a [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255motros formatos[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormato de archivo desconocido[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mDado que no se llamó a la opción [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, el siguiente contenido [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mno será[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m guardado.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mRuta del archivo de destino[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mError de conversión[0m"##),
    ],
}
}

/// Language ID: es;
/// Map name: "get";
/// Description: español, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Formato de archivo de destino");
/// ```
pub(super) const fn get_es_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Formato de archivo de destino"##),
        ("src-fmt", r##"Formato de archivo fuente"##),
    ],
}
}

/// Language ID: es;
/// Map name: "set";
/// Description: español, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ruta de archivo no válida");
/// ```
pub(super) const fn get_es_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Dado que no se llamó a la opción `--save`, la información modificada **no será** guardada."##),
        ("new-value", r##"Nuevo valor"##),
        ("invalid-path", r##"Ruta de archivo no válida"##),
    ],
}
}

/// Language ID: es;
/// Map name: "set_md";
/// Description: español, latino, España;
pub(super) const fn get_es_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDado que no se llamó a la opción [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, la información modificada [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mno será[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m guardada.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNuevo valor[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mRuta de archivo no válida[0m"##),
    ],
}
}

/// es: español, latino, España
pub(super) const fn get_es_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_es_map_conversion),
        ("get", get_es_map_get),
        ("set_md", get_es_map_set_md),
        ("set", get_es_map_set),
        ("conversion_md", get_es_map_conversion_md),
    ],
}
}

/// Language ID: et;
/// Map name: "conversion";
/// Description: eesti, ladina, Eesti;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Vormingu automaatselt tuvastamine ebaõnnestus.Palun täpsustage käsitsi.");
/// ```
pub(super) const fn get_et_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Fail ei ole kehtiv `JSON 1.0` vorming, üritades sõeluda kui `json5`..."##),
        ("not-included", r##"See binaarne ** ei hõlma ** vastava vormingu teisendamise funktsionaalsust.
Peate lubama oma `Cargo.toml` ja uuesti kompileerimise asjakohase funktsiooni.
Kui see tarkvara ei sisalda vastavat funktsionaalsust, esitage palun probleem."##),
        ("currently-supported", r##"Praegu toetatud vormingute loend:"##),
        ("unsupported", r##"Vormingu toetamata muundamine"##),
        ("auto-detection-failed", r##"Vormingu automaatselt tuvastamine ebaõnnestus.Palun täpsustage käsitsi."##),
        ("not-support-deser-sexp", r##"** pole veel toetatud **: teisendamine `Lisp S-Expression`` muudeks vorminguteks"##),
        ("unknown-fmt", r##"Tundmatu failivorming"##),
        ("not-saved", r##"Järgmist sisu ** ei salvestata **, kuna `--save` ei kutsutud."##),
        ("dst", r##"sihtfaili tee"##),
        ("conv-error", r##"teisendamise viga"##),
    ],
}
}

/// Language ID: et;
/// Map name: "conversion_md";
/// Description: eesti, ladina, Eesti;
pub(super) const fn get_et_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFail ei ole kehtiv [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m vorming, üritades sõeluda kui [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mSee binaarne ** ei hõlma ** vastava vormingu teisendamise funktsionaalsust.
[48;2;34;34;34m[38;2;255;255;255mPeate lubama oma [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ja uuesti kompileerimise asjakohase funktsiooni.
[48;2;34;34;34m[38;2;255;255;255mKui see tarkvara ei sisalda vastavat funktsionaalsust, esitage palun probleem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mPraegu toetatud vormingute loend:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mVormingu toetamata muundamine[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mVormingu automaatselt tuvastamine ebaõnnestus.Palun täpsustage käsitsi.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** pole veel toetatud [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: teisendamine [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression`` muudeks vorminguteks[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mTundmatu failivorming[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mJärgmist sisu ** ei salvestata [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m, kuna [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ei kutsutud.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255msihtfaili tee[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mteisendamise viga[0m"##),
    ],
}
}

/// Language ID: et;
/// Map name: "set";
/// Description: eesti, ladina, Eesti;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Vale failitee.");
/// ```
pub(super) const fn get_et_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Muudetud sisu ** ei salvestata **, kuna `--sv` ei kutsutud."##),
        ("new-value", r##"Uus väärtus"##),
        ("invalid-path", r##"Vale failitee."##),
    ],
}
}

/// Language ID: et;
/// Map name: "set_md";
/// Description: eesti, ladina, Eesti;
pub(super) const fn get_et_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mMuudetud sisu ** ei salvestata [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m, kuna [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ei kutsutud.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mUus väärtus[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mVale failitee.[0m"##),
    ],
}
}

/// Language ID: et;
/// Map name: "get";
/// Description: eesti, ladina, Eesti;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Sihtvorming");
/// ```
pub(super) const fn get_et_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Sihtvorming"##),
        ("src-fmt", r##"Lähtefaili vorming"##),
    ],
}
}

/// et: eesti, ladina, Eesti
pub(super) const fn get_et_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_et_map_conversion),
        ("get", get_et_map_get),
        ("set_md", get_et_map_set_md),
        ("set", get_et_map_set),
        ("conversion_md", get_et_map_conversion_md),
    ],
}
}

/// Language ID: eu;
/// Map name: "conversion";
/// Description: euskara, latinoa, Espainia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Ezin izan da formatua automatikoki hautematen.Mesedez, zehaztu eskuz.");
/// ```
pub(super) const fn get_eu_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Fitxategia ez da baliozko `Json 1.0` formatua, `json5` ... bezala analizatu nahian"##),
        ("not-included", r##"Binary ** honek ez du ** dagokion formatuaren bihurketa funtzionalitatea barne hartzen.
Ezaugarri garrantzitsua zure `Cargo.toml` eta birplanteatuan gaitu behar duzu.
Software honek dagokion funtzionaltasuna jasotzen ez badu, bidali arazo bat."##),
        ("currently-supported", r##"Onartutako formatuen zerrenda:"##),
        ("unsupported", r##"Onartu gabeko formatu bihurketa"##),
        ("auto-detection-failed", r##"Ezin izan da formatua automatikoki hautematen.Mesedez, zehaztu eskuz."##),
        ("not-support-deser-sexp", r##"** ez da onartzen oraindik **: "Lisp s-espresiotik" beste formatu batzuetara bihurtzea"##),
        ("unknown-fmt", r##"Fitxategi formatu ezezaguna"##),
        ("not-saved", r##"Hurrengo edukia ** ez da gordeko ** `--save`  deitzen ez delako."##),
        ("dst", r##"Helmuga fitxategiaren bidea"##),
        ("conv-error", r##"Bihurketa errorea"##),
    ],
}
}

/// Language ID: eu;
/// Map name: "conversion_md";
/// Description: euskara, latinoa, Espainia;
pub(super) const fn get_eu_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFitxategia ez da baliozko [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m formatua, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ... bezala analizatu nahian[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinary ** honek ez du ** dagokion formatuaren bihurketa funtzionalitatea barne hartzen.
[48;2;34;34;34m[38;2;255;255;255mEzaugarri garrantzitsua zure [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m eta birplanteatuan gaitu behar duzu.
[48;2;34;34;34m[38;2;255;255;255mSoftware honek dagokion funtzionaltasuna jasotzen ez badu, bidali arazo bat.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mOnartutako formatuen zerrenda:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mOnartu gabeko formatu bihurketa[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mEzin izan da formatua automatikoki hautematen.Mesedez, zehaztu eskuz.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ez da onartzen oraindik [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: "Lisp s-espresiotik" beste formatu batzuetara bihurtzea[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFitxategi formatu ezezaguna[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mHurrengo edukia ** ez da gordeko ** [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m  deitzen ez delako.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mHelmuga fitxategiaren bidea[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mBihurketa errorea[0m"##),
    ],
}
}

/// Language ID: eu;
/// Map name: "set";
/// Description: euskara, latinoa, Espainia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Fitxategi bide baliogabea.");
/// ```
pub(super) const fn get_eu_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Aldatutako edukia ** ez da gordeko * - --sv` ez delako deitzen."##),
        ("new-value", r##"Balio berria"##),
        ("invalid-path", r##"Fitxategi bide baliogabea."##),
    ],
}
}

/// Language ID: eu;
/// Map name: "set_md";
/// Description: euskara, latinoa, Espainia;
pub(super) const fn get_eu_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mAldatutako edukia ** ez da gordeko * - --sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ez delako deitzen.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mBalio berria[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mFitxategi bide baliogabea.[0m"##),
    ],
}
}

/// Language ID: eu;
/// Map name: "get";
/// Description: euskara, latinoa, Espainia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Helmuga formatua");
/// ```
pub(super) const fn get_eu_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Helmuga formatua"##),
        ("src-fmt", r##"Iturburu fitxategiaren formatua"##),
    ],
}
}

/// eu: euskara, latinoa, Espainia
pub(super) const fn get_eu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_eu_map_conversion),
        ("get", get_eu_map_get),
        ("set_md", get_eu_map_set_md),
        ("set", get_eu_map_set),
        ("conversion_md", get_eu_map_conversion_md),
    ],
}
}

/// Language ID: fa;
/// Map name: "conversion";
/// Description: فارسی, عربی, ایران;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "به طور خودکار قالب را تشخیص داد.لطفا به صورت دستی مشخص کنید.");
/// ```
pub(super) const fn get_fa_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"پرونده یک فرمت معتبر `json 1.0` نیست ، و سعی در تجزیه به عنوان `json5` ..."##),
        ("not-included", r##"این باینری ** شامل ** عملکرد تبدیل برای قالب مربوطه نیست.
شما باید ویژگی مربوطه را در `Cargo.toml` و recompile خود فعال کنید.
اگر این نرم افزار عملکرد مربوطه را شامل نمی شود ، لطفاً یک مسئله را ارسال کنید."##),
        ("currently-supported", r##"در حال حاضر لیست فرمت های پشتیبانی شده:"##),
        ("unsupported", r##"تبدیل فرمت پشتیبانی نشده"##),
        ("auto-detection-failed", r##"به طور خودکار قالب را تشخیص داد.لطفا به صورت دستی مشخص کنید."##),
        ("not-support-deser-sexp", r##"** هنوز پشتیبانی نشده است **: تبدیل از `Lisp S-Expression` به «فرمت های دیگر»"##),
        ("unknown-fmt", r##"قالب پرونده ناشناخته"##),
        ("not-saved", r##"محتوای زیر ** ** ذخیره نمی شود زیرا `--save` فراخوانی نشده است."##),
        ("dst", r##"مسیر پرونده مقصد"##),
        ("conv-error", r##"خطای تبدیل"##),
    ],
}
}

/// Language ID: fa;
/// Map name: "conversion_md";
/// Description: فارسی, عربی, ایران;
pub(super) const fn get_fa_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mپرونده یک فرمت معتبر [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m نیست ، و سعی در تجزیه به عنوان [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mاین باینری ** شامل ** عملکرد تبدیل برای قالب مربوطه نیست.
[48;2;34;34;34m[38;2;255;255;255mشما باید ویژگی مربوطه را در [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m و recompile خود فعال کنید.
[48;2;34;34;34m[38;2;255;255;255mاگر این نرم افزار عملکرد مربوطه را شامل نمی شود ، لطفاً یک مسئله را ارسال کنید.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mدر حال حاضر لیست فرمت های پشتیبانی شده:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mتبدیل فرمت پشتیبانی نشده[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mبه طور خودکار قالب را تشخیص داد.لطفا به صورت دستی مشخص کنید.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** هنوز پشتیبانی نشده است [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: تبدیل از [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m به «فرمت های دیگر»[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mقالب پرونده ناشناخته[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mمحتوای زیر ** ** ذخیره نمی شود زیرا [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m فراخوانی نشده است.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mمسیر پرونده مقصد[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mخطای تبدیل[0m"##),
    ],
}
}

/// Language ID: fa;
/// Map name: "set";
/// Description: فارسی, عربی, ایران;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "مسیر پرونده نامعتبر است.");
/// ```
pub(super) const fn get_fa_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"محتوای اصلاح شده ** ** ذخیره نمی شود زیرا `-SV" خوانده نشده است."##),
        ("new-value", r##"مقدار جدید"##),
        ("invalid-path", r##"مسیر پرونده نامعتبر است."##),
    ],
}
}

/// Language ID: fa;
/// Map name: "set_md";
/// Description: فارسی, عربی, ایران;
pub(super) const fn get_fa_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mمحتوای اصلاح شده ** ** ذخیره نمی شود زیرا [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m-SV" خوانده نشده است.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mمقدار جدید[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mمسیر پرونده نامعتبر است.[0m"##),
    ],
}
}

/// Language ID: fa;
/// Map name: "get";
/// Description: فارسی, عربی, ایران;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "قالب مقصد");
/// ```
pub(super) const fn get_fa_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"قالب مقصد"##),
        ("src-fmt", r##"قالب پرونده منبع"##),
    ],
}
}

/// fa: فارسی, عربی, ایران
pub(super) const fn get_fa_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_fa_map_conversion),
        ("get", get_fa_map_get),
        ("set_md", get_fa_map_set_md),
        ("set", get_fa_map_set),
        ("conversion_md", get_fa_map_conversion_md),
    ],
}
}

/// Language ID: fi;
/// Map name: "conversion";
/// Description: suomi, latinalainen, Suomi;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Muodon automaattisesti havaitseminen epäonnistui.Määritä manuaalisesti.");
/// ```
pub(super) const fn get_fi_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Tiedosto ei ole kelvollinen `JSON 1.0` -muoto, yrittäen jäsentää nimellä `JSON5`..."##),
        ("not-included", r##"Tämä binaari ** ei sisällä ** asiaankuuluvan muodon muuntamistoimintoja.
Sinun on otettava käyttöön asiaankuuluva ominaisuus `Cargo.toml` -sivustollasi ja käännettävä uudelleen.
Jos tämä ohjelmisto ei sisällä vastaavia toimintoja, lähetä ongelma."##),
        ("currently-supported", r##"Tällä hetkellä tuetut muodot -luettelo:"##),
        ("unsupported", r##"tukemattoman muodon muuntaminen"##),
        ("auto-detection-failed", r##"Muodon automaattisesti havaitseminen epäonnistui.Määritä manuaalisesti."##),
        ("not-support-deser-sexp", r##"** ei ole vielä tuettu **: muuntaminen `Lisp S-Expression` -järjestelmästä `muihin muotoihin`"##),
        ("unknown-fmt", r##"tuntematon tiedostomuoto"##),
        ("not-saved", r##"Seuraavaa sisältöä ** ei ** tallenneta, koska `--save` ei kutsuttu."##),
        ("dst", r##"kohdetiedostopolku"##),
        ("conv-error", r##"muuntamisvirhe"##),
    ],
}
}

/// Language ID: fi;
/// Map name: "conversion_md";
/// Description: suomi, latinalainen, Suomi;
pub(super) const fn get_fi_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mTiedosto ei ole kelvollinen [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m -muoto, yrittäen jäsentää nimellä [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mTämä binaari ** ei sisällä ** asiaankuuluvan muodon muuntamistoimintoja.
[48;2;34;34;34m[38;2;255;255;255mSinun on otettava käyttöön asiaankuuluva ominaisuus [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m -sivustollasi ja käännettävä uudelleen.
[48;2;34;34;34m[38;2;255;255;255mJos tämä ohjelmisto ei sisällä vastaavia toimintoja, lähetä ongelma.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mTällä hetkellä tuetut muodot -luettelo:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mtukemattoman muodon muuntaminen[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mMuodon automaattisesti havaitseminen epäonnistui.Määritä manuaalisesti.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ei ole vielä tuettu [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: muuntaminen [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m -järjestelmästä [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mmuihin muotoihin[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mtuntematon tiedostomuoto[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mSeuraavaa sisältöä ** ei ** tallenneta, koska [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ei kutsuttu.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mkohdetiedostopolku[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mmuuntamisvirhe[0m"##),
    ],
}
}

/// Language ID: fi;
/// Map name: "set";
/// Description: suomi, latinalainen, Suomi;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "virheellinen tiedostopolku.");
/// ```
pub(super) const fn get_fi_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Muokattua sisältöä ** ei ** tallennetta, koska `--sv` ei kutsuttu."##),
        ("new-value", r##"uusi arvo"##),
        ("invalid-path", r##"virheellinen tiedostopolku."##),
    ],
}
}

/// Language ID: fi;
/// Map name: "set_md";
/// Description: suomi, latinalainen, Suomi;
pub(super) const fn get_fi_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mMuokattua sisältöä ** ei ** tallennetta, koska [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ei kutsuttu.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255muusi arvo[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mvirheellinen tiedostopolku.[0m"##),
    ],
}
}

/// Language ID: fi;
/// Map name: "get";
/// Description: suomi, latinalainen, Suomi;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Kohdemuoto");
/// ```
pub(super) const fn get_fi_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Kohdemuoto"##),
        ("src-fmt", r##"Lähdetiedostomuoto"##),
    ],
}
}

/// fi: suomi, latinalainen, Suomi
pub(super) const fn get_fi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_fi_map_conversion),
        ("get", get_fi_map_get),
        ("set_md", get_fi_map_set_md),
        ("set", get_fi_map_set),
        ("conversion_md", get_fi_map_conversion_md),
    ],
}
}

/// Language ID: fil;
/// Map name: "conversion";
/// Description: Filipino, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Nabigo upang awtomatikong makita ang format.Mangyaring tukuyin nang manu -mano.");
/// ```
pub(super) const fn get_fil_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Ang file ay hindi isang wastong format na `json 1.0`, sinusubukan na i -parse bilang `json5` ..."##),
        ("not-included", r##"Ang binary ** ay hindi kasama ang ** ang pag -andar ng conversion para sa may -katuturang format.
Kailangan mong paganahin ang may -katuturang tampok sa iyong `Cargo.toml` at muling pagsasaayos.
Kung ang software na ito ay hindi kasama ang kaukulang pag -andar, mangyaring magsumite ng isang isyu."##),
        ("currently-supported", r##"Kasalukuyang suportado ang listahan ng mga format:"##),
        ("unsupported", r##"Hindi suportadong format ng conversion"##),
        ("auto-detection-failed", r##"Nabigo upang awtomatikong makita ang format.Mangyaring tukuyin nang manu -mano."##),
        ("not-support-deser-sexp", r##"** hindi pa suportado **: pag-convert mula sa `lisp s-expression` sa` iba pang mga format`"##),
        ("unknown-fmt", r##"hindi kilalang format ng file"##),
        ("not-saved", r##"Ang sumusunod na nilalaman ** ay hindi ** mai-save dahil ang `--save` ay hindi tinawag."##),
        ("dst", r##"patutunguhan na landas ng file"##),
        ("conv-error", r##"error sa conversion"##),
    ],
}
}

/// Language ID: fil;
/// Map name: "conversion_md";
/// Description: Filipino, Latin, Pilipinas;
pub(super) const fn get_fil_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mAng file ay hindi isang wastong format na [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, sinusubukan na i -parse bilang [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mAng binary ** ay hindi kasama ang ** ang pag -andar ng conversion para sa may -katuturang format.
[48;2;34;34;34m[38;2;255;255;255mKailangan mong paganahin ang may -katuturang tampok sa iyong [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m at muling pagsasaayos.
[48;2;34;34;34m[38;2;255;255;255mKung ang software na ito ay hindi kasama ang kaukulang pag -andar, mangyaring magsumite ng isang isyu.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mKasalukuyang suportado ang listahan ng mga format:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mHindi suportadong format ng conversion[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNabigo upang awtomatikong makita ang format.Mangyaring tukuyin nang manu -mano.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** hindi pa suportado [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: pag-convert mula sa [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m sa[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m iba pang mga format[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mhindi kilalang format ng file[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mAng sumusunod na nilalaman ** ay hindi ** mai-save dahil ang [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ay hindi tinawag.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mpatutunguhan na landas ng file[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255merror sa conversion[0m"##),
    ],
}
}

/// Language ID: fil;
/// Map name: "set";
/// Description: Filipino, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "hindi wastong landas ng file.");
/// ```
pub(super) const fn get_fil_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Ang binagong nilalaman ** ay hindi ** mai-save dahil ang `--sv` ay hindi tinawag."##),
        ("new-value", r##"Bagong halaga"##),
        ("invalid-path", r##"hindi wastong landas ng file."##),
    ],
}
}

/// Language ID: fil;
/// Map name: "set_md";
/// Description: Filipino, Latin, Pilipinas;
pub(super) const fn get_fil_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mAng binagong nilalaman ** ay hindi ** mai-save dahil ang [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ay hindi tinawag.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mBagong halaga[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mhindi wastong landas ng file.[0m"##),
    ],
}
}

/// Language ID: fil;
/// Map name: "get";
/// Description: Filipino, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format ng patutunguhan");
/// ```
pub(super) const fn get_fil_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format ng patutunguhan"##),
        ("src-fmt", r##"Format ng mapagkukunan ng mapagkukunan"##),
    ],
}
}

/// fil: Filipino, Latin, Pilipinas
pub(super) const fn get_fil_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_fil_map_conversion),
        ("get", get_fil_map_get),
        ("set_md", get_fil_map_set_md),
        ("set", get_fil_map_set),
        ("conversion_md", get_fil_map_conversion_md),
    ],
}
}

/// Language ID: fr;
/// Map name: "conversion";
/// Description: français, latin, France;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "La détection automatique de format a échoué. Veuillez spécifier manuellement le format.");
/// ```
pub(super) const fn get_fr_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Le fichier n'est pas au format `json 1.0` valide, tentative de conversion en `json5`..."##),
        ("not-included", r##"Ce fichier binaire ne contient pas de fonctionnalités de conversion de format associées.
Vous devez activer les fonctionnalités correspondantes dans votre `Cargo.toml`, puis recompiler votre projet.
Si ce logiciel ne prend pas en charge cette fonctionnalité, veuillez signaler un problème."##),
        ("currently-supported", r##"Liste des formats pris en charge actuellement"##),
        ("unsupported", r##"Conversion de format non prise en charge"##),
        ("auto-detection-failed", r##"La détection automatique de format a échoué. Veuillez spécifier manuellement le format."##),
        ("not-support-deser-sexp", r##"**Non pris en charge pour le moment** : Conversion de `Lisp S-Expression` en `autres formats`"##),
        ("unknown-fmt", r##"Format de fichier inconnu"##),
        ("not-saved", r##"Comme `--save` n'a pas été appelé, les contenus suivants **ne seront pas** enregistrés."##),
        ("dst", r##"Chemin du fichier de destination"##),
        ("conv-error", r##"Erreur de conversion"##),
    ],
}
}

/// Language ID: fr;
/// Map name: "conversion_md";
/// Description: français, latin, France;
pub(super) const fn get_fr_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mLe fichier n'est pas au format [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m valide, tentative de conversion en [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mCe fichier binaire ne contient pas de fonctionnalités de conversion de format associées.
[48;2;34;34;34m[38;2;255;255;255mVous devez activer les fonctionnalités correspondantes dans votre [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, puis recompiler votre projet.
[48;2;34;34;34m[38;2;255;255;255mSi ce logiciel ne prend pas en charge cette fonctionnalité, veuillez signaler un problème.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mListe des formats pris en charge actuellement[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mConversion de format non prise en charge[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mLa détection automatique de format a échoué. Veuillez spécifier manuellement le format.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mNon pris en charge pour le moment[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m : Conversion de [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m en [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mautres formats[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormat de fichier inconnu[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mComme [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m n'a pas été appelé, les contenus suivants [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mne seront pas[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m enregistrés.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mChemin du fichier de destination[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mErreur de conversion[0m"##),
    ],
}
}

/// Language ID: fr;
/// Map name: "get";
/// Description: français, latin, France;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format de destination");
/// ```
pub(super) const fn get_fr_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format de destination"##),
        ("src-fmt", r##"Format du fichier source"##),
    ],
}
}

/// Language ID: fr;
/// Map name: "set";
/// Description: français, latin, France;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Chemin de fichier invalide");
/// ```
pub(super) const fn get_fr_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Comme `--save` n'a pas été appelé, les modifications apportées **ne seront pas** enregistrées."##),
        ("new-value", r##"Nouvelle valeur"##),
        ("invalid-path", r##"Chemin de fichier invalide"##),
    ],
}
}

/// Language ID: fr;
/// Map name: "set_md";
/// Description: français, latin, France;
pub(super) const fn get_fr_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mComme [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m n'a pas été appelé, les modifications apportées [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mne seront pas[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m enregistrées.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNouvelle valeur[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mChemin de fichier invalide[0m"##),
    ],
}
}

/// fr: français, latin, France
pub(super) const fn get_fr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_fr_map_conversion),
        ("get", get_fr_map_get),
        ("set_md", get_fr_map_set_md),
        ("set", get_fr_map_set),
        ("conversion_md", get_fr_map_conversion_md),
    ],
}
}

/// Language ID: fy;
/// Map name: "conversion";
/// Description: Frysk, Latyn, Nederlân;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Mislearre om automatysk it formaat te detektearjen.Spesifisearje asjebleaft asjebleaft.");
/// ```
pub(super) const fn get_fy_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"It bestân is gjin jildich `JSON 1.0` Opmaak, besykje te parse as `json5` ..."##),
        ("not-included", r##"Dizze binêre ** omfettet net ** de konverzje-funksjonaliteit foar it oanbelangjende opmaak.
Jo moatte de oanbelangjende funksje ynskeakelje yn jo `Cargo.toml` en opnij unrepelje.
As dizze software de oerienkommende funksjonaliteit net befettet, dan yntsjinje dan in probleem."##),
        ("currently-supported", r##"Op it stuit stipe formatenlist:"##),
        ("unsupported", r##"Net-stipe formaat konverzje"##),
        ("auto-detection-failed", r##"Mislearre om automatysk it formaat te detektearjen.Spesifisearje asjebleaft asjebleaft."##),
        ("not-support-deser-sexp", r##"** Noch net stipe **: konvertearje fan 'Lisp S-Expression' oan 'oare formaten'"##),
        ("unknown-fmt", r##"Unbekend bestânsformaat"##),
        ("not-saved", r##"De folgjende ynhâld ** Sil net ** wurde opslein, om't `--save 'net wurde neamd."##),
        ("dst", r##"Bestânbestânspaad"##),
        ("conv-error", r##"Conversion Error"##),
    ],
}
}

/// Language ID: fy;
/// Map name: "conversion_md";
/// Description: Frysk, Latyn, Nederlân;
pub(super) const fn get_fy_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mIt bestân is gjin jildich [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m Opmaak, besykje te parse as [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDizze binêre ** omfettet net ** de konverzje-funksjonaliteit foar it oanbelangjende opmaak.
[48;2;34;34;34m[38;2;255;255;255mJo moatte de oanbelangjende funksje ynskeakelje yn jo [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m en opnij unrepelje.
[48;2;34;34;34m[38;2;255;255;255mAs dizze software de oerienkommende funksjonaliteit net befettet, dan yntsjinje dan in probleem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mOp it stuit stipe formatenlist:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNet-stipe formaat konverzje[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mMislearre om automatysk it formaat te detektearjen.Spesifisearje asjebleaft asjebleaft.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Noch net stipe [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: konvertearje fan 'Lisp S-Expression' oan 'oare formaten'[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mUnbekend bestânsformaat[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mDe folgjende ynhâld ** Sil net ** wurde opslein, om't [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save 'net wurde neamd.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mBestânbestânspaad[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mConversion Error[0m"##),
    ],
}
}

/// Language ID: fy;
/// Map name: "set";
/// Description: Frysk, Latyn, Nederlân;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Unjildich bestânspaad.");
/// ```
pub(super) const fn get_fy_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"The Wizige Tweid ** Sil net ** wurde opslein, om't `--v` net neamd waard."##),
        ("new-value", r##"Nije wearde"##),
        ("invalid-path", r##"Unjildich bestânspaad."##),
    ],
}
}

/// Language ID: fy;
/// Map name: "set_md";
/// Description: Frysk, Latyn, Nederlân;
pub(super) const fn get_fy_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mThe Wizige Tweid ** Sil net ** wurde opslein, om't [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--v[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m net neamd waard.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNije wearde[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mUnjildich bestânspaad.[0m"##),
    ],
}
}

/// Language ID: fy;
/// Map name: "get";
/// Description: Frysk, Latyn, Nederlân;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Bestimmingformaat");
/// ```
pub(super) const fn get_fy_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Bestimmingformaat"##),
        ("src-fmt", r##"Boarne-bestânformaat"##),
    ],
}
}

/// fy: Frysk, Latyn, Nederlân
pub(super) const fn get_fy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_fy_map_conversion),
        ("get", get_fy_map_get),
        ("set_md", get_fy_map_set_md),
        ("set", get_fy_map_set),
        ("conversion_md", get_fy_map_conversion_md),
    ],
}
}

/// Language ID: ga;
/// Map name: "conversion";
/// Description: Gaeilge, Laidineach, Éire;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Theip ar an bhformáid a bhrath go huathoibríoch.Sonraigh de láimh le do thoil.");
/// ```
pub(super) const fn get_ga_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Ní formáid bailí `JSON 1.0` atá sa chomhad, ag iarraidh a pharsáil mar `json5` ..."##),
        ("not-included", r##"Ní chuimsíonn an dénártha seo ** an fheidhmiúlacht tiontaithe don fhormáid ábhartha.
Ní mór duit an ghné ábhartha a chumasú i do `Cargo.toml` agus a athchruthú.
Mura gcuimsíonn na bogearraí seo an fheidhmiúlacht chomhfhreagrach, cuir isteach saincheist le do thoil."##),
        ("currently-supported", r##"Liosta formáidí tacaithe faoi láthair:"##),
        ("unsupported", r##"Comhshó formáide gan tacaíocht"##),
        ("auto-detection-failed", r##"Theip ar an bhformáid a bhrath go huathoibríoch.Sonraigh de láimh le do thoil."##),
        ("not-support-deser-sexp", r##"** Gan tacaíocht fós **: ag athrú ó `lisp s-expression` go` formáidí eile `"##),
        ("unknown-fmt", r##"Formáid comhaid anaithnid"##),
        ("not-saved", r##"Ní shábhálfar an t-ábhar seo a leanas ** toisc nár glaodh `--save`."##),
        ("dst", r##"cosán comhad ceann scríbe"##),
        ("conv-error", r##"earráid tiontaithe"##),
    ],
}
}

/// Language ID: ga;
/// Map name: "conversion_md";
/// Description: Gaeilge, Laidineach, Éire;
pub(super) const fn get_ga_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mNí formáid bailí [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m atá sa chomhad, ag iarraidh a pharsáil mar [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mNí chuimsíonn an dénártha seo ** an fheidhmiúlacht tiontaithe don fhormáid ábhartha.
[48;2;34;34;34m[38;2;255;255;255mNí mór duit an ghné ábhartha a chumasú i do [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m agus a athchruthú.
[48;2;34;34;34m[38;2;255;255;255mMura gcuimsíonn na bogearraí seo an fheidhmiúlacht chomhfhreagrach, cuir isteach saincheist le do thoil.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLiosta formáidí tacaithe faoi láthair:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mComhshó formáide gan tacaíocht[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mTheip ar an bhformáid a bhrath go huathoibríoch.Sonraigh de láimh le do thoil.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Gan tacaíocht fós [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: ag athrú ó [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m go[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m formáidí eile [48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormáid comhaid anaithnid[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNí shábhálfar an t-ábhar seo a leanas ** toisc nár glaodh [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mcosán comhad ceann scríbe[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mearráid tiontaithe[0m"##),
    ],
}
}

/// Language ID: ga;
/// Map name: "set";
/// Description: Gaeilge, Laidineach, Éire;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Cosán comhaid neamhbhailí.");
/// ```
pub(super) const fn get_ga_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Ní dhéanfar an t-ábhar modhnaithe ** a shábháil ** toisc nár glaodh `--sv`."##),
        ("new-value", r##"Luach nua"##),
        ("invalid-path", r##"Cosán comhaid neamhbhailí."##),
    ],
}
}

/// Language ID: ga;
/// Map name: "set_md";
/// Description: Gaeilge, Laidineach, Éire;
pub(super) const fn get_ga_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mNí dhéanfar an t-ábhar modhnaithe ** a shábháil ** toisc nár glaodh [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mLuach nua[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mCosán comhaid neamhbhailí.[0m"##),
    ],
}
}

/// Language ID: ga;
/// Map name: "get";
/// Description: Gaeilge, Laidineach, Éire;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Formáid scríbe");
/// ```
pub(super) const fn get_ga_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Formáid scríbe"##),
        ("src-fmt", r##"Formáid comhaid foinse"##),
    ],
}
}

/// ga: Gaeilge, Laidineach, Éire
pub(super) const fn get_ga_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ga_map_conversion),
        ("get", get_ga_map_get),
        ("set_md", get_ga_map_set_md),
        ("set", get_ga_map_set),
        ("conversion_md", get_ga_map_conversion_md),
    ],
}
}

/// Language ID: gd;
/// Map name: "conversion";
/// Description: Gàidhlig, Laideann, An Rìoghachd Aonaichte;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Dh'fhàillig an cruth a lorg gu fèin-ghluasadach.Sònraich le làimh.");
/// ```
pub(super) const fn get_gd_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (4, 6),
        (6, 0),
    ],
    entries: &[
        ("not-included", r##"Chan eil am binary *** seo a 'toirt a-steach ** an comas atharrachaidh airson an cruth iomchaidh.
Feumaidh tu an fheart buntainneach a chomasachadh anns an `Cargo.toml` agus rempost.
Mura h-eil am bathar-bog seo a 'toirt a-steach an gnìomh-gnìomhachd fhreagarrach, cuiridh e cùis a-steach."##),
        ("auto-detection-failed", r##"Dh'fhàillig an cruth a lorg gu fèin-ghluasadach.Sònraich le làimh."##),
        ("not-support-deser-sexp", r##"** Gun taic fhathast ach **. Ag atharrachadh bho `Lisp S-Consail` ri` cruthan eile"##),
        ("dst", r##"Slighe faidhle ceann-uidhe"##),
        ("invalid-json1.0", r##"Chan e cruth dligheach a th 'anns an fhaidhle, a' feuchainn ri cruth a chuir air adhart mar `JSON5` ..."##),
        ("currently-supported", r##"Liosta Cruthan taice an-dràsta:"##),
        ("not-saved", r##"Cha tèid an susbaint a leanas *** * *** a shàbhaladh oir cha deach `--save` fhaicinn."##),
        ("unknown-fmt", r##"cruth faidhle neo-aithnichte"##),
        ("conv-error", r##"mearachd atharrachaidh"##),
    ],
}
}

/// Language ID: gd;
/// Map name: "conversion_md";
/// Description: Gàidhlig, Laideann, An Rìoghachd Aonaichte;
pub(super) const fn get_gd_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (4, 6),
        (6, 0),
    ],
    entries: &[
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mChan eil am binary *** seo a 'toirt a-steach ** an comas atharrachaidh airson an cruth iomchaidh.
[48;2;34;34;34m[38;2;255;255;255mFeumaidh tu an fheart buntainneach a chomasachadh anns an [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m agus rempost.
[48;2;34;34;34m[38;2;255;255;255mMura h-eil am bathar-bog seo a 'toirt a-steach an gnìomh-gnìomhachd fhreagarrach, cuiridh e cùis a-steach.[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mDh'fhàillig an cruth a lorg gu fèin-ghluasadach.Sònraich le làimh.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Gun taic fhathast ach [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m. Ag atharrachadh bho [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Consail[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ri[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m cruthan eile[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mSlighe faidhle ceann-uidhe[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mChan e cruth dligheach a th 'anns an fhaidhle, a' feuchainn ri cruth a chuir air adhart mar [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLiosta Cruthan taice an-dràsta:[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mCha tèid an susbaint a leanas *** * *** a shàbhaladh oir cha deach [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m fhaicinn.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mcruth faidhle neo-aithnichte[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mmearachd atharrachaidh[0m"##),
    ],
}
}

/// Language ID: gd;
/// Map name: "set";
/// Description: Gàidhlig, Laideann, An Rìoghachd Aonaichte;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Slighe faidhle neo-dhligheach.");
/// ```
pub(super) const fn get_gd_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Cha tèid an susbaint atharraichte ** *** a shàbhaladh oir cha deach `--sv` a ghairm."##),
        ("new-value", r##"Luach ùr"##),
        ("invalid-path", r##"Slighe faidhle neo-dhligheach."##),
    ],
}
}

/// Language ID: gd;
/// Map name: "set_md";
/// Description: Gàidhlig, Laideann, An Rìoghachd Aonaichte;
pub(super) const fn get_gd_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mCha tèid an susbaint atharraichte ** *** a shàbhaladh oir cha deach [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m a ghairm.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mLuach ùr[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mSlighe faidhle neo-dhligheach.[0m"##),
    ],
}
}

/// Language ID: gd;
/// Map name: "get";
/// Description: Gàidhlig, Laideann, An Rìoghachd Aonaichte;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Cruth ceann-uidhe");
/// ```
pub(super) const fn get_gd_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Cruth ceann-uidhe"##),
        ("src-fmt", r##"Cruth faidhle stòr"##),
    ],
}
}

/// gd: Gàidhlig, Laideann, An Rìoghachd Aonaichte
pub(super) const fn get_gd_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_gd_map_conversion),
        ("get", get_gd_map_get),
        ("set_md", get_gd_map_set_md),
        ("set", get_gd_map_set),
        ("conversion_md", get_gd_map_conversion_md),
    ],
}
}

/// Language ID: gl;
/// Map name: "conversion";
/// Description: galego, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "non puido detectar automaticamente o formato.Especifique manualmente.");
/// ```
pub(super) const fn get_gl_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"O ficheiro non é un formato válido `json 1.0`, intentando analizar como `json5` ..."##),
        ("not-included", r##"Este binario ** non inclúe ** a funcionalidade de conversión para o formato pertinente.
Debe habilitar a característica relevante no seu `Cargo.toml` e recompilar.
Se este software non inclúe a funcionalidade correspondente, envíe un problema."##),
        ("currently-supported", r##"Lista de formatos compatibles actualmente:"##),
        ("unsupported", r##"Conversión de formato non compatible"##),
        ("auto-detection-failed", r##"non puido detectar automaticamente o formato.Especifique manualmente."##),
        ("not-support-deser-sexp", r##"** aínda non soportado **: convertendo de `lisp s-expression` a` outros formatos ""##),
        ("unknown-fmt", r##"formato de ficheiro descoñecido"##),
        ("not-saved", r##"Non se gardará o seguinte contido ** porque non se chamou `--save`."##),
        ("dst", r##"ruta do ficheiro de destino"##),
        ("conv-error", r##"erro de conversión"##),
    ],
}
}

/// Language ID: gl;
/// Map name: "conversion_md";
/// Description: galego, latino, España;
pub(super) const fn get_gl_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mO ficheiro non é un formato válido [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, intentando analizar como [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mEste binario ** non inclúe ** a funcionalidade de conversión para o formato pertinente.
[48;2;34;34;34m[38;2;255;255;255mDebe habilitar a característica relevante no seu [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m e recompilar.
[48;2;34;34;34m[38;2;255;255;255mSe este software non inclúe a funcionalidade correspondente, envíe un problema.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLista de formatos compatibles actualmente:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mConversión de formato non compatible[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mnon puido detectar automaticamente o formato.Especifique manualmente.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** aínda non soportado [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: convertendo de [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m a[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m outros formatos "[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mformato de ficheiro descoñecido[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNon se gardará o seguinte contido ** porque non se chamou [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mruta do ficheiro de destino[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255merro de conversión[0m"##),
    ],
}
}

/// Language ID: gl;
/// Map name: "set";
/// Description: galego, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ruta de ficheiro non válida.");
/// ```
pub(super) const fn get_gl_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"O contido modificado ** non se gardará ** porque non se chamou `--sv`."##),
        ("new-value", r##"novo valor"##),
        ("invalid-path", r##"ruta de ficheiro non válida."##),
    ],
}
}

/// Language ID: gl;
/// Map name: "set_md";
/// Description: galego, latino, España;
pub(super) const fn get_gl_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mO contido modificado ** non se gardará ** porque non se chamou [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnovo valor[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mruta de ficheiro non válida.[0m"##),
    ],
}
}

/// Language ID: gl;
/// Map name: "get";
/// Description: galego, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Formato de destino");
/// ```
pub(super) const fn get_gl_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Formato de destino"##),
        ("src-fmt", r##"Formato de ficheiro de orixe"##),
    ],
}
}

/// gl: galego, latino, España
pub(super) const fn get_gl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_gl_map_conversion),
        ("get", get_gl_map_get),
        ("set_md", get_gl_map_set_md),
        ("set", get_gl_map_set),
        ("conversion_md", get_gl_map_conversion_md),
    ],
}
}

/// Language ID: gu;
/// Map name: "conversion";
/// Description: ગુજરાતી, ગુજરાતી, ભારત;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ફોર\u{acd}મ\u{ac7}ટન\u{ac7} આપમ\u{ac7}ળ\u{ac7} શોધવામા\u{a82} નિષ\u{acd}ફળ.ક\u{ac3}પા કરીન\u{ac7} મ\u{ac7}ન\u{acd}ય\u{ac1}અલી સ\u{acd}પષ\u{acd}ટ કરો.");
/// ```
pub(super) const fn get_gu_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"ફાઇલ માન્ય નથી `json 1.0` ફોર્મેટ, `json5` તરીકે વિશ્લેષણ કરવાનો પ્રયાસ કરી રહ્યો છે ..."##),
        ("not-included", r##"આ દ્વિસંગી ** સંબંધિત ફોર્મેટ માટે રૂપાંતર કાર્યક્ષમતા ** શામેલ નથી.
તમારે તમારા `કાર્ગો.ટ om મલ અને ફરીથી કમ્પાઇલમાં સંબંધિત સુવિધાને સક્ષમ કરવાની જરૂર છે.
જો આ સ software ફ્ટવેરને અનુરૂપ વિધેય શામેલ નથી, તો કૃપા કરીને કોઈ મુદ્દો સબમિટ કરો."##),
        ("currently-supported", r##"હાલમાં સપોર્ટેડ ફોર્મેટ્સ સૂચિ:"##),
        ("unsupported", r##"અસમર્થિત ફોર્મેટ રૂપાંતર"##),
        ("auto-detection-failed", r##"ફોર્મેટને આપમેળે શોધવામાં નિષ્ફળ.કૃપા કરીને મેન્યુઅલી સ્પષ્ટ કરો."##),
        ("not-support-deser-sexp", r##"** હજી સુધી સપોર્ટેડ નથી **: `લિસ્પ એસ-એક્સપ્રેસન` થી` અન્ય બંધારણોમાં રૂપાંતરિત કરવું"##),
        ("unknown-fmt", r##"અજ્ unknown ાત ફાઇલ ફોર્મેટ"##),
        ("not-saved", r##"નીચેની સામગ્રી સાચવવામાં આવશે નહીં કારણ કે `--save` કહેવામાં આવતું ન હતું."##),
        ("dst", r##"ગંતવ્ય ફાઇલ પાથ"##),
        ("conv-error", r##"રૂપાંતર ભૂલ"##),
    ],
}
}

/// Language ID: gu;
/// Map name: "conversion_md";
/// Description: ગુજરાતી, ગુજરાતી, ભારત;
pub(super) const fn get_gu_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mફાઇલ માન્ય નથી [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ફોર્મેટ, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m તરીકે વિશ્લેષણ કરવાનો પ્રયાસ કરી રહ્યો છે ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mઆ દ્વિસંગી ** સંબંધિત ફોર્મેટ માટે રૂપાંતર કાર્યક્ષમતા ** શામેલ નથી.
[48;2;34;34;34m[38;2;255;255;255mતમારે તમારા [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mકાર્ગો.ટ om મલ અને ફરીથી કમ્પાઇલમાં સંબંધિત સુવિધાને સક્ષમ કરવાની જરૂર છે.
[48;2;34;34;34m[38;2;0;255;255mજો આ સ software ફ્ટવેરને અનુરૂપ વિધેય શામેલ નથી, તો કૃપા કરીને કોઈ મુદ્દો સબમિટ કરો.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mહાલમાં સપોર્ટેડ ફોર્મેટ્સ સૂચિ:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mઅસમર્થિત ફોર્મેટ રૂપાંતર[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mફોર્મેટને આપમેળે શોધવામાં નિષ્ફળ.કૃપા કરીને મેન્યુઅલી સ્પષ્ટ કરો.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** હજી સુધી સપોર્ટેડ નથી [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mલિસ્પ એસ-એક્સપ્રેસન[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m થી[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m અન્ય બંધારણોમાં રૂપાંતરિત કરવું[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mઅજ્ unknown ાત ફાઇલ ફોર્મેટ[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mનીચેની સામગ્રી સાચવવામાં આવશે નહીં કારણ કે [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m કહેવામાં આવતું ન હતું.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mગંતવ્ય ફાઇલ પાથ[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mરૂપાંતર ભૂલ[0m"##),
    ],
}
}

/// Language ID: gu;
/// Map name: "set";
/// Description: ગુજરાતી, ગુજરાતી, ભારત;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "અમાન\u{acd}ય ફાઇલ પાથ.");
/// ```
pub(super) const fn get_gu_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"સુધારેલી સામગ્રી ** ** સાચવશે નહીં કારણ કે `--એસવી` કહેવાતા ન હતા."##),
        ("new-value", r##"નવું મૂલ્ય"##),
        ("invalid-path", r##"અમાન્ય ફાઇલ પાથ."##),
    ],
}
}

/// Language ID: gu;
/// Map name: "set_md";
/// Description: ગુજરાતી, ગુજરાતી, ભારત;
pub(super) const fn get_gu_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mસુધારેલી સામગ્રી ** ** સાચવશે નહીં કારણ કે [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--એસવી[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m કહેવાતા ન હતા.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mનવું મૂલ્ય[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mઅમાન્ય ફાઇલ પાથ.[0m"##),
    ],
}
}

/// Language ID: gu;
/// Map name: "get";
/// Description: ગુજરાતી, ગુજરાતી, ભારત;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ગ\u{a82}તવ\u{acd}ય -ફોર\u{acd}મ\u{ac7}ટ");
/// ```
pub(super) const fn get_gu_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ગંતવ્ય -ફોર્મેટ"##),
        ("src-fmt", r##"સ્ત્રોત ફાઇલ ફોર્મેટ"##),
    ],
}
}

/// gu: ગુજરાતી, ગુજરાતી, ભારત
pub(super) const fn get_gu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_gu_map_conversion),
        ("get", get_gu_map_get),
        ("set_md", get_gu_map_set_md),
        ("set", get_gu_map_set),
        ("conversion_md", get_gu_map_conversion_md),
    ],
}
}

/// Language ID: ha;
/// Map name: "conversion";
/// Description: Hausa, Latin, Nijeriya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Ba a yi nasarar gano tsarin ba.Da fatan za a saka hannu da hannu.");
/// ```
pub(super) const fn get_ha_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** Ba a tallafawa ba tukuna **: Canza daga 'FursP S-magana' zuwa 'sauran nau'i`"##),
        ("unknown-fmt", r##"Tsarin fayil ɗin da ba a sani ba"##),
        ("currently-supported", r##"A halin yanzu ana tallafawa jerin abubuwan tsari:"##),
        ("auto-detection-failed", r##"Ba a yi nasarar gano tsarin ba.Da fatan za a saka hannu da hannu."##),
        ("not-included", r##"Wannan binary ** bai haɗa da aikin canjin ba don tsarin da ya dace.
Kuna buƙatar kunna fasalin da ya dace a cikin 'Cargo.toml` da kuma sake sakewa.
Idan wannan software ɗin ba ya haɗa da ayyukan da ya dace, don Allah a gabatar da batun."##),
        ("conv-error", r##"Kuskure"##),
        ("dst", r##"Hanyar fayil mai zuwa"##),
        ("unsupported", r##"Canje-canje na tsari"##),
        ("invalid-json1.0", r##"Fayil ba ingantacce bane 'Tsarin 1.0`, ƙoƙarin yin parese a matsayin' json5` ..."##),
    ],
}
}

/// Language ID: ha;
/// Map name: "conversion_md";
/// Description: Hausa, Latin, Nijeriya;
pub(super) const fn get_ha_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Ba a tallafawa ba tukuna [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Canza daga 'FursP S-magana' zuwa 'sauran nau'i[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mTsarin fayil ɗin da ba a sani ba[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mA halin yanzu ana tallafawa jerin abubuwan tsari:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mBa a yi nasarar gano tsarin ba.Da fatan za a saka hannu da hannu.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mWannan binary ** bai haɗa da aikin canjin ba don tsarin da ya dace.
[48;2;34;34;34m[38;2;255;255;255mKuna buƙatar kunna fasalin da ya dace a cikin 'Cargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m da kuma sake sakewa.
[48;2;34;34;34m[38;2;0;255;255mIdan wannan software ɗin ba ya haɗa da ayyukan da ya dace, don Allah a gabatar da batun.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKuskure[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mHanyar fayil mai zuwa[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mCanje-canje na tsari[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFayil ba ingantacce bane 'Tsarin 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m, ƙoƙarin yin parese a matsayin' json5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
    ],
}
}

/// Language ID: ha;
/// Map name: "set";
/// Description: Hausa, Latin, Nijeriya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Hanyar fayil ɗin da ba ta dace ba.");
/// ```
pub(super) const fn get_ha_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"A canza abun ciki ** ba ** don samun ceto saboda `- - ba a kira shi ba."##),
        ("new-value", r##"Sabon darajar"##),
        ("invalid-path", r##"Hanyar fayil ɗin da ba ta dace ba."##),
    ],
}
}

/// Language ID: ha;
/// Map name: "set_md";
/// Description: Hausa, Latin, Nijeriya;
pub(super) const fn get_ha_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mA canza abun ciki ** ba ** don samun ceto saboda [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m- - ba a kira shi ba.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mSabon darajar[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mHanyar fayil ɗin da ba ta dace ba.[0m"##),
    ],
}
}

/// Language ID: ha;
/// Map name: "get";
/// Description: Hausa, Latin, Nijeriya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Tsarin manufa");
/// ```
pub(super) const fn get_ha_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Tsarin manufa"##),
        ("src-fmt", r##"Tsarin fayil na tushe"##),
    ],
}
}

/// ha: Hausa, Latin, Nijeriya
pub(super) const fn get_ha_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ha_map_conversion),
        ("get", get_ha_map_get),
        ("set_md", get_ha_map_set_md),
        ("set", get_ha_map_set),
        ("conversion_md", get_ha_map_conversion_md),
    ],
}
}

/// Language ID: haw;
/// Map name: "conversion";
/// Description: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "invalid-json1.0");
///
/// assert_eq!(msg, "ʻaʻole he faile i kahi o ka `JSON 1.0`");
/// ```
pub(super) const fn get_haw_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"ʻaʻole he faile i kahi o ka `JSON 1.0`"##),
        ("not-support-deser-sexp", r##"ʻaʻole kākoʻoʻiaʻo *** **: E hoʻohuli ana mai ka 'Lisp S-Express"##),
        ("not-included", r##"ʻAʻole pili kēia binary **ʻaʻole e hoʻopili ** i ka hana hoʻololi no keʻano kūpono.
Ponoʻoe e hiki i ka hiʻohiʻona kūpono i kāu 'Cargo.Mtl` a uku hou.
Ināʻaʻole pili kēia polokalamu i ka hana pili i ka hana, eʻoluʻolu e hāʻawi i kahi pilikia."##),
    ],
}
}

/// Language ID: haw;
/// Map name: "conversion_md";
/// Description: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa;
pub(super) const fn get_haw_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mʻaʻole he faile i kahi o ka [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255mʻaʻole kākoʻoʻiaʻo *** [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: E hoʻohuli ana mai ka 'Lisp S-Express[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mʻAʻole pili kēia binary [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mʻaʻole e hoʻopili ** i ka hana hoʻololi no keʻano kūpono.
[48;2;34;34;34m[38;2;249;38;114mPonoʻoe e hiki i ka hiʻohiʻona kūpono i kāu 'Cargo.Mtl[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m a uku hou.
[48;2;34;34;34m[38;2;0;255;255mInāʻaʻole pili kēia polokalamu i ka hana pili i ka hana, eʻoluʻolu e hāʻawi i kahi pilikia.[0m"##),
    ],
}
}

/// Language ID: haw;
/// Map name: "set";
/// Description: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "new-value");
///
/// assert_eq!(msg, "waiwai hou");
/// ```
pub(super) const fn get_haw_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("new-value", r##"waiwai hou"##),
    ],
}
}

/// Language ID: haw;
/// Map name: "set_md";
/// Description: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa;
pub(super) const fn get_haw_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mwaiwai hou[0m"##),
    ],
}
}

/// haw: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa
pub(super) const fn get_haw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("set_md", get_haw_map_set_md),
        ("set", get_haw_map_set),
        ("conversion_md", get_haw_map_conversion_md),
        ("conversion", get_haw_map_conversion),
    ],
}
}

/// Language ID: he;
/// Map name: "conversion";
/// Description: עברית, עברי, ישראל;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "נכשל באיתור אוטומטי של הפורמט.אנא ציין ידנית.");
/// ```
pub(super) const fn get_he_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"הקובץ אינו פורמט 'JSON 1.0' תקף, מנסה לנתח כ- `JSON5` ..."##),
        ("not-included", r##"בינארי זה ** אינו כולל ** פונקציונליות ההמרה לפורמט הרלוונטי.
אתה צריך לאפשר את התכונה הרלוונטית ב'מטען 'שלך ולחזור מחדש.
אם תוכנה זו אינה כוללת את הפונקציונליות המתאימה, אנא הגש בעיה."##),
        ("currently-supported", r##"רשימת פורמטים נתמכת כרגע:"##),
        ("unsupported", r##"המרת פורמט לא נתמך"##),
        ("auto-detection-failed", r##"נכשל באיתור אוטומטי של הפורמט.אנא ציין ידנית."##),
        ("not-support-deser-sexp", r##"** טרם נתמך **: המרה מ "lisp s-expression" ל `פורמטים אחרים""##),
        ("unknown-fmt", r##"פורמט קובץ לא ידוע"##),
        ("not-saved", r##"התוכן הבא ** לא יישמר ** מכיוון ש- `--save` לא נקרא."##),
        ("dst", r##"נתיב קובץ היעד"##),
        ("conv-error", r##"שגיאת המרה"##),
    ],
}
}

/// Language ID: he;
/// Map name: "conversion_md";
/// Description: עברית, עברי, ישראל;
pub(super) const fn get_he_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mהקובץ אינו פורמט 'JSON 1.0' תקף, מנסה לנתח כ- [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mבינארי זה ** אינו כולל ** פונקציונליות ההמרה לפורמט הרלוונטי.
[48;2;34;34;34m[38;2;255;255;255mאתה צריך לאפשר את התכונה הרלוונטית ב'מטען 'שלך ולחזור מחדש.
[48;2;34;34;34m[38;2;255;255;255mאם תוכנה זו אינה כוללת את הפונקציונליות המתאימה, אנא הגש בעיה.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mרשימת פורמטים נתמכת כרגע:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mהמרת פורמט לא נתמך[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mנכשל באיתור אוטומטי של הפורמט.אנא ציין ידנית.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** טרם נתמך [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: המרה מ "lisp s-expression" ל [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mפורמטים אחרים"[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mפורמט קובץ לא ידוע[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mהתוכן הבא ** לא יישמר ** מכיוון ש- [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m לא נקרא.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mנתיב קובץ היעד[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mשגיאת המרה[0m"##),
    ],
}
}

/// Language ID: he;
/// Map name: "set";
/// Description: עברית, עברי, ישראל;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "נתיב קובץ לא חוקי.");
/// ```
pub(super) const fn get_he_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"התוכן שהשתנה ** לא יישמר ** מכיוון ש- `--sv` לא נקרא."##),
        ("new-value", r##"ערך חדש"##),
        ("invalid-path", r##"נתיב קובץ לא חוקי."##),
    ],
}
}

/// Language ID: he;
/// Map name: "set_md";
/// Description: עברית, עברי, ישראל;
pub(super) const fn get_he_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mהתוכן שהשתנה ** לא יישמר ** מכיוון ש- [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m לא נקרא.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mערך חדש[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mנתיב קובץ לא חוקי.[0m"##),
    ],
}
}

/// Language ID: he;
/// Map name: "get";
/// Description: עברית, עברי, ישראל;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "פורמט יעד");
/// ```
pub(super) const fn get_he_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"פורמט יעד"##),
        ("src-fmt", r##"פורמט קובץ מקור"##),
    ],
}
}

/// he: עברית, עברי, ישראל
pub(super) const fn get_he_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_he_map_conversion),
        ("get", get_he_map_get),
        ("set_md", get_he_map_set_md),
        ("set", get_he_map_set),
        ("conversion_md", get_he_map_conversion_md),
    ],
}
}

/// Language ID: hi;
/// Map name: "conversion";
/// Description: हिन्दी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "स\u{94d}वचालित र\u{942}प स\u{947} फ\u{93c}ॉर\u{94d}म\u{947}ट का पता नही\u{902} चला। क\u{943}पया म\u{948}न\u{94d}य\u{941}अली म\u{947}\u{902} निर\u{94d}दिष\u{94d}ट कर\u{947}\u{902}।");
/// ```
pub(super) const fn get_hi_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"फ़ाइल `json 1.0` स्वरूप अमान्य है, `json5` के रूप में विश्लेषण का प्रयास कर रहे हैं..."##),
        ("not-included", r##"इस बाइनरी में अनुच्छेद के लिए रूपांतरण कार्यक्षमता शामिल नहीं है।
आपको अपने `Cargo.toml` में उचित सुविधा को सक्षम करना और पुनर्संचालित करना होगा।
यदि इस सॉफ्टवेयर में संबंधित कार्यक्षमता शामिल नहीं है, तो कृपया एक मुद्दा सबमिट करें।"##),
        ("currently-supported", r##"वर्तमान में समर्थित स्वरूप सूची:"##),
        ("unsupported", r##"असमर्थित स्वरूप कनवर्ट"##),
        ("auto-detection-failed", r##"स्वचालित रूप से फ़ॉर्मेट का पता नहीं चला। कृपया मैन्युअली में निर्दिष्ट करें।"##),
        ("not-support-deser-sexp", r##"**अभी तक समर्थित नहीं**: `Lisp S-Expression` से `अन्य स्वरूपों` में रूपांतरण करना।"##),
        ("unknown-fmt", r##"अज्ञात फ़ाइल स्वरूप"##),
        ("not-saved", r##"`--save` को बुलाया नहीं गया था, अत: निम्नलिखित सामग्री **बचाई नहीं जाएगी**।"##),
        ("dst", r##"गंतव्य फाइल पथ"##),
        ("conv-error", r##"रूपांतरण त्रुटि"##),
    ],
}
}

/// Language ID: hi;
/// Map name: "conversion_md";
/// Description: हिन्दी, देवनागरी, भारत;
pub(super) const fn get_hi_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mफ़ाइल [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m स्वरूप अमान्य है, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m के रूप में विश्लेषण का प्रयास कर रहे हैं...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mइस बाइनरी में अनुच्छेद के लिए रूपांतरण कार्यक्षमता शामिल नहीं है।
[48;2;34;34;34m[38;2;255;255;255mआपको अपने [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m में उचित सुविधा को सक्षम करना और पुनर्संचालित करना होगा।
[48;2;34;34;34m[38;2;255;255;255mयदि इस सॉफ्टवेयर में संबंधित कार्यक्षमता शामिल नहीं है, तो कृपया एक मुद्दा सबमिट करें।[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mवर्तमान में समर्थित स्वरूप सूची:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mअसमर्थित स्वरूप कनवर्ट[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mस्वचालित रूप से फ़ॉर्मेट का पता नहीं चला। कृपया मैन्युअली में निर्दिष्ट करें।[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mअभी तक समर्थित नहीं[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m से [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mअन्य स्वरूपों[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m में रूपांतरण करना।[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mअज्ञात फ़ाइल स्वरूप[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m को बुलाया नहीं गया था, अत: निम्नलिखित सामग्री [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mबचाई नहीं जाएगी[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m।[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mगंतव्य फाइल पथ[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mरूपांतरण त्रुटि[0m"##),
    ],
}
}

/// Language ID: hi;
/// Map name: "set";
/// Description: हिन्दी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "अमान\u{94d}य फ\u{93c}ाइल पथ।");
/// ```
pub(super) const fn get_hi_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"`--sv` को बुलाया नहीं गया था, अत: संशोधित सामग्री **बचाई नहीं जाएगी**।"##),
        ("new-value", r##"नई मूल्य"##),
        ("invalid-path", r##"अमान्य फ़ाइल पथ।"##),
    ],
}
}

/// Language ID: hi;
/// Map name: "set_md";
/// Description: हिन्दी, देवनागरी, भारत;
pub(super) const fn get_hi_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m को बुलाया नहीं गया था, अत: संशोधित सामग्री [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mबचाई नहीं जाएगी[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m।[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mनई मूल्य[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mअमान्य फ़ाइल पथ।[0m"##),
    ],
}
}

/// Language ID: hi;
/// Map name: "get";
/// Description: हिन्दी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ग\u{902}तव\u{94d}य प\u{94d}रार\u{942}प");
/// ```
pub(super) const fn get_hi_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"गंतव्य प्रारूप"##),
        ("src-fmt", r##"स्रोत संचिका प्रारूप"##),
    ],
}
}

/// hi: हिन्दी, देवनागरी, भारत
pub(super) const fn get_hi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_hi_map_conversion),
        ("get", get_hi_map_get),
        ("set_md", get_hi_map_set_md),
        ("set", get_hi_map_set),
        ("conversion_md", get_hi_map_conversion_md),
    ],
}
}

/// Language ID: hr;
/// Map name: "conversion";
/// Description: hrvatski, latinica, Hrvatska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "nije uspio automatski otkriti format.Molimo odredite ručno.");
/// ```
pub(super) const fn get_hr_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Datoteka nije valjani `json 1.0` format, pokušavajući raščlaniti kao `json5` ..."##),
        ("not-included", r##"Ovaj binarni ** ne uključuje ** funkcionalnost pretvorbe za relevantni format.
Morate omogućiti relevantnu značajku u svom `Cargo.toml` i prekomponi.
Ako ovaj softver ne uključuje odgovarajuću funkcionalnost, pošaljite problem."##),
        ("currently-supported", r##"Trenutno podržani popis formata:"##),
        ("unsupported", r##"Nepodržana konverzija formata"##),
        ("auto-detection-failed", r##"nije uspio automatski otkriti format.Molimo odredite ručno."##),
        ("not-support-deser-sexp", r##"** Još nije podržano **: Pretvaranje iz `lisp s-ekspresije` u` ostale formate`"##),
        ("unknown-fmt", r##"Nepoznati format datoteke"##),
        ("not-saved", r##"Sljedeći sadržaj ** neće biti spreman jer `--save` nije pozvan."##),
        ("dst", r##"Put odredišne datoteke"##),
        ("conv-error", r##"Pogreška pretvorbe"##),
    ],
}
}

/// Language ID: hr;
/// Map name: "conversion_md";
/// Description: hrvatski, latinica, Hrvatska;
pub(super) const fn get_hr_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mDatoteka nije valjani [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m format, pokušavajući raščlaniti kao [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mOvaj binarni ** ne uključuje ** funkcionalnost pretvorbe za relevantni format.
[48;2;34;34;34m[38;2;255;255;255mMorate omogućiti relevantnu značajku u svom [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m i prekomponi.
[48;2;34;34;34m[38;2;255;255;255mAko ovaj softver ne uključuje odgovarajuću funkcionalnost, pošaljite problem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mTrenutno podržani popis formata:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNepodržana konverzija formata[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mnije uspio automatski otkriti format.Molimo odredite ručno.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Još nije podržano [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Pretvaranje iz [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-ekspresije[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m u[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ostale formate[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNepoznati format datoteke[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mSljedeći sadržaj ** neće biti spreman jer [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nije pozvan.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mPut odredišne datoteke[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mPogreška pretvorbe[0m"##),
    ],
}
}

/// Language ID: hr;
/// Map name: "set";
/// Description: hrvatski, latinica, Hrvatska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Nevažeći put datoteke.");
/// ```
pub(super) const fn get_hr_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Modificirani sadržaj ** neće biti spreman jer `--sv` nije pozvan."##),
        ("new-value", r##"Nova vrijednost"##),
        ("invalid-path", r##"Nevažeći put datoteke."##),
    ],
}
}

/// Language ID: hr;
/// Map name: "set_md";
/// Description: hrvatski, latinica, Hrvatska;
pub(super) const fn get_hr_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mModificirani sadržaj ** neće biti spreman jer [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nije pozvan.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNova vrijednost[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNevažeći put datoteke.[0m"##),
    ],
}
}

/// Language ID: hr;
/// Map name: "get";
/// Description: hrvatski, latinica, Hrvatska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Odredišni format");
/// ```
pub(super) const fn get_hr_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Odredišni format"##),
        ("src-fmt", r##"Format izvorne datoteke"##),
    ],
}
}

/// hr: hrvatski, latinica, Hrvatska
pub(super) const fn get_hr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_hr_map_conversion),
        ("get", get_hr_map_get),
        ("set_md", get_hr_map_set_md),
        ("set", get_hr_map_set),
        ("conversion_md", get_hr_map_conversion_md),
    ],
}
}

/// Language ID: ht;
/// Map name: "conversion";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "echwe pou pou otomatikman detekte fòma an.Tanpri presize manyèlman.");
/// ```
pub(super) const fn get_ht_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Dosye a se pa yon valab `JSON 1.0` fòma, ap eseye analize kòm `JSON5` ..."##),
        ("not-included", r##"Sa a binè ** pa gen ladan ** fonksyonalite a konvèsyon pou fòma a ki enpòtan.
Ou bezwen pèmèt karakteristik ki enpòtan nan `Cargo.toml 'ou ak recompile.
Si lojisyèl sa a pa enkli fonksyonalite ki koresponn lan, tanpri soumèt yon pwoblèm."##),
        ("currently-supported", r##"Lis Fòma Kounye a Sipòte:"##),
        ("unsupported", r##"konvèsyon fòma ki pa sipòte"##),
        ("auto-detection-failed", r##"echwe pou pou otomatikman detekte fòma an.Tanpri presize manyèlman."##),
        ("not-support-deser-sexp", r##"** pa sipòte ankò **: konvèti soti nan `Lisp s-ekspresyon` a` lòt fòma`"##),
        ("unknown-fmt", r##"fòma dosye enkoni"##),
        ("not-saved", r##"Kontni sa yo ** pa pral ** dwe sove paske `--save` pa te rele."##),
        ("dst", r##"chemen dosye destinasyon"##),
        ("conv-error", r##"erè konvèsyon"##),
    ],
}
}

/// Language ID: ht;
/// Map name: "conversion_md";
pub(super) const fn get_ht_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mDosye a se pa yon valab [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m fòma, ap eseye analize kòm [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mSa a binè ** pa gen ladan ** fonksyonalite a konvèsyon pou fòma a ki enpòtan.
[48;2;34;34;34m[38;2;255;255;255mOu bezwen pèmèt karakteristik ki enpòtan nan [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml 'ou ak recompile.
[48;2;34;34;34m[38;2;0;255;255mSi lojisyèl sa a pa enkli fonksyonalite ki koresponn lan, tanpri soumèt yon pwoblèm.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLis Fòma Kounye a Sipòte:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mkonvèsyon fòma ki pa sipòte[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mechwe pou pou otomatikman detekte fòma an.Tanpri presize manyèlman.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** pa sipòte ankò [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: konvèti soti nan [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp s-ekspresyon[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m a[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m lòt fòma[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mfòma dosye enkoni[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mKontni sa yo ** pa pral ** dwe sove paske [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m pa te rele.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mchemen dosye destinasyon[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255merè konvèsyon[0m"##),
    ],
}
}

/// Language ID: ht;
/// Map name: "set";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "chemen dosye valab.");
/// ```
pub(super) const fn get_ht_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"kontni an modifye ** pa pral ** dwe sove paske `--v" pa te rele."##),
        ("new-value", r##"Nouvo valè"##),
        ("invalid-path", r##"chemen dosye valab."##),
    ],
}
}

/// Language ID: ht;
/// Map name: "set_md";
pub(super) const fn get_ht_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mkontni an modifye ** pa pral ** dwe sove paske [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--v" pa te rele.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNouvo valè[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mchemen dosye valab.[0m"##),
    ],
}
}

/// Language ID: ht;
/// Map name: "get";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Fòma destinasyon");
/// ```
pub(super) const fn get_ht_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Fòma destinasyon"##),
        ("src-fmt", r##"Fòma dosye sous"##),
    ],
}
}

/// ht: ht-Latn-HT
pub(super) const fn get_ht_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ht_map_conversion),
        ("get", get_ht_map_get),
        ("set_md", get_ht_map_set_md),
        ("set", get_ht_map_set),
        ("conversion_md", get_ht_map_conversion_md),
    ],
}
}

/// Language ID: hu;
/// Map name: "conversion";
/// Description: magyar, Latin, Magyarország;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "nem sikerült automatikusan észlelni a formátumot.Kérjük, adja meg manuálisan.");
/// ```
pub(super) const fn get_hu_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"A fájl nem érvényes „JSON 1.0” formátum, amely megpróbálja elemezni `JSON5 '..."##),
        ("not-included", r##"Ez a bináris ** nem tartalmazza a ** konverziós funkcionalitást a vonatkozó formátumhoz.
Engedélyeznie kell a „Cargo.toml” releváns funkcióját, és újra kell készítenie.
Ha ez a szoftver nem tartalmazza a megfelelő funkciót, kérjük, nyújtson be egy problémát."##),
        ("currently-supported", r##"Jelenleg támogatott formátumok listája:"##),
        ("unsupported", r##"Nem támogatott formátumkonverzió"##),
        ("auto-detection-failed", r##"nem sikerült automatikusan észlelni a formátumot.Kérjük, adja meg manuálisan."##),
        ("not-support-deser-sexp", r##"** Még nem támogatott **: A „LISP S-Expression” -ről „más formátumokra” konvertálás"##),
        ("unknown-fmt", r##"Ismeretlen fájl formátum"##),
        ("not-saved", r##"A következő tartalom ** nem fog menteni, mert a `--save` nem hívták meg."##),
        ("dst", r##"célfájl elérési útja"##),
        ("conv-error", r##"Konverziós hiba"##),
    ],
}
}

/// Language ID: hu;
/// Map name: "conversion_md";
/// Description: magyar, Latin, Magyarország;
pub(super) const fn get_hu_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mA fájl nem érvényes „JSON 1.0” formátum, amely megpróbálja elemezni [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5 '...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mEz a bináris ** nem tartalmazza a ** konverziós funkcionalitást a vonatkozó formátumhoz.
[48;2;34;34;34m[38;2;255;255;255mEngedélyeznie kell a „Cargo.toml” releváns funkcióját, és újra kell készítenie.
[48;2;34;34;34m[38;2;255;255;255mHa ez a szoftver nem tartalmazza a megfelelő funkciót, kérjük, nyújtson be egy problémát.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mJelenleg támogatott formátumok listája:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNem támogatott formátumkonverzió[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mnem sikerült automatikusan észlelni a formátumot.Kérjük, adja meg manuálisan.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Még nem támogatott [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: A „LISP S-Expression” -ről „más formátumokra” konvertálás[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mIsmeretlen fájl formátum[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mA következő tartalom ** nem fog menteni, mert a [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nem hívták meg.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mcélfájl elérési útja[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonverziós hiba[0m"##),
    ],
}
}

/// Language ID: hu;
/// Map name: "set";
/// Description: magyar, Latin, Magyarország;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Érvénytelen fájl elérési útja.");
/// ```
pub(super) const fn get_hu_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"A módosított tartalom ** nem fog menteni, mert a `--sv` nem hívták meg."##),
        ("new-value", r##"új érték"##),
        ("invalid-path", r##"Érvénytelen fájl elérési útja."##),
    ],
}
}

/// Language ID: hu;
/// Map name: "set_md";
/// Description: magyar, Latin, Magyarország;
pub(super) const fn get_hu_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mA módosított tartalom ** nem fog menteni, mert a [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nem hívták meg.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255múj érték[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mÉrvénytelen fájl elérési útja.[0m"##),
    ],
}
}

/// Language ID: hu;
/// Map name: "get";
/// Description: magyar, Latin, Magyarország;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Célformátum");
/// ```
pub(super) const fn get_hu_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Célformátum"##),
        ("src-fmt", r##"Forrásfájl formátum"##),
    ],
}
}

/// hu: magyar, Latin, Magyarország
pub(super) const fn get_hu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_hu_map_conversion),
        ("get", get_hu_map_get),
        ("set_md", get_hu_map_set_md),
        ("set", get_hu_map_set),
        ("conversion_md", get_hu_map_conversion_md),
    ],
}
}

/// Language ID: hy;
/// Map name: "conversion";
/// Description: հայերեն, հայկական, Հայաստան;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Չհաջողվեց ինքնաբերաբար հայտնաբերել ձեւաչափը:Խնդրում ենք նշել ձեռքով:");
/// ```
pub(super) const fn get_hy_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** Դեռեւս չի ապահովվում **. «Lisp S-Express» - ից այլ ձեւաչափեր"##),
        ("unknown-fmt", r##"Անհայտ ֆայլի ձեւաչափ"##),
        ("currently-supported", r##"Ներկայումս օժանդակված ձեւաչափերի ցուցակը."##),
        ("auto-detection-failed", r##"Չհաջողվեց ինքնաբերաբար հայտնաբերել ձեւաչափը:Խնդրում ենք նշել ձեռքով:"##),
        ("not-included", r##"Այս երկուական ** չի ներառում ** համապատասխան ձեւաչափի փոխարկման ֆունկցիոնալությունը:
Դուք պետք է հնարավորություն ունենաք համապատասխան հատկությունը ձեր «Cargo.toml» եւ վերահաշվարկեք:
Եթե այս ծրագիրը չի պարունակում համապատասխան գործառույթ, խնդրում ենք ներկայացնել մի խնդիր:"##),
        ("conv-error", r##"Փոխակերպման սխալ"##),
        ("dst", r##"Նպատակային ֆայլի ուղին"##),
        ("unsupported", r##"Չօգտագործված ձեւաչափի փոխարկում"##),
        ("invalid-json1.0", r##"Ֆայլը վավեր չէ «JSON 1.0` ձեւաչափը, փորձելով վերլուծել« JSON5 »..."##),
    ],
}
}

/// Language ID: hy;
/// Map name: "conversion_md";
/// Description: հայերեն, հայկական, Հայաստան;
pub(super) const fn get_hy_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Դեռեւս չի ապահովվում [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m. «Lisp S-Express» - ից այլ ձեւաչափեր[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mԱնհայտ ֆայլի ձեւաչափ[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mՆերկայումս օժանդակված ձեւաչափերի ցուցակը.[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mՉհաջողվեց ինքնաբերաբար հայտնաբերել ձեւաչափը:Խնդրում ենք նշել ձեռքով:[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mԱյս երկուական ** չի ներառում ** համապատասխան ձեւաչափի փոխարկման ֆունկցիոնալությունը:
[48;2;34;34;34m[38;2;255;255;255mԴուք պետք է հնարավորություն ունենաք համապատասխան հատկությունը ձեր «Cargo.toml» եւ վերահաշվարկեք:
[48;2;34;34;34m[38;2;255;255;255mԵթե այս ծրագիրը չի պարունակում համապատասխան գործառույթ, խնդրում ենք ներկայացնել մի խնդիր:[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mՓոխակերպման սխալ[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mՆպատակային ֆայլի ուղին[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mՉօգտագործված ձեւաչափի փոխարկում[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mՖայլը վավեր չէ «JSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ձեւաչափը, փորձելով վերլուծել« JSON5 »...[0m"##),
    ],
}
}

/// Language ID: hy;
/// Map name: "set";
/// Description: հայերեն, հայկական, Հայաստան;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Անվավեր ֆայլի ուղին:");
/// ```
pub(super) const fn get_hy_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Փոփոխված բովանդակությունը ** չի պահվի, քանի որ «--sv» - ը չի կոչվում:"##),
        ("new-value", r##"Նոր արժեք"##),
        ("invalid-path", r##"Անվավեր ֆայլի ուղին:"##),
    ],
}
}

/// Language ID: hy;
/// Map name: "set_md";
/// Description: հայերեն, հայկական, Հայաստան;
pub(super) const fn get_hy_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mՓոփոխված բովանդակությունը ** չի պահվի, քանի որ «--sv» - ը չի կոչվում:[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mՆոր արժեք[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mԱնվավեր ֆայլի ուղին:[0m"##),
    ],
}
}

/// Language ID: hy;
/// Map name: "get";
/// Description: հայերեն, հայկական, Հայաստան;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Նպատակը ձեւաչափ");
/// ```
pub(super) const fn get_hy_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Նպատակը ձեւաչափ"##),
        ("src-fmt", r##"Աղբյուրի ֆայլի ձեւաչափը"##),
    ],
}
}

/// hy: հայերեն, հայկական, Հայաստան
pub(super) const fn get_hy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_hy_map_conversion),
        ("get", get_hy_map_get),
        ("set_md", get_hy_map_set_md),
        ("set", get_hy_map_set),
        ("conversion_md", get_hy_map_conversion_md),
    ],
}
}

/// Language ID: id;
/// Map name: "conversion";
/// Description: Indonesia, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Gagal mendeteksi format secara otomatis.Harap tentukan secara manual.");
/// ```
pub(super) const fn get_id_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"file bukan format `json 1.0` yang valid, mencoba mengurai sebagai `json5` ..."##),
        ("not-included", r##"Biner ** ini tidak termasuk ** fungsionalitas konversi untuk format yang relevan.
Anda perlu mengaktifkan fitur yang relevan di `kargo.toml` dan kompilasi ulang.
Jika perangkat lunak ini tidak termasuk fungsionalitas yang sesuai, silakan kirimkan masalah."##),
        ("currently-supported", r##"Daftar Format yang Didukung Saat Ini:"##),
        ("unsupported", r##"konversi format yang tidak didukung"##),
        ("auto-detection-failed", r##"Gagal mendeteksi format secara otomatis.Harap tentukan secara manual."##),
        ("not-support-deser-sexp", r##"** Belum didukung **: Mengonversi dari `Lisp S-Expression` ke` format lain`"##),
        ("unknown-fmt", r##"format file yang tidak diketahui"##),
        ("not-saved", r##"konten berikut ** tidak akan ** disimpan karena `--save` tidak dipanggil."##),
        ("dst", r##"jalur file tujuan"##),
        ("conv-error", r##"kesalahan konversi"##),
    ],
}
}

/// Language ID: id;
/// Map name: "conversion_md";
/// Description: Indonesia, Latin, Indonesia;
pub(super) const fn get_id_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mfile bukan format [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m yang valid, mencoba mengurai sebagai [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBiner ** ini tidak termasuk ** fungsionalitas konversi untuk format yang relevan.
[48;2;34;34;34m[38;2;255;255;255mAnda perlu mengaktifkan fitur yang relevan di [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mkargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m dan kompilasi ulang.
[48;2;34;34;34m[38;2;255;255;255mJika perangkat lunak ini tidak termasuk fungsionalitas yang sesuai, silakan kirimkan masalah.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mDaftar Format yang Didukung Saat Ini:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mkonversi format yang tidak didukung[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mGagal mendeteksi format secara otomatis.Harap tentukan secara manual.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Belum didukung [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Mengonversi dari [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ke[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m format lain[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mformat file yang tidak diketahui[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mkonten berikut ** tidak akan ** disimpan karena [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m tidak dipanggil.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mjalur file tujuan[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mkesalahan konversi[0m"##),
    ],
}
}

/// Language ID: id;
/// Map name: "set";
/// Description: Indonesia, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "path file tidak valid.");
/// ```
pub(super) const fn get_id_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"konten yang dimodifikasi ** tidak akan ** disimpan karena `--sv` tidak dipanggil."##),
        ("new-value", r##"nilai baru"##),
        ("invalid-path", r##"path file tidak valid."##),
    ],
}
}

/// Language ID: id;
/// Map name: "set_md";
/// Description: Indonesia, Latin, Indonesia;
pub(super) const fn get_id_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mkonten yang dimodifikasi ** tidak akan ** disimpan karena [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m tidak dipanggil.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnilai baru[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mpath file tidak valid.[0m"##),
    ],
}
}

/// Language ID: id;
/// Map name: "get";
/// Description: Indonesia, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format tujuan");
/// ```
pub(super) const fn get_id_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format tujuan"##),
        ("src-fmt", r##"Format file sumber"##),
    ],
}
}

/// id: Indonesia, Latin, Indonesia
pub(super) const fn get_id_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_id_map_conversion),
        ("get", get_id_map_get),
        ("set_md", get_id_map_set_md),
        ("set", get_id_map_set),
        ("conversion_md", get_id_map_conversion_md),
    ],
}
}

/// Language ID: ig;
/// Map name: "conversion";
/// Description: Igbo, Latin, Naịjịrịa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Agbalaghị aka ịchọpụta usoro ahụ.Biko jiri aka dee ya.");
/// ```
pub(super) const fn get_ig_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"Ntughari njehie"##),
        ("not-support-deser-sexp", r##"**Not supported yet**: converting from `Lisp S-Expression` to `other formats`"##),
        ("dst", r##"Abinced faịlụ"##),
        ("unsupported", r##"Ntughari Usoro akwadoghị"##),
        ("auto-detection-failed", r##"Agbalaghị aka ịchọpụta usoro ahụ.Biko jiri aka dee ya."##),
        ("currently-supported", r##"Ederede ndepụta aha:"##),
        ("not-included", r##"Ọnụọgụ abụọ a na-etinyeghị ** n 'arụmọrụ ntụgharị maka usoro dị mkpa.
Ikwesiri ime ka njirimara dị mkpa na gị 'Cargo.toml' na recompile.
Ọ bụrụ na ngwanrọ a agunyeghị arụmọrụ kwekọrọ na ya, biko nyefee okwu."##),
        ("unknown-fmt", r##"Usoro faịlụ a na-amaghị"##),
    ],
}
}

/// Language ID: ig;
/// Map name: "conversion_md";
/// Description: Igbo, Latin, Naịjịrịa;
pub(super) const fn get_ig_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mNtughari njehie[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mNot supported yet[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: converting from [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m to [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mother formats[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mAbinced faịlụ[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNtughari Usoro akwadoghị[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mAgbalaghị aka ịchọpụta usoro ahụ.Biko jiri aka dee ya.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mEderede ndepụta aha:[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mỌnụọgụ abụọ a na-etinyeghị ** n 'arụmọrụ ntụgharị maka usoro dị mkpa.
[48;2;34;34;34m[38;2;255;255;255mIkwesiri ime ka njirimara dị mkpa na gị 'Cargo.toml' na recompile.
[48;2;34;34;34m[38;2;255;255;255mỌ bụrụ na ngwanrọ a agunyeghị arụmọrụ kwekọrọ na ya, biko nyefee okwu.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mUsoro faịlụ a na-amaghị[0m"##),
    ],
}
}

/// Language ID: ig;
/// Map name: "set";
/// Description: Igbo, Latin, Naịjịrịa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Forthzọ faịlụ ezighi ezi.");
/// ```
pub(super) const fn get_ig_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Agaghị echekwa ọdịnaya ***** Azoputaghi n'ihi na aghi."##),
        ("new-value", r##"Uru ọhụrụ"##),
        ("invalid-path", r##"Forthzọ faịlụ ezighi ezi."##),
    ],
}
}

/// Language ID: ig;
/// Map name: "set_md";
/// Description: Igbo, Latin, Naịjịrịa;
pub(super) const fn get_ig_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mAgaghị echekwa ọdịnaya ***** Azoputaghi n'ihi na aghi.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mUru ọhụrụ[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mForthzọ faịlụ ezighi ezi.[0m"##),
    ],
}
}

/// Language ID: ig;
/// Map name: "get";
/// Description: Igbo, Latin, Naịjịrịa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Usoro ị ga-aga");
/// ```
pub(super) const fn get_ig_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Usoro ị ga-aga"##),
        ("src-fmt", r##"Usoro faili faịlụ"##),
    ],
}
}

/// ig: Igbo, Latin, Naịjịrịa
pub(super) const fn get_ig_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ig_map_conversion),
        ("get", get_ig_map_get),
        ("set_md", get_ig_map_set_md),
        ("set", get_ig_map_set),
        ("conversion_md", get_ig_map_conversion_md),
    ],
}
}

/// Language ID: is;
/// Map name: "conversion";
/// Description: íslenska, latneskt, Ísland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "tókst ekki að greina sniðið sjálfkrafa.Vinsamlegast tilgreindu handvirkt.");
/// ```
pub(super) const fn get_is_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Skráin er ekki gilt `json 1.0` snið, að reyna að flokka sem `json5` ..."##),
        ("not-included", r##"Þessi tvöfaldur ** felur ekki í sér ** umbreytingarvirkni fyrir viðkomandi snið.
Þú verður að virkja viðeigandi eiginleika í `Cargo.Toml` og Rethopile.
Ef þessi hugbúnaður felur ekki í sér samsvarandi virkni, vinsamlegast sendu mál."##),
        ("currently-supported", r##"Nú er studdur sniðslisti:"##),
        ("unsupported", r##"Óstudd umbreyting á sniði"##),
        ("auto-detection-failed", r##"tókst ekki að greina sniðið sjálfkrafa.Vinsamlegast tilgreindu handvirkt."##),
        ("not-support-deser-sexp", r##"** Ekki studd enn **: umbreyta úr `lisp s-expression` í` önnur snið`"##),
        ("unknown-fmt", r##"Óþekkt skráarsnið"##),
        ("not-saved", r##"Eftirfarandi efni ** verður ekki ** vistað vegna þess að `--save` var ekki kallað."##),
        ("dst", r##"Destination File Path"##),
        ("conv-error", r##"umbreytingarvilla"##),
    ],
}
}

/// Language ID: is;
/// Map name: "conversion_md";
/// Description: íslenska, latneskt, Ísland;
pub(super) const fn get_is_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mSkráin er ekki gilt [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m snið, að reyna að flokka sem [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mÞessi tvöfaldur ** felur ekki í sér ** umbreytingarvirkni fyrir viðkomandi snið.
[48;2;34;34;34m[38;2;255;255;255mÞú verður að virkja viðeigandi eiginleika í [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m og Rethopile.
[48;2;34;34;34m[38;2;255;255;255mEf þessi hugbúnaður felur ekki í sér samsvarandi virkni, vinsamlegast sendu mál.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mNú er studdur sniðslisti:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mÓstudd umbreyting á sniði[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mtókst ekki að greina sniðið sjálfkrafa.Vinsamlegast tilgreindu handvirkt.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Ekki studd enn [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: umbreyta úr [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m í[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m önnur snið[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mÓþekkt skráarsnið[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mEftirfarandi efni ** verður ekki ** vistað vegna þess að [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m var ekki kallað.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestination File Path[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mumbreytingarvilla[0m"##),
    ],
}
}

/// Language ID: is;
/// Map name: "set";
/// Description: íslenska, latneskt, Ísland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ógild skráarslóð.");
/// ```
pub(super) const fn get_is_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Breytt efni ** verður ekki ** vistað vegna þess að `--sv` var ekki kallað."##),
        ("new-value", r##"nýtt gildi"##),
        ("invalid-path", r##"Ógild skráarslóð."##),
    ],
}
}

/// Language ID: is;
/// Map name: "set_md";
/// Description: íslenska, latneskt, Ísland;
pub(super) const fn get_is_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mBreytt efni ** verður ekki ** vistað vegna þess að [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m var ekki kallað.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnýtt gildi[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mÓgild skráarslóð.[0m"##),
    ],
}
}

/// Language ID: is;
/// Map name: "get";
/// Description: íslenska, latneskt, Ísland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Áfangastað");
/// ```
pub(super) const fn get_is_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Áfangastað"##),
        ("src-fmt", r##"Upprunalega skráarsnið"##),
    ],
}
}

/// is: íslenska, latneskt, Ísland
pub(super) const fn get_is_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_is_map_conversion),
        ("get", get_is_map_get),
        ("set_md", get_is_map_set_md),
        ("set", get_is_map_set),
        ("conversion_md", get_is_map_conversion_md),
    ],
}
}

/// Language ID: it;
/// Map name: "conversion";
/// Description: italiano, latino, Italia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Impossibile rilevare automaticamente il formato.Si prega di specificare manualmente.");
/// ```
pub(super) const fn get_it_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Il file non è un formato `json 1.0` valido, cercando di analizzare come `json5` ..."##),
        ("not-included", r##"Questo binario ** non include ** la funzionalità di conversione per il formato pertinente.
È necessario abilitare la funzione pertinente nel tuo `Cargo.toml` e ricompilare.
Se questo software non include la funzionalità corrispondente, si prega di inviare un problema."##),
        ("currently-supported", r##"Elenco dei formati attualmente supportati:"##),
        ("unsupported", r##"conversione del formato non supportato"##),
        ("auto-detection-failed", r##"Impossibile rilevare automaticamente il formato.Si prega di specificare manualmente."##),
        ("not-support-deser-sexp", r##"** Non ancora supportato **: conversione da `lisp s-espression` in` altri formati`"##),
        ("unknown-fmt", r##"formato di file sconosciuto"##),
        ("not-saved", r##"Il seguente contenuto ** non verrà risparmiato perché `--save` non è stato chiamato."##),
        ("dst", r##"percorso del file di destinazione"##),
        ("conv-error", r##"Errore di conversione"##),
    ],
}
}

/// Language ID: it;
/// Map name: "conversion_md";
/// Description: italiano, latino, Italia;
pub(super) const fn get_it_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mIl file non è un formato [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m valido, cercando di analizzare come [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mQuesto binario ** non include ** la funzionalità di conversione per il formato pertinente.
[48;2;34;34;34m[38;2;255;255;255mÈ necessario abilitare la funzione pertinente nel tuo [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m e ricompilare.
[48;2;34;34;34m[38;2;255;255;255mSe questo software non include la funzionalità corrispondente, si prega di inviare un problema.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mElenco dei formati attualmente supportati:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mconversione del formato non supportato[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mImpossibile rilevare automaticamente il formato.Si prega di specificare manualmente.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Non ancora supportato [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: conversione da [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-espression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m in[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m altri formati[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mformato di file sconosciuto[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mIl seguente contenuto ** non verrà risparmiato perché [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m non è stato chiamato.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mpercorso del file di destinazione[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mErrore di conversione[0m"##),
    ],
}
}

/// Language ID: it;
/// Map name: "set";
/// Description: italiano, latino, Italia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Percorso del file non valido.");
/// ```
pub(super) const fn get_it_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Il contenuto modificato ** non verrà salvato ** perché `--sv` non è stato chiamato."##),
        ("new-value", r##"Nuovo valore"##),
        ("invalid-path", r##"Percorso del file non valido."##),
    ],
}
}

/// Language ID: it;
/// Map name: "set_md";
/// Description: italiano, latino, Italia;
pub(super) const fn get_it_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mIl contenuto modificato ** non verrà salvato ** perché [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m non è stato chiamato.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNuovo valore[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mPercorso del file non valido.[0m"##),
    ],
}
}

/// Language ID: it;
/// Map name: "get";
/// Description: italiano, latino, Italia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Formato di destinazione");
/// ```
pub(super) const fn get_it_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Formato di destinazione"##),
        ("src-fmt", r##"Formato del file di origine"##),
    ],
}
}

/// it: italiano, latino, Italia
pub(super) const fn get_it_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_it_map_conversion),
        ("get", get_it_map_get),
        ("set_md", get_it_map_set_md),
        ("set", get_it_map_set),
        ("conversion_md", get_it_map_conversion_md),
    ],
}
}

/// Language ID: ja;
/// Map name: "conversion";
/// Description: 日本語, 日本語の文字, 日本;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "自動フォーマット検出が失敗しました。手動で指定してください。");
/// ```
pub(super) const fn get_ja_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"ファイルが `json 1.0` 形式ではありません。`json5`形式として解析を試みています..."##),
        ("not-included", r##"このバイナリには、関連する形式の変換機能が**含まれていません**。
関連する機能を `Cargo.toml` で有効にし、再コンパイルする必要があります。
このソフトウェアに関連機能が含まれていない場合、問題を報告してください。"##),
        ("currently-supported", r##"現在サポートされているフォーマット"##),
        ("unsupported", r##"未対応の形式変換"##),
        ("auto-detection-failed", r##"自動フォーマット検出が失敗しました。手動で指定してください。"##),
        ("not-support-deser-sexp", r##"`Lisp S-Expression` を他の形式に変換することは現在サポートされていません。"##),
        ("unknown-fmt", r##"不明なファイル形式"##),
        ("not-saved", r##"`--save`が呼び出されなかったため、以下の内容は**保存されません**："##),
        ("dst", r##"出力ファイルパス"##),
        ("conv-error", r##"変換エラー"##),
    ],
}
}

/// Language ID: ja;
/// Map name: "conversion_md";
/// Description: 日本語, 日本語の文字, 日本;
pub(super) const fn get_ja_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mファイルが [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 形式ではありません。[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m形式として解析を試みています...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mこのバイナリには、関連する形式の変換機能が[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m含まれていません[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m。
[48;2;34;34;34m[38;2;255;255;255m関連する機能を [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m で有効にし、再コンパイルする必要があります。
[48;2;34;34;34m[38;2;255;255;255mこのソフトウェアに関連機能が含まれていない場合、問題を報告してください。[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255m現在サポートされているフォーマット[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255m未対応の形式変換[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255m自動フォーマット検出が失敗しました。手動で指定してください。[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m を他の形式に変換することは現在サポートされていません。[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255m不明なファイル形式[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mが呼び出されなかったため、以下の内容は[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m保存されません[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m：[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255m出力ファイルパス[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255m変換エラー[0m"##),
    ],
}
}

/// Language ID: ja;
/// Map name: "get";
/// Description: 日本語, 日本語の文字, 日本;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "出力形式");
/// ```
pub(super) const fn get_ja_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"出力形式"##),
        ("src-fmt", r##"ソース形式"##),
    ],
}
}

/// Language ID: ja;
/// Map name: "set";
/// Description: 日本語, 日本語の文字, 日本;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "無効なファイルパス");
/// ```
pub(super) const fn get_ja_map_set() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"無効なファイルパス"##),
        ("new-value", r##"新しい値"##),
        ("unsave-warn", r##"`--save`が呼び出されなかったため、変更内容は**保存されません**。"##),
        ("type", r##"型"##),
    ],
}
}

/// Language ID: ja;
/// Map name: "set_md";
/// Description: 日本語, 日本語の文字, 日本;
pub(super) const fn get_ja_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255m無効なファイルパス[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255m新しい値[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mが呼び出されなかったため、変更内容は[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m保存されません[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m。[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255m型[0m"##),
    ],
}
}

/// ja: 日本語, 日本語の文字, 日本
pub(super) const fn get_ja_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ja_map_conversion),
        ("get", get_ja_map_get),
        ("set_md", get_ja_map_set_md),
        ("set", get_ja_map_set),
        ("conversion_md", get_ja_map_conversion_md),
    ],
}
}

/// Language ID: jw;
/// Map name: "conversion";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Gagal kanthi otomatis ndeteksi format kasebut.Mangga nemtokake kanthi manual.");
/// ```
pub(super) const fn get_jw_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** Ora Didhukung **: Ngonversi saka `lisp s-expression` to` Format liyane"##),
        ("unknown-fmt", r##"Formulir file sing ora dingerteni"##),
        ("currently-supported", r##"Daftar Format sing didhukung:"##),
        ("auto-detection-failed", r##"Gagal kanthi otomatis ndeteksi format kasebut.Mangga nemtokake kanthi manual."##),
        ("not-included", r##"BINARI BINARI iki ora kalebu ** fungsionalitas konversi kanggo format sing gegandhengan.
Sampeyan kudu ngaktifake fitur sing relevan ing 'kargotom sampeyan lan mbalek maneh.
Yen piranti lunak iki ora kalebu fungsi sing cocog, kirimake masalah."##),
        ("conv-error", r##"Konversi Kesalahan"##),
        ("dst", r##"Jalur file tujuan"##),
        ("unsupported", r##"konversi format sing ora didhukung"##),
        ("invalid-json1.0", r##"File dudu format dudu sing bener `Json 1.0`, nyoba parse minangka ... json5`"##),
    ],
}
}

/// Language ID: jw;
/// Map name: "conversion_md";
pub(super) const fn get_jw_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Ora Didhukung [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Ngonversi saka [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m to[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m Format liyane[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormulir file sing ora dingerteni[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mDaftar Format sing didhukung:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mGagal kanthi otomatis ndeteksi format kasebut.Mangga nemtokake kanthi manual.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBINARI BINARI iki ora kalebu ** fungsionalitas konversi kanggo format sing gegandhengan.
[48;2;34;34;34m[38;2;255;255;255mSampeyan kudu ngaktifake fitur sing relevan ing 'kargotom sampeyan lan mbalek maneh.
[48;2;34;34;34m[38;2;255;255;255mYen piranti lunak iki ora kalebu fungsi sing cocog, kirimake masalah.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonversi Kesalahan[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mJalur file tujuan[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mkonversi format sing ora didhukung[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFile dudu format dudu sing bener [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, nyoba parse minangka ... json5[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
    ],
}
}

/// Language ID: jw;
/// Map name: "set";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Dalan file sing ora bener.");
/// ```
pub(super) const fn get_jw_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Konten sing wis diowahi ** Dadi ** disimpen amarga `--sv` ora diarani."##),
        ("new-value", r##"Nilai Anyar"##),
        ("invalid-path", r##"Dalan file sing ora bener."##),
    ],
}
}

/// Language ID: jw;
/// Map name: "set_md";
pub(super) const fn get_jw_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mKonten sing wis diowahi ** Dadi ** disimpen amarga [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ora diarani.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNilai Anyar[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mDalan file sing ora bener.[0m"##),
    ],
}
}

/// Language ID: jw;
/// Map name: "get";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format tujuan");
/// ```
pub(super) const fn get_jw_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format tujuan"##),
        ("src-fmt", r##"Format File Sumber"##),
    ],
}
}

/// jw: jw-Latn-ID
pub(super) const fn get_jw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_jw_map_conversion),
        ("get", get_jw_map_get),
        ("set_md", get_jw_map_set_md),
        ("set", get_jw_map_set),
        ("conversion_md", get_jw_map_conversion_md),
    ],
}
}

/// Language ID: ka;
/// Map name: "conversion";
/// Description: ქართული, ქართული, საქართველო;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ვერ მოხერხდა ფორმატის ავტომატურად გამოვლენა.გთხოვთ მიუთითოთ ხელით.");
/// ```
pub(super) const fn get_ka_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (5, 0),
    ],
    entries: &[
        ("conv-error", r##"კონვერტაციის შეცდომა"##),
        ("unknown-fmt", r##"უცნობი ფაილის ფორმატი"##),
        ("auto-detection-failed", r##"ვერ მოხერხდა ფორმატის ავტომატურად გამოვლენა.გთხოვთ მიუთითოთ ხელით."##),
        ("currently-supported", r##"ამჟამად მხარდაჭერილი ფორმატების სია:"##),
        ("dst", r##"დანიშნულების ფაილის გზა"##),
        ("unsupported", r##"დაუსაბუთებელი ფორმატის კონვერტაცია"##),
        ("not-included", r##"ეს ორობითი ** არ შეიცავს ** კონვერტაციის ფუნქციონირებას შესაბამისი ფორმატისთვის.
თქვენ უნდა ჩართოთ შესაბამისი ფუნქცია თქვენს `Cargo.toml` და ანაზღაურება.
თუ ეს პროგრამა არ შეიცავს შესაბამის ფუნქციონირებას, გთხოვთ, წარუდგინოთ საკითხი."##),
        ("not-saved", r##"შემდეგი შინაარსი ** არ შეინახება, რადგან `--save` არ დაურეკეს."##),
    ],
}
}

/// Language ID: ka;
/// Map name: "conversion_md";
/// Description: ქართული, ქართული, საქართველო;
pub(super) const fn get_ka_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (5, 0),
    ],
    entries: &[
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mკონვერტაციის შეცდომა[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mუცნობი ფაილის ფორმატი[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mვერ მოხერხდა ფორმატის ავტომატურად გამოვლენა.გთხოვთ მიუთითოთ ხელით.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mამჟამად მხარდაჭერილი ფორმატების სია:[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mდანიშნულების ფაილის გზა[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mდაუსაბუთებელი ფორმატის კონვერტაცია[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mეს ორობითი ** არ შეიცავს ** კონვერტაციის ფუნქციონირებას შესაბამისი ფორმატისთვის.
[48;2;34;34;34m[38;2;255;255;255mთქვენ უნდა ჩართოთ შესაბამისი ფუნქცია თქვენს [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m და ანაზღაურება.
[48;2;34;34;34m[38;2;255;255;255mთუ ეს პროგრამა არ შეიცავს შესაბამის ფუნქციონირებას, გთხოვთ, წარუდგინოთ საკითხი.[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mშემდეგი შინაარსი ** არ შეინახება, რადგან [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m არ დაურეკეს.[0m"##),
    ],
}
}

/// Language ID: ka;
/// Map name: "set";
/// Description: ქართული, ქართული, საქართველო;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "არასწორი ფაილის გზა.");
/// ```
pub(super) const fn get_ka_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"შეცვლილი შინაარსი ** არ შეინახება, რადგან `–-sv` არ დაურეკეს."##),
        ("new-value", r##"ახალი მნიშვნელობა"##),
        ("invalid-path", r##"არასწორი ფაილის გზა."##),
    ],
}
}

/// Language ID: ka;
/// Map name: "set_md";
/// Description: ქართული, ქართული, საქართველო;
pub(super) const fn get_ka_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mშეცვლილი შინაარსი ** არ შეინახება, რადგან [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m–-sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m არ დაურეკეს.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mახალი მნიშვნელობა[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mარასწორი ფაილის გზა.[0m"##),
    ],
}
}

/// Language ID: ka;
/// Map name: "get";
/// Description: ქართული, ქართული, საქართველო;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "დანიშნულების ფორმატი");
/// ```
pub(super) const fn get_ka_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"დანიშნულების ფორმატი"##),
        ("src-fmt", r##"წყაროს ფაილის ფორმატი"##),
    ],
}
}

/// ka: ქართული, ქართული, საქართველო
pub(super) const fn get_ka_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ka_map_conversion),
        ("get", get_ka_map_get),
        ("set_md", get_ka_map_set_md),
        ("set", get_ka_map_set),
        ("conversion_md", get_ka_map_conversion_md),
    ],
}
}

/// Language ID: kk;
/// Map name: "conversion";
/// Description: қазақ тілі, кирилл жазуы, Қазақстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "пішімін автоматты түрде анықтай алмады.Қолмен көрсетіңіз.");
/// ```
pub(super) const fn get_kk_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** қолдау көрсетілмейді **: `LISP S-өрнегінен` басқа форматтарға түрлендіру"##),
        ("unknown-fmt", r##"Белгісіз файл пішімін таңдаңыз"##),
        ("currently-supported", r##"Қазіргі уақытта қолдау көрсетілетін форматтар тізімі:"##),
        ("auto-detection-failed", r##"пішімін автоматты түрде анықтай алмады.Қолмен көрсетіңіз."##),
        ("not-included", r##"Бұл екілік ** тиісті формат үшін конверсиялық функцияны ** қамтымайды.
Сізге «Cargo.toml» және қондырғылардағы тиісті мүмкіндікті қосу керек.
Егер бұл бағдарламалық жасақтамада тиісті функцияларды қамтымаса, мәселені жіберіңіз."##),
        ("conv-error", r##"Айырбастау қатесі"##),
        ("dst", r##"тағайындалған файл жолы"##),
        ("unsupported", r##"Қолдау көрсетілмейтін форматты түрлендіру"##),
        ("invalid-json1.0", r##"Файл дұрыс емес, `JSON 1.0 пішімі емес,« JSON5` »деп талдап, талдауға тырыспайды."##),
    ],
}
}

/// Language ID: kk;
/// Map name: "conversion_md";
/// Description: қазақ тілі, кирилл жазуы, Қазақстан;
pub(super) const fn get_kk_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** қолдау көрсетілмейді [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLISP S-өрнегінен[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m басқа форматтарға түрлендіру[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mБелгісіз файл пішімін таңдаңыз[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mҚазіргі уақытта қолдау көрсетілетін форматтар тізімі:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mпішімін автоматты түрде анықтай алмады.Қолмен көрсетіңіз.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mБұл екілік ** тиісті формат үшін конверсиялық функцияны ** қамтымайды.
[48;2;34;34;34m[38;2;255;255;255mСізге «Cargo.toml» және қондырғылардағы тиісті мүмкіндікті қосу керек.
[48;2;34;34;34m[38;2;255;255;255mЕгер бұл бағдарламалық жасақтамада тиісті функцияларды қамтымаса, мәселені жіберіңіз.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mАйырбастау қатесі[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mтағайындалған файл жолы[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mҚолдау көрсетілмейтін форматты түрлендіру[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mФайл дұрыс емес, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0 пішімі емес,« JSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m »деп талдап, талдауға тырыспайды.[0m"##),
    ],
}
}

/// Language ID: kk;
/// Map name: "set";
/// Description: қазақ тілі, кирилл жазуы, Қазақстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "жарамсыз файл жолы.");
/// ```
pub(super) const fn get_kk_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Өзгертілген мазмұн ** сақталмайды ** сақталмайды, өйткені `--sv` шақырылмаған."##),
        ("new-value", r##"Жаңа мән"##),
        ("invalid-path", r##"жарамсыз файл жолы."##),
    ],
}
}

/// Language ID: kk;
/// Map name: "set_md";
/// Description: қазақ тілі, кирилл жазуы, Қазақстан;
pub(super) const fn get_kk_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mӨзгертілген мазмұн ** сақталмайды ** сақталмайды, өйткені [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m шақырылмаған.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mЖаңа мән[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mжарамсыз файл жолы.[0m"##),
    ],
}
}

/// Language ID: kk;
/// Map name: "get";
/// Description: қазақ тілі, кирилл жазуы, Қазақстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Тағайындау форматы");
/// ```
pub(super) const fn get_kk_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Тағайындау форматы"##),
        ("src-fmt", r##"Бастапқы файл пішімі"##),
    ],
}
}

/// kk: қазақ тілі, кирилл жазуы, Қазақстан
pub(super) const fn get_kk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_kk_map_conversion),
        ("get", get_kk_map_get),
        ("set_md", get_kk_map_set_md),
        ("set", get_kk_map_set),
        ("conversion_md", get_kk_map_conversion_md),
    ],
}
}

/// Language ID: km;
/// Map name: "conversion";
/// Description: ខ្មែរ, ខ្មែរ, កម្ពុជា;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "បានបរាជ\u{17d0}យក\u{17d2}ន\u{17bb}ងការរកឃើញទ\u{17d2}រង\u{17cb}ទ\u{17d2}រាយដោយស\u{17d2}វ\u{17d0}យប\u{17d2}រវត\u{17d2}ត\u{17b7}។ស\u{17bc}មបញ\u{17d2}ជាក\u{17cb}ដោយដៃ។");
/// ```
pub(super) const fn get_km_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
        (0, 0),
    ],
    entries: &[
        ("currently-supported", r##"បញ្ជីទ្រង់ទ្រាយដែលបានគាំទ្របច្ចុប្បន្ន:"##),
        ("unsupported", r##"ការបំលែងទ្រង់ទ្រាយដែលមិនបានគាំទ្រ"##),
        ("unknown-fmt", r##"មិនស្គាល់ទ្រង់ទ្រាយឯកសារ"##),
        ("not-support-deser-sexp", r##"មិនត្រូវបានគាំទ្រនៅឡើយទេប៉ុន្តែ **: ការបំលែងពី `lisp s-callment` ទំរង់ផ្សេងទៀត"##),
        ("conv-error", r##"ការបំលែងកំហុសកំហុស"##),
        ("dst", r##"ផ្លូវឯកសារទិសដៅ"##),
        ("auto-detection-failed", r##"បានបរាជ័យក្នុងការរកឃើញទ្រង់ទ្រាយដោយស្វ័យប្រវត្តិ។សូមបញ្ជាក់ដោយដៃ។"##),
    ],
}
}

/// Language ID: km;
/// Map name: "conversion_md";
/// Description: ខ្មែរ, ខ្មែរ, កម្ពុជា;
pub(super) const fn get_km_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
        (0, 0),
    ],
    entries: &[
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mបញ្ជីទ្រង់ទ្រាយដែលបានគាំទ្របច្ចុប្បន្ន:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mការបំលែងទ្រង់ទ្រាយដែលមិនបានគាំទ្រ[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mមិនស្គាល់ទ្រង់ទ្រាយឯកសារ[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255mមិនត្រូវបានគាំទ្រនៅឡើយទេប៉ុន្តែ [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: ការបំលែងពី [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-callment[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ទំរង់ផ្សេងទៀត[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mការបំលែងកំហុសកំហុស[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mផ្លូវឯកសារទិសដៅ[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mបានបរាជ័យក្នុងការរកឃើញទ្រង់ទ្រាយដោយស្វ័យប្រវត្តិ។សូមបញ្ជាក់ដោយដៃ។[0m"##),
    ],
}
}

/// Language ID: km;
/// Map name: "set";
/// Description: ខ្មែរ, ខ្មែរ, កម្ពុជា;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ផ\u{17d2}ល\u{17bc}វឯកសារម\u{17b7}នត\u{17d2}រ\u{17b9}មត\u{17d2}រ\u{17bc}វ។");
/// ```
pub(super) const fn get_km_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"មាតិកាដែលបានកែប្រែ ** នឹងមិន ** ត្រូវបានរក្សាទុកព្រោះ `- មិនត្រូវបានគេហៅទេ។"##),
        ("new-value", r##"តម្លៃថ្មី"##),
        ("invalid-path", r##"ផ្លូវឯកសារមិនត្រឹមត្រូវ។"##),
    ],
}
}

/// Language ID: km;
/// Map name: "set_md";
/// Description: ខ្មែរ, ខ្មែរ, កម្ពុជា;
pub(super) const fn get_km_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mមាតិកាដែលបានកែប្រែ ** នឹងមិន ** ត្រូវបានរក្សាទុកព្រោះ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m- មិនត្រូវបានគេហៅទេ។[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mតម្លៃថ្មី[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mផ្លូវឯកសារមិនត្រឹមត្រូវ។[0m"##),
    ],
}
}

/// Language ID: km;
/// Map name: "get";
/// Description: ខ្មែរ, ខ្មែរ, កម្ពុជា;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ទ\u{17d2}រង\u{17cb}ទ\u{17d2}រាយទ\u{17b7}សដៅ");
/// ```
pub(super) const fn get_km_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ទ្រង់ទ្រាយទិសដៅ"##),
        ("src-fmt", r##"ទ្រង់ទ្រាយឯកសារប្រភព"##),
    ],
}
}

/// km: ខ្មែរ, ខ្មែរ, កម្ពុជា
pub(super) const fn get_km_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_km_map_conversion),
        ("get", get_km_map_get),
        ("set_md", get_km_map_set_md),
        ("set", get_km_map_set),
        ("conversion_md", get_km_map_conversion_md),
    ],
}
}

/// Language ID: kn;
/// Map name: "conversion";
/// Description: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ಸ\u{ccd}ವರ\u{cc2}ಪವನ\u{ccd}ನು ಸ\u{ccd}ವಯಂಚಾಲ\u{cbf}ತವಾಗ\u{cbf} ಕಂಡುಹ\u{cbf}ಡ\u{cbf}ಯಲು ವ\u{cbf}ಫಲವಾಗ\u{cbf}ದ\u{cc6}.ದಯವ\u{cbf}ಟ\u{ccd}ಟು ಹಸ\u{ccd}ತಚಾಲ\u{cbf}ತವಾಗ\u{cbf} ನ\u{cbf}ರ\u{ccd}ದ\u{cbf}ಷ\u{ccd}ಟಪಡ\u{cbf}ಸ\u{cbf}.");
/// ```
pub(super) const fn get_kn_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"ಫೈಲ್ ಮಾನ್ಯ `JSON 1.0` ಸ್ವರೂಪವಲ್ಲ, `json5` ಎಂದು ಪಾರ್ಸ್ ಮಾಡಲು ಪ್ರಯತ್ನಿಸುತ್ತಿದೆ ..."##),
        ("not-included", r##"ಈ ಬೈನರಿ ** ಸಂಬಂಧಿತ ಸ್ವರೂಪಕ್ಕಾಗಿ ** ಪರಿವರ್ತನೆ ಕಾರ್ಯವನ್ನು ಒಳಗೊಂಡಿಲ್ಲ.
ನಿಮ್ಮ `ಕಾರ್ಗೋ.ಟೊಮ್ಲ್` ಮತ್ತು ಮರು ಕಂಪೈಲ್ನಲ್ಲಿ ನೀವು ಸಂಬಂಧಿತ ವೈಶಿಷ್ಟ್ಯವನ್ನು ಸಕ್ರಿಯಗೊಳಿಸಬೇಕಾಗಿದೆ.
ಈ ಸಾಫ್ಟ್‌ವೇರ್ ಅನುಗುಣವಾದ ಕಾರ್ಯವನ್ನು ಒಳಗೊಂಡಿರದಿದ್ದರೆ, ದಯವಿಟ್ಟು ಸಮಸ್ಯೆಯನ್ನು ಸಲ್ಲಿಸಿ."##),
        ("currently-supported", r##"ಪ್ರಸ್ತುತ ಬೆಂಬಲಿತ ಸ್ವರೂಪಗಳ ಪಟ್ಟಿ:"##),
        ("unsupported", r##"ಬೆಂಬಲಿಸದ ಸ್ವರೂಪ ಪರಿವರ್ತನೆ ಪರಿವರ್ತನೆ"##),
        ("auto-detection-failed", r##"ಸ್ವರೂಪವನ್ನು ಸ್ವಯಂಚಾಲಿತವಾಗಿ ಕಂಡುಹಿಡಿಯಲು ವಿಫಲವಾಗಿದೆ.ದಯವಿಟ್ಟು ಹಸ್ತಚಾಲಿತವಾಗಿ ನಿರ್ದಿಷ್ಟಪಡಿಸಿ."##),
        ("not-support-deser-sexp", r##"** ಇನ್ನೂ ಬೆಂಬಲಿತವಾಗಿಲ್ಲ **: `ಲಿಸ್ಪ್ ಎಸ್-ಎಕ್ಸ್‌ಪ್ರೆಶನ್'ನಿಂದ` ಇತರ ಸ್ವರೂಪಗಳು'ಗೆ ಪರಿವರ್ತಿಸುವುದು"##),
        ("unknown-fmt", r##"ಅಜ್ಞಾತ ಫೈಲ್ ಫಾರ್ಮ್ಯಾಟ್"##),
        ("not-saved", r##"ಈ ಕೆಳಗಿನ ವಿಷಯವನ್ನು ** ಉಳಿಸಲಾಗುವುದಿಲ್ಲ ಏಕೆಂದರೆ `--save` ಎಂದು ಕರೆಯಲಾಗುವುದಿಲ್ಲ."##),
        ("dst", r##"ಗಮ್ಯಸ್ಥಾನ ಫೈಲ್ ಮಾರ್ಗ"##),
        ("conv-error", r##"ಪರಿವರ್ತನೆ ದೋಷ"##),
    ],
}
}

/// Language ID: kn;
/// Map name: "conversion_md";
/// Description: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ;
pub(super) const fn get_kn_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mಫೈಲ್ ಮಾನ್ಯ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ಸ್ವರೂಪವಲ್ಲ, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ಎಂದು ಪಾರ್ಸ್ ಮಾಡಲು ಪ್ರಯತ್ನಿಸುತ್ತಿದೆ ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mಈ ಬೈನರಿ ** ಸಂಬಂಧಿತ ಸ್ವರೂಪಕ್ಕಾಗಿ ** ಪರಿವರ್ತನೆ ಕಾರ್ಯವನ್ನು ಒಳಗೊಂಡಿಲ್ಲ.
[48;2;34;34;34m[38;2;255;255;255mನಿಮ್ಮ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mಕಾರ್ಗೋ.ಟೊಮ್ಲ್[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ಮತ್ತು ಮರು ಕಂಪೈಲ್ನಲ್ಲಿ ನೀವು ಸಂಬಂಧಿತ ವೈಶಿಷ್ಟ್ಯವನ್ನು ಸಕ್ರಿಯಗೊಳಿಸಬೇಕಾಗಿದೆ.
[48;2;34;34;34m[38;2;255;255;255mಈ ಸಾಫ್ಟ್‌ವೇರ್ ಅನುಗುಣವಾದ ಕಾರ್ಯವನ್ನು ಒಳಗೊಂಡಿರದಿದ್ದರೆ, ದಯವಿಟ್ಟು ಸಮಸ್ಯೆಯನ್ನು ಸಲ್ಲಿಸಿ.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mಪ್ರಸ್ತುತ ಬೆಂಬಲಿತ ಸ್ವರೂಪಗಳ ಪಟ್ಟಿ:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mಬೆಂಬಲಿಸದ ಸ್ವರೂಪ ಪರಿವರ್ತನೆ ಪರಿವರ್ತನೆ[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mಸ್ವರೂಪವನ್ನು ಸ್ವಯಂಚಾಲಿತವಾಗಿ ಕಂಡುಹಿಡಿಯಲು ವಿಫಲವಾಗಿದೆ.ದಯವಿಟ್ಟು ಹಸ್ತಚಾಲಿತವಾಗಿ ನಿರ್ದಿಷ್ಟಪಡಿಸಿ.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ಇನ್ನೂ ಬೆಂಬಲಿತವಾಗಿಲ್ಲ [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mಲಿಸ್ಪ್ ಎಸ್-ಎಕ್ಸ್‌ಪ್ರೆಶನ್'ನಿಂದ[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ಇತರ ಸ್ವರೂಪಗಳು'ಗೆ ಪರಿವರ್ತಿಸುವುದು[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mಅಜ್ಞಾತ ಫೈಲ್ ಫಾರ್ಮ್ಯಾಟ್[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mಈ ಕೆಳಗಿನ ವಿಷಯವನ್ನು ** ಉಳಿಸಲಾಗುವುದಿಲ್ಲ ಏಕೆಂದರೆ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ಎಂದು ಕರೆಯಲಾಗುವುದಿಲ್ಲ.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mಗಮ್ಯಸ್ಥಾನ ಫೈಲ್ ಮಾರ್ಗ[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mಪರಿವರ್ತನೆ ದೋಷ[0m"##),
    ],
}
}

/// Language ID: kn;
/// Map name: "set";
/// Description: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ಅಮಾನ\u{ccd}ಯ ಫೈಲ\u{ccd} ಮಾರ\u{ccd}ಗ.");
/// ```
pub(super) const fn get_kn_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ಮಾರ್ಪಡಿಸಿದ ವಿಷಯವನ್ನು ** ಉಳಿಸಲಾಗುವುದಿಲ್ಲ ಏಕೆಂದರೆ `--sv` ಎಂದು ಕರೆಯಲಾಗುವುದಿಲ್ಲ."##),
        ("new-value", r##"ಹೊಸ ಮೌಲ್ಯ"##),
        ("invalid-path", r##"ಅಮಾನ್ಯ ಫೈಲ್ ಮಾರ್ಗ."##),
    ],
}
}

/// Language ID: kn;
/// Map name: "set_md";
/// Description: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ;
pub(super) const fn get_kn_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mಮಾರ್ಪಡಿಸಿದ ವಿಷಯವನ್ನು ** ಉಳಿಸಲಾಗುವುದಿಲ್ಲ ಏಕೆಂದರೆ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ಎಂದು ಕರೆಯಲಾಗುವುದಿಲ್ಲ.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mಹೊಸ ಮೌಲ್ಯ[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mಅಮಾನ್ಯ ಫೈಲ್ ಮಾರ್ಗ.[0m"##),
    ],
}
}

/// Language ID: kn;
/// Map name: "get";
/// Description: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ಗಮ\u{ccd}ಯಸ\u{ccd}ಥಾನ ಸ\u{ccd}ವರ\u{cc2}ಪ");
/// ```
pub(super) const fn get_kn_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ಗಮ್ಯಸ್ಥಾನ ಸ್ವರೂಪ"##),
        ("src-fmt", r##"ಮೂಲ ಫೈಲ್ ಸ್ವರೂಪ"##),
    ],
}
}

/// kn: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ
pub(super) const fn get_kn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_kn_map_conversion),
        ("get", get_kn_map_get),
        ("set_md", get_kn_map_set_md),
        ("set", get_kn_map_set),
        ("conversion_md", get_kn_map_conversion_md),
    ],
}
}

/// Language ID: ko;
/// Map name: "conversion";
/// Description: 한국어, 한국 문자, 대한민국;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "자동 형식 감지 실패, 수동으로 지정해야 합니다.");
/// ```
pub(super) const fn get_ko_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"`json 1.0` 형식이 올바르지 않습니다. `json5`로 분석 중입니다..."##),
        ("not-included", r##"이 바이너리 파일에는 관련된 형식의 변환 기능이 **포함되어 있지 않습니다**.
`Cargo.toml`에서 해당 기능을 활성화하고 다시 컴파일해야 합니다.
이 소프트웨어에 해당 기능이 없으면 이슈를 제출하세요."##),
        ("currently-supported", r##"현재 지원되는 형식 목록"##),
        ("unsupported", r##"지원되지 않는 형식 변환"##),
        ("auto-detection-failed", r##"자동 형식 감지 실패, 수동으로 지정해야 합니다."##),
        ("not-support-deser-sexp", r##"`Lisp S-Expression`을(를) `다른 형식`으로 변환하는 것은 **아직 지원되지 않습니다**."##),
        ("unknown-fmt", r##"알 수 없는 파일 형식"##),
        ("not-saved", r##"`--save`를 호출하지 않았기 때문에 다음 내용은 **저장되지 않습니다**."##),
        ("dst", r##"대상 파일 경로"##),
        ("conv-error", r##"변환 오류"##),
    ],
}
}

/// Language ID: ko;
/// Map name: "conversion_md";
/// Description: 한국어, 한국 문자, 대한민국;
pub(super) const fn get_ko_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 형식이 올바르지 않습니다. [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m로 분석 중입니다...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255m이 바이너리 파일에는 관련된 형식의 변환 기능이 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m포함되어 있지 않습니다[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m.
[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m에서 해당 기능을 활성화하고 다시 컴파일해야 합니다.
[48;2;34;34;34m[38;2;255;255;255m이 소프트웨어에 해당 기능이 없으면 이슈를 제출하세요.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255m현재 지원되는 형식 목록[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255m지원되지 않는 형식 변환[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255m자동 형식 감지 실패, 수동으로 지정해야 합니다.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m을(를) [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m다른 형식[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m으로 변환하는 것은 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m아직 지원되지 않습니다[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255m알 수 없는 파일 형식[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m를 호출하지 않았기 때문에 다음 내용은 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m저장되지 않습니다[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255m대상 파일 경로[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255m변환 오류[0m"##),
    ],
}
}

/// Language ID: ko;
/// Map name: "set";
/// Description: 한국어, 한국 문자, 대한민국;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "유효하지 않은 파일 경로");
/// ```
pub(super) const fn get_ko_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"`--save`를 호출하지 않았기 때문에 수정된 내용은 **저장되지 않습니다**."##),
        ("new-value", r##"새 값(value)"##),
        ("invalid-path", r##"유효하지 않은 파일 경로"##),
    ],
}
}

/// Language ID: ko;
/// Map name: "set_md";
/// Description: 한국어, 한국 문자, 대한민국;
pub(super) const fn get_ko_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m를 호출하지 않았기 때문에 수정된 내용은 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m저장되지 않습니다[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255m새 값(value)[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255m유효하지 않은 파일 경로[0m"##),
    ],
}
}

/// Language ID: ko;
/// Map name: "get";
/// Description: 한국어, 한국 문자, 대한민국;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "대상 형식");
/// ```
pub(super) const fn get_ko_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"대상 형식"##),
        ("src-fmt", r##"소스 파일 형식"##),
    ],
}
}

/// ko: 한국어, 한국 문자, 대한민국
pub(super) const fn get_ko_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ko_map_conversion),
        ("get", get_ko_map_get),
        ("set_md", get_ko_map_set_md),
        ("set", get_ko_map_set),
        ("conversion_md", get_ko_map_conversion_md),
    ],
}
}

/// Language ID: ku;
/// Map name: "conversion";
/// Description: kurdî, latînî, Tirkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Nehişt ku bixweber format bixwe bibîne.Ji kerema xwe bi destan diyar bikin.");
/// ```
pub(super) const fn get_ku_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Pel ne formatek derbasdar e `Json 1.0`, hewl dide ku wekî `JSON5` ..."##),
        ("not-included", r##"Ev binary ** ** fonksiyona veguhastinê ji bo forma têkildar nagire.
Hûn hewce ne ku taybetmendiya têkildar di `Cargo.toml` û Recompile de çalak bikin.
Ger ev nermalava fonksiyonê têkildar neke, ji kerema xwe pirsgirêkek bişînin."##),
        ("currently-supported", r##"Navnîşa Formatên Piştgirî:"##),
        ("unsupported", r##"Veguheztina Format a Piştgirî"##),
        ("auto-detection-failed", r##"Nehişt ku bixweber format bixwe bibîne.Ji kerema xwe bi destan diyar bikin."##),
        ("not-support-deser-sexp", r##"** Piştgirî nehatiye **: Ji `Lisp S-îfadeya` LISP-ê` ji `Formatên din ve hatî veguheztin"##),
        ("unknown-fmt", r##"Forma pelê nediyar"##),
        ("not-saved", r##"Naveroka jêrîn ** Naha ** dê xilas nebe ji ber ku `--save` nayê gotin."##),
        ("dst", r##"Pêla Destination Destination"##),
        ("conv-error", r##"Errorewtiya veguherînê"##),
    ],
}
}

/// Language ID: ku;
/// Map name: "conversion_md";
/// Description: kurdî, latînî, Tirkiye;
pub(super) const fn get_ku_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mPel ne formatek derbasdar e [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, hewl dide ku wekî [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mEv binary ** ** fonksiyona veguhastinê ji bo forma têkildar nagire.
[48;2;34;34;34m[38;2;255;255;255mHûn hewce ne ku taybetmendiya têkildar di [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m û Recompile de çalak bikin.
[48;2;34;34;34m[38;2;255;255;255mGer ev nermalava fonksiyonê têkildar neke, ji kerema xwe pirsgirêkek bişînin.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mNavnîşa Formatên Piştgirî:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mVeguheztina Format a Piştgirî[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNehişt ku bixweber format bixwe bibîne.Ji kerema xwe bi destan diyar bikin.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Piştgirî nehatiye [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Ji [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-îfadeya[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m LISP-ê[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ji [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114mFormatên din ve hatî veguheztin[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mForma pelê nediyar[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNaveroka jêrîn ** Naha ** dê xilas nebe ji ber ku [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nayê gotin.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mPêla Destination Destination[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mErrorewtiya veguherînê[0m"##),
    ],
}
}

/// Language ID: ku;
/// Map name: "set";
/// Description: kurdî, latînî, Tirkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "riya pelê nederbasdar.");
/// ```
pub(super) const fn get_ku_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Pelê   Naveroka Guhertin ** ****************** na **** na xelas nebe ji ber ku `--sv` nayê gazî kirin."##),
        ("new-value", r##"Nirxa Nû"##),
        ("invalid-path", r##"riya pelê nederbasdar."##),
    ],
}
}

/// Language ID: ku;
/// Map name: "set_md";
/// Description: kurdî, latînî, Tirkiye;
pub(super) const fn get_ku_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mPelê   Naveroka Guhertin ** ****************** na **** na xelas nebe ji ber ku [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nayê gazî kirin.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNirxa Nû[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mriya pelê nederbasdar.[0m"##),
    ],
}
}

/// Language ID: ku;
/// Map name: "get";
/// Description: kurdî, latînî, Tirkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Formata Destana");
/// ```
pub(super) const fn get_ku_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Formata Destana"##),
        ("src-fmt", r##"Formata pelê çavkaniyê"##),
    ],
}
}

/// ku: kurdî, latînî, Tirkiye
pub(super) const fn get_ku_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ku_map_conversion),
        ("get", get_ku_map_get),
        ("set_md", get_ku_map_set_md),
        ("set", get_ku_map_set),
        ("conversion_md", get_ku_map_conversion_md),
    ],
}
}

/// Language ID: ky;
/// Map name: "conversion";
/// Description: кыргызча, Кирилл, Кыргызстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Форматты автоматтык түрдө аныктай алган жок.Сураныч, кол менен белгилеңиз.");
/// ```
pub(super) const fn get_ky_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Файл `JSON 1.0` 'форматына ылайык эмес"##),
        ("not-included", r##"Бул экилик ** камтылбайт ** Качан, тиешелүү форматта конверсиялык функция.
Сиздин `Cargo.toml` жана Recial'до тиешелүү функцияны иштетишиңиз керек.
Эгерде бул программа тиешелүү функционалдык камтылбаса, анда маселени тапшырыңыз."##),
        ("currently-supported", r##"Учурда колдоого алынган форматтар тизмеси:"##),
        ("unsupported", r##"Форматты колдоого алынбаган өзгөртүү"##),
        ("auto-detection-failed", r##"Форматты автоматтык түрдө аныктай алган жок.Сураныч, кол менен белгилеңиз."##),
        ("not-support-deser-sexp", r##"** Бирок колдоого алынбайт **: `Lisp S-сөз айкашы`"##),
        ("unknown-fmt", r##"Белгисиз файл Форматы"##),
        ("not-saved", r##"Төмөнкү мазмун ** ** сакталбайт ** сакталбайт, анткени `--save` деп аталган жок."##),
        ("dst", r##"көздөгөн файл жол"##),
        ("conv-error", r##"Конверсиялык ката"##),
    ],
}
}

/// Language ID: ky;
/// Map name: "conversion_md";
/// Description: кыргызча, Кирилл, Кыргызстан;
pub(super) const fn get_ky_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mФайл [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 'форматына ылайык эмес[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mБул экилик ** камтылбайт ** Качан, тиешелүү форматта конверсиялык функция.
[48;2;34;34;34m[38;2;255;255;255mСиздин [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m жана Recial'до тиешелүү функцияны иштетишиңиз керек.
[48;2;34;34;34m[38;2;255;255;255mЭгерде бул программа тиешелүү функционалдык камтылбаса, анда маселени тапшырыңыз.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mУчурда колдоого алынган форматтар тизмеси:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mФорматты колдоого алынбаган өзгөртүү[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mФорматты автоматтык түрдө аныктай алган жок.Сураныч, кол менен белгилеңиз.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Бирок колдоого алынбайт [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-сөз айкашы[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mБелгисиз файл Форматы[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mТөмөнкү мазмун ** ** сакталбайт ** сакталбайт, анткени [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m деп аталган жок.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mкөздөгөн файл жол[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mКонверсиялык ката[0m"##),
    ],
}
}

/// Language ID: ky;
/// Map name: "set";
/// Description: кыргызча, Кирилл, Кыргызстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Жараксыз файл жолу.");
/// ```
pub(super) const fn get_ky_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Өзгөртүлгөн мазмун ** жок *** сакталбайт, анткени `--sv` деп аталган жок."##),
        ("new-value", r##"Жаңы маани"##),
        ("invalid-path", r##"Жараксыз файл жолу."##),
    ],
}
}

/// Language ID: ky;
/// Map name: "set_md";
/// Description: кыргызча, Кирилл, Кыргызстан;
pub(super) const fn get_ky_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mӨзгөртүлгөн мазмун ** жок *** сакталбайт, анткени [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m деп аталган жок.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mЖаңы маани[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mЖараксыз файл жолу.[0m"##),
    ],
}
}

/// Language ID: ky;
/// Map name: "get";
/// Description: кыргызча, Кирилл, Кыргызстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Көздөлгөн формат");
/// ```
pub(super) const fn get_ky_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Көздөлгөн формат"##),
        ("src-fmt", r##"Булак файл форматы"##),
    ],
}
}

/// ky: кыргызча, Кирилл, Кыргызстан
pub(super) const fn get_ky_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ky_map_conversion),
        ("get", get_ky_map_get),
        ("set_md", get_ky_map_set_md),
        ("set", get_ky_map_set),
        ("conversion_md", get_ky_map_conversion_md),
    ],
}
}

/// Language ID: la;
/// Map name: "conversion";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Deficio ad automatice deprehendere forma.Placere specificare manually.");
/// ```
pub(super) const fn get_la_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Quod file non valet `JSON 1.0` Forma, conatur parse ut `json5` ..."##),
        ("not-included", r##"Hoc binarii ** non includit ** conversionem functionality ad pertinet format.
Vos postulo ut enable pertinet pluma in vestri `Cargo.toml` et recompile.
Si hoc software non includit correspondentes functionality, velit submittere an proventus."##),
        ("currently-supported", r##"Currently Formats List:"##),
        ("unsupported", r##"Unsupported Forma Conversion"##),
        ("auto-detection-failed", r##"Deficio ad automatice deprehendere forma.Placere specificare manually."##),
        ("not-support-deser-sexp", r##"** ** non valet tamen **: Converting a `Lisp S-expression` ut` Alii formats`"##),
        ("unknown-fmt", r##"Forma"##),
        ("not-saved", r##"Sequenti contentus ** non ** salvus erit quia `--save` non dicitur."##),
        ("dst", r##"Destination file semita"##),
        ("conv-error", r##"Conversion Error"##),
    ],
}
}

/// Language ID: la;
/// Map name: "conversion_md";
pub(super) const fn get_la_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mQuod file non valet [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m Forma, conatur parse ut [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mHoc binarii ** non includit ** conversionem functionality ad pertinet format.
[48;2;34;34;34m[38;2;255;255;255mVos postulo ut enable pertinet pluma in vestri [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m et recompile.
[48;2;34;34;34m[38;2;255;255;255mSi hoc software non includit correspondentes functionality, velit submittere an proventus.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mCurrently Formats List:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mUnsupported Forma Conversion[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mDeficio ad automatice deprehendere forma.Placere specificare manually.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ** non valet tamen [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Converting a [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ut[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m Alii formats[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mForma[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mSequenti contentus ** non ** salvus erit quia [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m non dicitur.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestination file semita[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mConversion Error[0m"##),
    ],
}
}

/// Language ID: la;
/// Map name: "set";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Aliquam File semita.");
/// ```
pub(super) const fn get_la_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Mutatio contentus ** non ** salvus erit quia `--sv` non vocavit."##),
        ("new-value", r##"Novum Value"##),
        ("invalid-path", r##"Aliquam File semita."##),
    ],
}
}

/// Language ID: la;
/// Map name: "set_md";
pub(super) const fn get_la_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mMutatio contentus ** non ** salvus erit quia [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m non vocavit.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNovum Value[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mAliquam File semita.[0m"##),
    ],
}
}

/// Language ID: la;
/// Map name: "get";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Destination Format");
/// ```
pub(super) const fn get_la_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Destination Format"##),
        ("src-fmt", r##"Fontem file format"##),
    ],
}
}

/// la: la-Latn-VA
pub(super) const fn get_la_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_la_map_conversion),
        ("get", get_la_map_get),
        ("set_md", get_la_map_set_md),
        ("set", get_la_map_set),
        ("conversion_md", get_la_map_conversion_md),
    ],
}
}

/// Language ID: lb;
/// Map name: "conversion";
/// Description: Lëtzebuergesch, Laténgesch, Lëtzebuerg;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ass net automatesch d'Format automatesch z'entdecken.Gitt w.e.g. manuell un.");
/// ```
pub(super) const fn get_lb_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** Net huet nach net ënnerstëtzt **: Konvertéiert vun `Lisp S-Timer` op` Aner Formatoren`"##),
        ("unknown-fmt", r##"Onbekannt Dateiformat"##),
        ("currently-supported", r##"AKTAKT KONFACHT FORMITATER Lëscht:"##),
        ("auto-detection-failed", r##"ass net automatesch d'Format automatesch z'entdecken.Gitt w.e.g. manuell un."##),
        ("not-included", r##"Dëst binär *** D'Konversiouns Funktionalitéit fir déi entspriechend Format.
Dir musst déi relevant Feature an Ärem `cargo.atoml` erlaben an erholl.
Wann dës Software net déi entspriechend Funktionalitéit enthält, da gitt w.e.g."##),
        ("conv-error", r##"Conversiounsfeeler"##),
        ("dst", r##"Destinatioun Dateiwee"##),
        ("unsupported", r##"net ënnerstëtzten Format Konversioun"##),
        ("invalid-json1.0", r##"ass d'Datei net e gülteg `JSON 1.0`. Format, probéiert als `JSON5` ..."##),
    ],
}
}

/// Language ID: lb;
/// Map name: "conversion_md";
/// Description: Lëtzebuergesch, Laténgesch, Lëtzebuerg;
pub(super) const fn get_lb_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Net huet nach net ënnerstëtzt [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Konvertéiert vun [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Timer[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m op[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m Aner Formatoren[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mOnbekannt Dateiformat[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mAKTAKT KONFACHT FORMITATER Lëscht:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mass net automatesch d'Format automatesch z'entdecken.Gitt w.e.g. manuell un.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDëst binär *** D'Konversiouns Funktionalitéit fir déi entspriechend Format.
[48;2;34;34;34m[38;2;255;255;255mDir musst déi relevant Feature an Ärem [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mcargo.atoml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m erlaben an erholl.
[48;2;34;34;34m[38;2;255;255;255mWann dës Software net déi entspriechend Funktionalitéit enthält, da gitt w.e.g.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mConversiounsfeeler[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestinatioun Dateiwee[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mnet ënnerstëtzten Format Konversioun[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mass d'Datei net e gülteg [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m. Format, probéiert als [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
    ],
}
}

/// Language ID: lb;
/// Map name: "set";
/// Description: Lëtzebuergesch, Laténgesch, Lëtzebuerg;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ongëlteg Dateiwee.");
/// ```
pub(super) const fn get_lb_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##">"##),
        ("new-value", r##"Neie Wäert"##),
        ("invalid-path", r##"Ongëlteg Dateiwee."##),
    ],
}
}

/// Language ID: lb;
/// Map name: "set_md";
/// Description: Lëtzebuergesch, Laténgesch, Lëtzebuerg;
pub(super) const fn get_lb_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;102;217;239m>[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNeie Wäert[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mOngëlteg Dateiwee.[0m"##),
    ],
}
}

/// Language ID: lb;
/// Map name: "get";
/// Description: Lëtzebuergesch, Laténgesch, Lëtzebuerg;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Destinatioun Format");
/// ```
pub(super) const fn get_lb_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Destinatioun Format"##),
        ("src-fmt", r##"Quell Datei Format"##),
    ],
}
}

/// lb: Lëtzebuergesch, Laténgesch, Lëtzebuerg
pub(super) const fn get_lb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_lb_map_conversion),
        ("get", get_lb_map_get),
        ("set_md", get_lb_map_set_md),
        ("set", get_lb_map_set),
        ("conversion_md", get_lb_map_conversion_md),
    ],
}
}

/// Language ID: lo;
/// Map name: "conversion";
/// Description: ລາວ, ລາວ, ລາວ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ລ\u{ebb}\u{ec9}ມເຫລວໃນການກວດພ\u{ebb}ບຮ\u{eb9}ບແບບໂດຍອ\u{eb1}ດຕະໂນມ\u{eb1}ດ.ກະລ\u{eb8}ນາລະບ\u{eb8}ດ\u{ec9}ວຍຕ\u{ebb}ນເອງ.");
/// ```
pub(super) const fn get_lo_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"ຂໍ້ຜິດພາດໃນການແປງ"##),
        ("not-support-deser-sexp", r##"** ບໍ່ໄດ້ຮັບການສະຫນັບສະຫນູນເທື່ອ"##),
        ("dst", r##"ເສັ້ນທາງໄຟລ໌ປາຍທາງ"##),
        ("unsupported", r##"ການແປງຮູບແບບທີ່ບໍ່ໄດ້ຮັບການສະຫນັບສະຫນູນ"##),
        ("auto-detection-failed", r##"ລົ້ມເຫລວໃນການກວດພົບຮູບແບບໂດຍອັດຕະໂນມັດ.ກະລຸນາລະບຸດ້ວຍຕົນເອງ."##),
        ("currently-supported", r##"ປະຈຸບັນສະຫນັບສະຫນູນບັນຊີລາຍຊື່ຮູບແບບ:"##),
        ("not-included", r##"Binary ນີ້ ** ບໍ່ລວມເອົາ ** ການເຮັດວຽກຂອງການປ່ຽນໃຈເຫລື້ອມໃສສໍາລັບຮູບແບບທີ່ກ່ຽວຂ້ອງ.
ທ່ານຈໍາເປັນຕ້ອງເປີດໃຊ້ຄຸນສົມບັດທີ່ກ່ຽວຂ້ອງໃນ `Cargo.Toml` ແລະ recompile.
ຖ້າຊອບແວນີ້ບໍ່ໄດ້ລວມເອົາການເຮັດວຽກທີ່ສອດຄ້ອງກັນ, ກະລຸນາສົ່ງບັນຫາ."##),
        ("unknown-fmt", r##"ຮູບແບບເອກະສານທີ່ບໍ່ຮູ້ຈັກ"##),
    ],
}
}

/// Language ID: lo;
/// Map name: "conversion_md";
/// Description: ລາວ, ລາວ, ລາວ;
pub(super) const fn get_lo_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mຂໍ້ຜິດພາດໃນການແປງ[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ບໍ່ໄດ້ຮັບການສະຫນັບສະຫນູນເທື່ອ[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mເສັ້ນທາງໄຟລ໌ປາຍທາງ[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mການແປງຮູບແບບທີ່ບໍ່ໄດ້ຮັບການສະຫນັບສະຫນູນ[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mລົ້ມເຫລວໃນການກວດພົບຮູບແບບໂດຍອັດຕະໂນມັດ.ກະລຸນາລະບຸດ້ວຍຕົນເອງ.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mປະຈຸບັນສະຫນັບສະຫນູນບັນຊີລາຍຊື່ຮູບແບບ:[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinary ນີ້ ** ບໍ່ລວມເອົາ ** ການເຮັດວຽກຂອງການປ່ຽນໃຈເຫລື້ອມໃສສໍາລັບຮູບແບບທີ່ກ່ຽວຂ້ອງ.
[48;2;34;34;34m[38;2;255;255;255mທ່ານຈໍາເປັນຕ້ອງເປີດໃຊ້ຄຸນສົມບັດທີ່ກ່ຽວຂ້ອງໃນ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ແລະ recompile.
[48;2;34;34;34m[38;2;255;255;255mຖ້າຊອບແວນີ້ບໍ່ໄດ້ລວມເອົາການເຮັດວຽກທີ່ສອດຄ້ອງກັນ, ກະລຸນາສົ່ງບັນຫາ.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mຮູບແບບເອກະສານທີ່ບໍ່ຮູ້ຈັກ[0m"##),
    ],
}
}

/// Language ID: lo;
/// Map name: "set";
/// Description: ລາວ, ລາວ, ລາວ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ເສ\u{eb1}\u{ec9}ນທາງເອກະສານບ\u{ecd}\u{ec8}ຖ\u{eb7}ກຕ\u{ec9}ອງ.");
/// ```
pub(super) const fn get_lo_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ເນື້ອຫາທີ່ຖືກດັດແກ້ ** ຈະບໍ່ ** ຈະໄດ້ຮັບຄວາມລອດເພາະວ່າ `- ຂ້ອຍບໍ່ໄດ້ຖືກເອີ້ນ."##),
        ("new-value", r##"ມູນຄ່າໃຫມ່"##),
        ("invalid-path", r##"ເສັ້ນທາງເອກະສານບໍ່ຖືກຕ້ອງ."##),
    ],
}
}

/// Language ID: lo;
/// Map name: "set_md";
/// Description: ລາວ, ລາວ, ລາວ;
pub(super) const fn get_lo_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mເນື້ອຫາທີ່ຖືກດັດແກ້ ** ຈະບໍ່ ** ຈະໄດ້ຮັບຄວາມລອດເພາະວ່າ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m- ຂ້ອຍບໍ່ໄດ້ຖືກເອີ້ນ.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mມູນຄ່າໃຫມ່[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mເສັ້ນທາງເອກະສານບໍ່ຖືກຕ້ອງ.[0m"##),
    ],
}
}

/// Language ID: lo;
/// Map name: "get";
/// Description: ລາວ, ລາວ, ລາວ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ຮ\u{eb9}ບແບບປາຍທາງ");
/// ```
pub(super) const fn get_lo_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ຮູບແບບປາຍທາງ"##),
        ("src-fmt", r##"ຮູບແບບເອກະສານແຫຼ່ງຂໍ້ມູນ"##),
    ],
}
}

/// lo: ລາວ, ລາວ, ລາວ
pub(super) const fn get_lo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_lo_map_conversion),
        ("get", get_lo_map_get),
        ("set_md", get_lo_map_set_md),
        ("set", get_lo_map_set),
        ("conversion_md", get_lo_map_conversion_md),
    ],
}
}

/// Language ID: lt;
/// Map name: "conversion";
/// Description: lietuvių, lotynų, Lietuva;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Nepavyko automatiškai aptikti formato.Nurodykite rankiniu būdu.");
/// ```
pub(super) const fn get_lt_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** dar nepalaikomas **: Konvertuoti iš „lisp s-ekspression“ į „kitus formatus“"##),
        ("unknown-fmt", r##"Nežinomas failo formatas"##),
        ("currently-supported", r##"Šiuo metu palaikomų formatų sąrašas:"##),
        ("auto-detection-failed", r##"Nepavyko automatiškai aptikti formato.Nurodykite rankiniu būdu."##),
        ("not-included", r##"Šis dvejetainis ** neapima ** atitinkamo formato konvertavimo funkcijos.
Turite įgalinti atitinkamą funkciją „Cargo.toml“ ir perkompiliuoti.
Jei šioje programinėje įrangoje nėra atitinkamos funkcijos, pateikite problemą."##),
        ("conv-error", r##"Konversijos klaida"##),
        ("dst", r##"paskirties failo kelias"##),
        ("unsupported", r##"Nepalaikomas formato konvertavimas"##),
        ("invalid-json1.0", r##"Failas nėra galiojantis „JSON 1.0“ formatas, bandantis išanalizuoti kaip „JSON5“ ..."##),
    ],
}
}

/// Language ID: lt;
/// Map name: "conversion_md";
/// Description: lietuvių, lotynų, Lietuva;
pub(super) const fn get_lt_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** dar nepalaikomas [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Konvertuoti iš „lisp s-ekspression“ į „kitus formatus“[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNežinomas failo formatas[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mŠiuo metu palaikomų formatų sąrašas:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNepavyko automatiškai aptikti formato.Nurodykite rankiniu būdu.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mŠis dvejetainis ** neapima ** atitinkamo formato konvertavimo funkcijos.
[48;2;34;34;34m[38;2;255;255;255mTurite įgalinti atitinkamą funkciją „Cargo.toml“ ir perkompiliuoti.
[48;2;34;34;34m[38;2;255;255;255mJei šioje programinėje įrangoje nėra atitinkamos funkcijos, pateikite problemą.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonversijos klaida[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mpaskirties failo kelias[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNepalaikomas formato konvertavimas[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFailas nėra galiojantis „JSON 1.0“ formatas, bandantis išanalizuoti kaip „JSON5“ ...[0m"##),
    ],
}
}

/// Language ID: lt;
/// Map name: "set";
/// Description: lietuvių, lotynų, Lietuva;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Neteisingas failo kelias.");
/// ```
pub(super) const fn get_lt_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Modifikuotas turinys ** nebus išsaugotas, nes „--sv“ nebuvo iškviestas."##),
        ("new-value", r##"Nauja vertė"##),
        ("invalid-path", r##"Neteisingas failo kelias."##),
    ],
}
}

/// Language ID: lt;
/// Map name: "set_md";
/// Description: lietuvių, lotynų, Lietuva;
pub(super) const fn get_lt_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mModifikuotas turinys ** nebus išsaugotas, nes „--sv“ nebuvo iškviestas.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNauja vertė[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNeteisingas failo kelias.[0m"##),
    ],
}
}

/// Language ID: lt;
/// Map name: "get";
/// Description: lietuvių, lotynų, Lietuva;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Paskirties formatas");
/// ```
pub(super) const fn get_lt_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Paskirties formatas"##),
        ("src-fmt", r##"Šaltinio failo formatas"##),
    ],
}
}

/// lt: lietuvių, lotynų, Lietuva
pub(super) const fn get_lt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_lt_map_conversion),
        ("get", get_lt_map_get),
        ("set_md", get_lt_map_set_md),
        ("set", get_lt_map_set),
        ("conversion_md", get_lt_map_conversion_md),
    ],
}
}

/// Language ID: lv;
/// Map name: "conversion";
/// Description: latviešu, latīņu, Latvija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Neizdevās automātiski noteikt formātu.Lūdzu, norādiet manuāli.");
/// ```
pub(super) const fn get_lv_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Fails nav derīgs `JSON 1.0` formāts, mēģinot parsēt kā `json5` ..."##),
        ("not-included", r##"Šis binārais ** neietver ** attiecīgā formāta pārveidošanas funkcionalitāti.
Jums jāiespējo attiecīgā funkcija savā `Cargo.toml` un atkārtoti kompilējiet.
Ja šī programmatūra neietver atbilstošo funkcionalitāti, lūdzu, iesniedziet problēmu."##),
        ("currently-supported", r##"Pašlaik atbalstīto formātu saraksts:"##),
        ("unsupported", r##"neatbalstīts formāta konvertēšana"##),
        ("auto-detection-failed", r##"Neizdevās automātiski noteikt formātu.Lūdzu, norādiet manuāli."##),
        ("not-support-deser-sexp", r##"** Pagaidām nav atbalstīts **: pārveidošana no `lisp s-ekspresijas` uz" citiem formātiem ""##),
        ("unknown-fmt", r##"nezināms faila formāts"##),
        ("not-saved", r##"Šis saturs ** netiks saglabāts, jo `--save` netika izsaukts."##),
        ("dst", r##"mērķa faila ceļš"##),
        ("conv-error", r##"reklāmguvumu kļūda"##),
    ],
}
}

/// Language ID: lv;
/// Map name: "conversion_md";
/// Description: latviešu, latīņu, Latvija;
pub(super) const fn get_lv_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFails nav derīgs [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m formāts, mēģinot parsēt kā [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mŠis binārais ** neietver ** attiecīgā formāta pārveidošanas funkcionalitāti.
[48;2;34;34;34m[38;2;255;255;255mJums jāiespējo attiecīgā funkcija savā [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m un atkārtoti kompilējiet.
[48;2;34;34;34m[38;2;255;255;255mJa šī programmatūra neietver atbilstošo funkcionalitāti, lūdzu, iesniedziet problēmu.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mPašlaik atbalstīto formātu saraksts:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mneatbalstīts formāta konvertēšana[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNeizdevās automātiski noteikt formātu.Lūdzu, norādiet manuāli.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Pagaidām nav atbalstīts [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: pārveidošana no [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-ekspresijas[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m uz" citiem formātiem "[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mnezināms faila formāts[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mŠis saturs ** netiks saglabāts, jo [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m netika izsaukts.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mmērķa faila ceļš[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mreklāmguvumu kļūda[0m"##),
    ],
}
}

/// Language ID: lv;
/// Map name: "set";
/// Description: latviešu, latīņu, Latvija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "nederīgs faila ceļš.");
/// ```
pub(super) const fn get_lv_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Modificētais saturs ** netiks saglabāts, jo `--sv` netika izsaukts."##),
        ("new-value", r##"jauna vērtība"##),
        ("invalid-path", r##"nederīgs faila ceļš."##),
    ],
}
}

/// Language ID: lv;
/// Map name: "set_md";
/// Description: latviešu, latīņu, Latvija;
pub(super) const fn get_lv_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mModificētais saturs ** netiks saglabāts, jo [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m netika izsaukts.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mjauna vērtība[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mnederīgs faila ceļš.[0m"##),
    ],
}
}

/// Language ID: lv;
/// Map name: "get";
/// Description: latviešu, latīņu, Latvija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Galamērķa formāts");
/// ```
pub(super) const fn get_lv_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Galamērķa formāts"##),
        ("src-fmt", r##"Avota faila formāts"##),
    ],
}
}

/// lv: latviešu, latīņu, Latvija
pub(super) const fn get_lv_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_lv_map_conversion),
        ("get", get_lv_map_get),
        ("set_md", get_lv_map_set_md),
        ("set", get_lv_map_set),
        ("conversion_md", get_lv_map_conversion_md),
    ],
}
}

/// Language ID: mg;
/// Map name: "conversion";
/// Description: Malagasy, Latn, Madagasikara;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "tsy nahavita namantatra ny format.Azafady, farito amin'ny fomba manual.");
/// ```
pub(super) const fn get_mg_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (2, 1),
        (5, 0),
    ],
    entries: &[
        ("unknown-fmt", r##"Format rakitra tsy fantatra"##),
        ("unsupported", r##"fiovam-po tsy azo antoka"##),
        ("dst", r##"Lalana rakitra toerana"##),
        ("invalid-json1.0", r##"Ny rakitra dia tsy ny endrika 'JSON 1.0'"##),
        ("not-included", r##"Ity binary ity ** tsy ahitana ny fiasa ** ny fiovam-po amin'ny endrika mifanaraka amin'izany.
Mila mamela ny endri-javatra mifanaraka amin'ny `Cargo.toml` ianao ary averina.
Raha tsy misy ny rindranasa mifanaraka amin'izany dia azafady alefaso ny olana iray."##),
        ("auto-detection-failed", r##"tsy nahavita namantatra ny format.Azafady, farito amin'ny fomba manual."##),
        ("conv-error", r##"Fahadisoana fiovam-po"##),
        ("currently-supported", r##"misy lisitra misy eo amin'ny endrika ankehitriny:"##),
    ],
}
}

/// Language ID: mg;
/// Map name: "conversion_md";
/// Description: Malagasy, Latn, Madagasikara;
pub(super) const fn get_mg_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (2, 1),
        (5, 0),
    ],
    entries: &[
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormat rakitra tsy fantatra[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mfiovam-po tsy azo antoka[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mLalana rakitra toerana[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mNy rakitra dia tsy ny endrika 'JSON 1.0'[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mIty binary ity ** tsy ahitana ny fiasa ** ny fiovam-po amin'ny endrika mifanaraka amin'izany.
[48;2;34;34;34m[38;2;255;255;255mMila mamela ny endri-javatra mifanaraka amin'ny [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ianao ary averina.
[48;2;34;34;34m[38;2;255;255;255mRaha tsy misy ny rindranasa mifanaraka amin'izany dia azafady alefaso ny olana iray.[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mtsy nahavita namantatra ny format.Azafady, farito amin'ny fomba manual.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mFahadisoana fiovam-po[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mmisy lisitra misy eo amin'ny endrika ankehitriny:[0m"##),
    ],
}
}

/// Language ID: mg;
/// Map name: "set";
/// Description: Malagasy, Latn, Madagasikara;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "lalana tsy mety.");
/// ```
pub(super) const fn get_mg_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Ny votoaty novaina ** dia tsy ho voavonjy satria tsy nantsoina ny `--sv`."##),
        ("new-value", r##"Sarobidy vaovao"##),
        ("invalid-path", r##"lalana tsy mety."##),
    ],
}
}

/// Language ID: mg;
/// Map name: "set_md";
/// Description: Malagasy, Latn, Madagasikara;
pub(super) const fn get_mg_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mNy votoaty novaina ** dia tsy ho voavonjy satria tsy nantsoina ny [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mSarobidy vaovao[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mlalana tsy mety.[0m"##),
    ],
}
}

/// Language ID: mg;
/// Map name: "get";
/// Description: Malagasy, Latn, Madagasikara;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format format");
/// ```
pub(super) const fn get_mg_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format format"##),
        ("src-fmt", r##"Format rakitra loharano"##),
    ],
}
}

/// mg: Malagasy, Latn, Madagasikara
pub(super) const fn get_mg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_mg_map_conversion),
        ("get", get_mg_map_get),
        ("set_md", get_mg_map_set_md),
        ("set", get_mg_map_set),
        ("conversion_md", get_mg_map_conversion_md),
    ],
}
}

/// Language ID: mi;
/// Map name: "conversion";
/// Description: Māori, Rātina, Aotearoa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "I rahua te tirotiro aunoa i te whakatakotoranga.Tena koa tohuhia.");
/// ```
pub(super) const fn get_mi_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** kaore ano kia tautokohia **: te huri mai i te 'Lisp S"##),
        ("unknown-fmt", r##"Hōputu Kōnae Kaore e mohiotia"##),
        ("currently-supported", r##"I tenei wa kua tautokohia te raarangi whakatakotoranga:"##),
        ("auto-detection-failed", r##"I rahua te tirotiro aunoa i te whakatakotoranga.Tena koa tohuhia."##),
        ("not-included", r##"Ko tenei ruau ** kaore i te whakauru ** te mahinga whakawhitinga mo te whakatakotoranga e tika ana.
Me whakahoihia e koe te ahuatanga e tika ana i roto i to `Cargo.toml` me te hoko mai.
Mena kaore i te whakauru atu tenei raupaparorohiko ki te mahinga e tika ana, tukuna mai he take."##),
        ("conv-error", r##"Ko te hapa faafariu"##),
        ("dst", r##"Te Waehere Kōnae"##),
        ("unsupported", r##"Ko te hurihanga hōputu kore tautokona"##),
        ("invalid-json1.0", r##"Ehara i te konae he whakatakotoranga tika a Json 1.0``````], e ngana ana ki te tarai i te 'JSON5` ..."##),
    ],
}
}

/// Language ID: mi;
/// Map name: "conversion_md";
/// Description: Māori, Rātina, Aotearoa;
pub(super) const fn get_mi_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** kaore ano kia tautokohia [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: te huri mai i te 'Lisp S[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mHōputu Kōnae Kaore e mohiotia[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mI tenei wa kua tautokohia te raarangi whakatakotoranga:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mI rahua te tirotiro aunoa i te whakatakotoranga.Tena koa tohuhia.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mKo tenei ruau ** kaore i te whakauru ** te mahinga whakawhitinga mo te whakatakotoranga e tika ana.
[48;2;34;34;34m[38;2;255;255;255mMe whakahoihia e koe te ahuatanga e tika ana i roto i to [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m me te hoko mai.
[48;2;34;34;34m[38;2;255;255;255mMena kaore i te whakauru atu tenei raupaparorohiko ki te mahinga e tika ana, tukuna mai he take.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKo te hapa faafariu[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mTe Waehere Kōnae[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mKo te hurihanga hōputu kore tautokona[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mEhara i te konae he whakatakotoranga tika a Json 1.0[48;2;34;34;34m[38;2;0;255;255m``````[48;2;34;34;34m[38;2;0;255;255m], e ngana ana ki te tarai i te 'JSON5` ...[0m"##),
    ],
}
}

/// Language ID: mi;
/// Map name: "set";
/// Description: Māori, Rātina, Aotearoa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Aratau Kōnae kore.");
/// ```
pub(super) const fn get_mi_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Ko te ihirangi kua whakarerekehia ** kaore e ora * na te mea kaore i karangahia."##),
        ("new-value", r##"Te uara hou"##),
        ("invalid-path", r##"Aratau Kōnae kore."##),
    ],
}
}

/// Language ID: mi;
/// Map name: "set_md";
/// Description: Māori, Rātina, Aotearoa;
pub(super) const fn get_mi_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mKo te ihirangi kua whakarerekehia ** kaore e ora * na te mea kaore i karangahia.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mTe uara hou[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mAratau Kōnae kore.[0m"##),
    ],
}
}

/// Language ID: mi;
/// Map name: "get";
/// Description: Māori, Rātina, Aotearoa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Hōputu ūnga");
/// ```
pub(super) const fn get_mi_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Hōputu ūnga"##),
        ("src-fmt", r##"Hōputu konae"##),
    ],
}
}

/// mi: Māori, Rātina, Aotearoa
pub(super) const fn get_mi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_mi_map_conversion),
        ("get", get_mi_map_get),
        ("set_md", get_mi_map_set_md),
        ("set", get_mi_map_set),
        ("conversion_md", get_mi_map_conversion_md),
    ],
}
}

/// Language ID: mk;
/// Map name: "conversion";
/// Description: македонски, кирилско писмо, Северна Македонија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Не успеа автоматски да го открие форматот.Ве молиме, наведете рачно.");
/// ```
pub(super) const fn get_mk_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Датотеката не е валиден формат `JSON 1.0`, обидувајќи се да се анализира како `json5` ..."##),
        ("not-included", r##"Оваа бинарна ** не вклучува ** функционалност за конверзија за релевантниот формат.
Треба да ја овозможите релевантната карактеристика во вашиот `Cargo.toml` и да се рекомпилот.
Ако овој софтвер не ја вклучува соодветната функционалност, ве молиме доставете проблем."##),
        ("currently-supported", r##"Тековно поддржан список со формати:"##),
        ("unsupported", r##"Неподдржана конверзија на формат"##),
        ("auto-detection-failed", r##"Не успеа автоматски да го открие форматот.Ве молиме, наведете рачно."##),
        ("not-support-deser-sexp", r##"** Сè уште не е поддржано **: Конвертирање од `LISP S-експресија` во„ други формати “"##),
        ("unknown-fmt", r##"Непознат формат на датотека"##),
        ("not-saved", r##"Следната содржина ** нема да се зачува затоа што не беше повикана `--save`."##),
        ("dst", r##"патека за датотека за дестинација"##),
        ("conv-error", r##"Грешка во конверзија"##),
    ],
}
}

/// Language ID: mk;
/// Map name: "conversion_md";
/// Description: македонски, кирилско писмо, Северна Македонија;
pub(super) const fn get_mk_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mДатотеката не е валиден формат [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, обидувајќи се да се анализира како [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mОваа бинарна ** не вклучува ** функционалност за конверзија за релевантниот формат.
[48;2;34;34;34m[38;2;255;255;255mТреба да ја овозможите релевантната карактеристика во вашиот [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m и да се рекомпилот.
[48;2;34;34;34m[38;2;255;255;255mАко овој софтвер не ја вклучува соодветната функционалност, ве молиме доставете проблем.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mТековно поддржан список со формати:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mНеподдржана конверзија на формат[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mНе успеа автоматски да го открие форматот.Ве молиме, наведете рачно.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Сè уште не е поддржано [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Конвертирање од [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLISP S-експресија[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m во„ други формати “[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mНепознат формат на датотека[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mСледната содржина ** нема да се зачува затоа што не беше повикана [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mпатека за датотека за дестинација[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mГрешка во конверзија[0m"##),
    ],
}
}

/// Language ID: mk;
/// Map name: "set";
/// Description: македонски, кирилско писмо, Северна Македонија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Невалидна патека за датотеки.");
/// ```
pub(super) const fn get_mk_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Изменетата содржина ** нема да се зачува ** затоа што `--sv` не беше повикана."##),
        ("new-value", r##"нова вредност"##),
        ("invalid-path", r##"Невалидна патека за датотеки."##),
    ],
}
}

/// Language ID: mk;
/// Map name: "set_md";
/// Description: македонски, кирилско писмо, Северна Македонија;
pub(super) const fn get_mk_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mИзменетата содржина ** нема да се зачува ** затоа што [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m не беше повикана.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mнова вредност[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mНевалидна патека за датотеки.[0m"##),
    ],
}
}

/// Language ID: mk;
/// Map name: "get";
/// Description: македонски, кирилско писмо, Северна Македонија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Формат на дестинација");
/// ```
pub(super) const fn get_mk_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Формат на дестинација"##),
        ("src-fmt", r##"Формат на изворна датотека"##),
    ],
}
}

/// mk: македонски, кирилско писмо, Северна Македонија
pub(super) const fn get_mk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_mk_map_conversion),
        ("get", get_mk_map_get),
        ("set_md", get_mk_map_set_md),
        ("set", get_mk_map_set),
        ("conversion_md", get_mk_map_conversion_md),
    ],
}
}

/// Language ID: ml;
/// Map name: "conversion";
/// Description: മലയാളം, മലയാളം, ഇന്ത്യ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ഫോർമ\u{d3e}റ\u{d4d}റ\u{d4d} സ\u{d4d}വപ\u{d4d}രേരിതമ\u{d3e}യി കണ\u{d4d}ടെത\u{d4d}ത\u{d41}ന\u{d4d}നതിൽ പര\u{d3e}ജയപ\u{d4d}പെട\u{d4d}ട\u{d41}.സ\u{d4d}വമേധയ\u{d3e} വ\u{d4d}യക\u{d4d}തമ\u{d3e}ക\u{d4d}ക\u{d41}ക.");
/// ```
pub(super) const fn get_ml_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** പിന്തുണയ്ക്കുന്നില്ല **: `ലിസ്പ് എസ്-എക്സ്പ്രഷനിൽ നിന്ന്` മറ്റ് ഫോർമാറ്റ്സ്` വരെ പരിവർത്തനം ചെയ്യുന്നു"##),
        ("unknown-fmt", r##"അജ്ഞാത ഫയൽ ഫോർമാറ്റ്"##),
        ("currently-supported", r##"നിലവിൽ പിന്തുണയ്ക്കുന്ന ഫോർമാറ്റ് ലിസ്റ്റ്:"##),
        ("auto-detection-failed", r##"ഫോർമാറ്റ് സ്വപ്രേരിതമായി കണ്ടെത്തുന്നതിൽ പരാജയപ്പെട്ടു.സ്വമേധയാ വ്യക്തമാക്കുക."##),
        ("not-included", r##"ഈ ബൈനറി ** ഉൾപ്പെടുന്നില്ല ** പ്രസക്തമായ ഫോർമാറ്റിനുള്ള പരിവർത്തന പ്രവർത്തനം.
നിങ്ങളുടെ ``argo.toml` ടു റെയിൻസിഫും പ്രസക്തമായ സവിശേഷത പ്രവർത്തനക്ഷമമാക്കേണ്ടതുണ്ട്.
ഈ സോഫ്റ്റ്വെയറിനെ അനുബന്ധ പ്രവർത്തനം ഉൾപ്പെടുത്തിയിട്ടില്ലെങ്കിൽ, ദയവായി ഒരു പ്രശ്നം സമർപ്പിക്കുക."##),
        ("conv-error", r##"പരിവർത്തന പിശക്"##),
        ("dst", r##"ഡെസ്റ്റിനേഷൻ ഫയൽ പാത്ത്"##),
        ("unsupported", r##"പിന്തുണയ്ക്കാത്ത ഫോർമാറ്റ് പരിവർത്തനം"##),
        ("invalid-json1.0", r##"ഫയൽ ഒരു സാധുവായ `JSON 1.0` ഫോർമാല്ല, `JSON5` എന്ന് പാഴ്സുചെയ്യാൻ ശ്രമിക്കുന്നു."##),
    ],
}
}

/// Language ID: ml;
/// Map name: "conversion_md";
/// Description: മലയാളം, മലയാളം, ഇന്ത്യ;
pub(super) const fn get_ml_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** പിന്തുണയ്ക്കുന്നില്ല [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mലിസ്പ് എസ്-എക്സ്പ്രഷനിൽ നിന്ന്[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m മറ്റ് ഫോർമാറ്റ്സ്[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m വരെ പരിവർത്തനം ചെയ്യുന്നു[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mഅജ്ഞാത ഫയൽ ഫോർമാറ്റ്[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mനിലവിൽ പിന്തുണയ്ക്കുന്ന ഫോർമാറ്റ് ലിസ്റ്റ്:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mഫോർമാറ്റ് സ്വപ്രേരിതമായി കണ്ടെത്തുന്നതിൽ പരാജയപ്പെട്ടു.സ്വമേധയാ വ്യക്തമാക്കുക.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mഈ ബൈനറി ** ഉൾപ്പെടുന്നില്ല ** പ്രസക്തമായ ഫോർമാറ്റിനുള്ള പരിവർത്തന പ്രവർത്തനം.
[48;2;34;34;34m[38;2;255;255;255mനിങ്ങളുടെ [48;2;34;34;34m[38;2;0;255;255m``[48;2;34;34;34m[38;2;0;255;255margo.toml` ടു റെയിൻസിഫും പ്രസക്തമായ സവിശേഷത പ്രവർത്തനക്ഷമമാക്കേണ്ടതുണ്ട്.
[48;2;34;34;34m[38;2;0;255;255mഈ സോഫ്റ്റ്വെയറിനെ അനുബന്ധ പ്രവർത്തനം ഉൾപ്പെടുത്തിയിട്ടില്ലെങ്കിൽ, ദയവായി ഒരു പ്രശ്നം സമർപ്പിക്കുക.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mപരിവർത്തന പിശക്[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mഡെസ്റ്റിനേഷൻ ഫയൽ പാത്ത്[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mപിന്തുണയ്ക്കാത്ത ഫോർമാറ്റ് പരിവർത്തനം[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mഫയൽ ഒരു സാധുവായ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ഫോർമാല്ല, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m എന്ന് പാഴ്സുചെയ്യാൻ ശ്രമിക്കുന്നു.[0m"##),
    ],
}
}

/// Language ID: ml;
/// Map name: "set";
/// Description: മലയാളം, മലയാളം, ഇന്ത്യ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "അസ\u{d3e}ധ\u{d41}വ\u{d3e}യ ഫയൽ പ\u{d3e}ത\u{d4d}ത\u{d4d}.");
/// ```
pub(super) const fn get_ml_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"പരിഷ്ക്കരിച്ച ഉള്ളടക്കം ** `--sv 'എന്ന് വിളിക്കാത്തതിനാൽ ** സംരക്ഷിക്കരുത്."##),
        ("new-value", r##"പുതിയ മൂല്യം"##),
        ("invalid-path", r##"അസാധുവായ ഫയൽ പാത്ത്."##),
    ],
}
}

/// Language ID: ml;
/// Map name: "set_md";
/// Description: മലയാളം, മലയാളം, ഇന്ത്യ;
pub(super) const fn get_ml_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mപരിഷ്ക്കരിച്ച ഉള്ളടക്കം ** [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv 'എന്ന് വിളിക്കാത്തതിനാൽ ** സംരക്ഷിക്കരുത്.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mപുതിയ മൂല്യം[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mഅസാധുവായ ഫയൽ പാത്ത്.[0m"##),
    ],
}
}

/// Language ID: ml;
/// Map name: "get";
/// Description: മലയാളം, മലയാളം, ഇന്ത്യ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ലക\u{d4d}ഷ\u{d4d}യ ഫോർമ\u{d3e}റ\u{d4d}റ\u{d4d}");
/// ```
pub(super) const fn get_ml_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ലക്ഷ്യ ഫോർമാറ്റ്"##),
        ("src-fmt", r##"ഉറവിട ഫയൽ ഫോർമാറ്റ്"##),
    ],
}
}

/// ml: മലയാളം, മലയാളം, ഇന്ത്യ
pub(super) const fn get_ml_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ml_map_conversion),
        ("get", get_ml_map_get),
        ("set_md", get_ml_map_set_md),
        ("set", get_ml_map_set),
        ("conversion_md", get_ml_map_conversion_md),
    ],
}
}

/// Language ID: mn;
/// Map name: "conversion";
/// Description: монгол, кирилл, Монгол;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Форматыг автоматаар илрүүлж чадсангүй.Гараар тодорхойлно уу.");
/// ```
pub(super) const fn get_mn_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"Хөрвүүлэх алдаа"##),
        ("not-support-deser-sexp", r##"** дэмжигдээгүй байгаа хараахан дэмжигдээгүй байна **: `LISP S илэрхийлэх 'нь` Бусад Formats` руу хөрвүүлэх"##),
        ("dst", r##"Зорилтот файлын зам"##),
        ("unsupported", r##"Дэмжигдээгүй форматыг хөрвүүлэх"##),
        ("auto-detection-failed", r##"Форматыг автоматаар илрүүлж чадсангүй.Гараар тодорхойлно уу."##),
        ("currently-supported", r##"Одоогийн байдлаар дэмжигдсэн форматын жагсаалт:"##),
        ("not-included", r##"Энэ хоёртын ** нь холбогдох форматын хувьд хувиргах функцийг агуулаагүй байна.
Та `ачааны` ачааны.toml` ба Recompile-д холбогдох функцийг идэвхжүүлэх хэрэгтэй.
Хэрэв энэ програм нь холбогдох функцийг оруулаагүй бол асуудлыг илгээнэ үү."##),
        ("unknown-fmt", r##"File File Forment"##),
    ],
}
}

/// Language ID: mn;
/// Map name: "conversion_md";
/// Description: монгол, кирилл, Монгол;
pub(super) const fn get_mn_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mХөрвүүлэх алдаа[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** дэмжигдээгүй байгаа хараахан дэмжигдээгүй байна [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLISP S илэрхийлэх 'нь[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m Бусад Formats[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m руу хөрвүүлэх[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mЗорилтот файлын зам[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mДэмжигдээгүй форматыг хөрвүүлэх[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mФорматыг автоматаар илрүүлж чадсангүй.Гараар тодорхойлно уу.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mОдоогийн байдлаар дэмжигдсэн форматын жагсаалт:[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mЭнэ хоёртын ** нь холбогдох форматын хувьд хувиргах функцийг агуулаагүй байна.
[48;2;34;34;34m[38;2;255;255;255mТа [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mачааны[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ачааны.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ба Recompile-д холбогдох функцийг идэвхжүүлэх хэрэгтэй.
[48;2;34;34;34m[38;2;0;255;255mХэрэв энэ програм нь холбогдох функцийг оруулаагүй бол асуудлыг илгээнэ үү.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFile File Forment[0m"##),
    ],
}
}

/// Language ID: mn;
/// Map name: "set";
/// Description: монгол, кирилл, Монгол;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Буруу файлын зам буруу байна.");
/// ```
pub(super) const fn get_mn_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Өөрчлөгдсөн агуулга ** -г өөрчлөх ** -ыг ** -г хадгалахгүй."##),
        ("new-value", r##"Шинэ утга"##),
        ("invalid-path", r##"Буруу файлын зам буруу байна."##),
    ],
}
}

/// Language ID: mn;
/// Map name: "set_md";
/// Description: монгол, кирилл, Монгол;
pub(super) const fn get_mn_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mӨөрчлөгдсөн агуулга ** -г өөрчлөх ** -ыг ** -г хадгалахгүй.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mШинэ утга[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mБуруу файлын зам буруу байна.[0m"##),
    ],
}
}

/// Language ID: mn;
/// Map name: "get";
/// Description: монгол, кирилл, Монгол;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Эрхэм хэсэг");
/// ```
pub(super) const fn get_mn_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Эрхэм хэсэг"##),
        ("src-fmt", r##"Эх файл файлын формат"##),
    ],
}
}

/// mn: монгол, кирилл, Монгол
pub(super) const fn get_mn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_mn_map_conversion),
        ("get", get_mn_map_get),
        ("set_md", get_mn_map_set_md),
        ("set", get_mn_map_set),
        ("conversion_md", get_mn_map_conversion_md),
    ],
}
}

/// Language ID: mr;
/// Map name: "conversion";
/// Description: मराठी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "स\u{94d}वर\u{942}प स\u{94d}वय\u{902}चलितपण\u{947} शोधण\u{94d}यात अयशस\u{94d}वी.क\u{943}पया व\u{94d}यक\u{94d}तिचलितपण\u{947} निर\u{94d}दिष\u{94d}ट करा.");
/// ```
pub(super) const fn get_mr_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** अद्याप समर्थित नाही **: `लिस्प एस-एक्सप्रेशन` पासून `इतर स्वरूपात रूपांतरित करणे"##),
        ("unknown-fmt", r##"अज्ञात फाइल स्वरूप"##),
        ("currently-supported", r##"सध्या समर्थित स्वरूपांची यादी:"##),
        ("auto-detection-failed", r##"स्वरूप स्वयंचलितपणे शोधण्यात अयशस्वी.कृपया व्यक्तिचलितपणे निर्दिष्ट करा."##),
        ("not-included", r##"या बायनरी ** मध्ये संबंधित स्वरूपासाठी ** रूपांतरण कार्यक्षमता समाविष्ट नाही.
आपल्याला आपल्या `Caggo.toml` आणि recompile मधील संबंधित वैशिष्ट्य सक्षम करण्याची आवश्यकता आहे.
या सॉफ्टवेअरमध्ये संबंधित कार्यक्षमता समाविष्ट नसल्यास, कृपया एक समस्या सबमिट करा."##),
        ("conv-error", r##"रूपांतरण त्रुटी"##),
        ("dst", r##"गंतव्य फाइल पथ"##),
        ("unsupported", r##"असमर्थित स्वरूप रूपांतरण"##),
        ("invalid-json1.0", r##"फाईल वैध नाही `json 1.0` स्वरूप, `json5` म्हणून विश्लेषित करण्याचा प्रयत्न करीत आहे ..."##),
    ],
}
}

/// Language ID: mr;
/// Map name: "conversion_md";
/// Description: मराठी, देवनागरी, भारत;
pub(super) const fn get_mr_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** अद्याप समर्थित नाही [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mलिस्प एस-एक्सप्रेशन[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m पासून [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mइतर स्वरूपात रूपांतरित करणे[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mअज्ञात फाइल स्वरूप[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mसध्या समर्थित स्वरूपांची यादी:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mस्वरूप स्वयंचलितपणे शोधण्यात अयशस्वी.कृपया व्यक्तिचलितपणे निर्दिष्ट करा.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mया बायनरी ** मध्ये संबंधित स्वरूपासाठी ** रूपांतरण कार्यक्षमता समाविष्ट नाही.
[48;2;34;34;34m[38;2;255;255;255mआपल्याला आपल्या [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCaggo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m आणि recompile मधील संबंधित वैशिष्ट्य सक्षम करण्याची आवश्यकता आहे.
[48;2;34;34;34m[38;2;255;255;255mया सॉफ्टवेअरमध्ये संबंधित कार्यक्षमता समाविष्ट नसल्यास, कृपया एक समस्या सबमिट करा.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mरूपांतरण त्रुटी[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mगंतव्य फाइल पथ[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mअसमर्थित स्वरूप रूपांतरण[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mफाईल वैध नाही [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m स्वरूप, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m म्हणून विश्लेषित करण्याचा प्रयत्न करीत आहे ...[0m"##),
    ],
}
}

/// Language ID: mr;
/// Map name: "set";
/// Description: मराठी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "अव\u{948}ध फाइल पथ.");
/// ```
pub(super) const fn get_mr_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"सुधारित सामग्री ** ** जतन केली जाणार नाही कारण `--एसव्हीओला कॉल केला गेला नाही."##),
        ("new-value", r##"नवीन मूल्य"##),
        ("invalid-path", r##"अवैध फाइल पथ."##),
    ],
}
}

/// Language ID: mr;
/// Map name: "set_md";
/// Description: मराठी, देवनागरी, भारत;
pub(super) const fn get_mr_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mसुधारित सामग्री ** ** जतन केली जाणार नाही कारण [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--एसव्हीओला कॉल केला गेला नाही.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mनवीन मूल्य[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mअवैध फाइल पथ.[0m"##),
    ],
}
}

/// Language ID: mr;
/// Map name: "get";
/// Description: मराठी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ग\u{902}तव\u{94d}य स\u{94d}वर\u{942}प");
/// ```
pub(super) const fn get_mr_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"गंतव्य स्वरूप"##),
        ("src-fmt", r##"स्त्रोत फाइल स्वरूप"##),
    ],
}
}

/// mr: मराठी, देवनागरी, भारत
pub(super) const fn get_mr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_mr_map_conversion),
        ("get", get_mr_map_get),
        ("set_md", get_mr_map_set_md),
        ("set", get_mr_map_set),
        ("conversion_md", get_mr_map_conversion_md),
    ],
}
}

/// Language ID: ms;
/// Map name: "conversion";
/// Description: Melayu, Latin, Malaysia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "gagal secara automatik mengesan format.Sila nyatakan secara manual.");
/// ```
pub(super) const fn get_ms_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Fail bukan format `json 1.0` yang sah, cuba menghuraikan `json5` ..."##),
        ("not-included", r##"Binari ini ** tidak termasuk ** fungsi penukaran untuk format yang berkaitan.
Anda perlu mengaktifkan ciri yang relevan dalam `Cargo.toml` dan recompile.
Jika perisian ini tidak termasuk fungsi yang sepadan, sila hantar isu."##),
        ("currently-supported", r##"Senarai Format yang disokong sekarang:"##),
        ("unsupported", r##"penukaran format yang tidak disokong"##),
        ("auto-detection-failed", r##"gagal secara automatik mengesan format.Sila nyatakan secara manual."##),
        ("not-support-deser-sexp", r##"** belum disokong **: menukar dari `lisp s-expression` ke` format lain`"##),
        ("unknown-fmt", r##"format fail yang tidak diketahui"##),
        ("not-saved", r##"kandungan berikut ** tidak akan ** disimpan kerana `--save` tidak dipanggil."##),
        ("dst", r##"laluan fail destinasi"##),
        ("conv-error", r##"ralat penukaran"##),
    ],
}
}

/// Language ID: ms;
/// Map name: "conversion_md";
/// Description: Melayu, Latin, Malaysia;
pub(super) const fn get_ms_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFail bukan format [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m yang sah, cuba menghuraikan [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinari ini ** tidak termasuk ** fungsi penukaran untuk format yang berkaitan.
[48;2;34;34;34m[38;2;255;255;255mAnda perlu mengaktifkan ciri yang relevan dalam [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m dan recompile.
[48;2;34;34;34m[38;2;255;255;255mJika perisian ini tidak termasuk fungsi yang sepadan, sila hantar isu.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mSenarai Format yang disokong sekarang:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mpenukaran format yang tidak disokong[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mgagal secara automatik mengesan format.Sila nyatakan secara manual.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** belum disokong [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: menukar dari [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ke[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m format lain[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mformat fail yang tidak diketahui[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mkandungan berikut ** tidak akan ** disimpan kerana [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m tidak dipanggil.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mlaluan fail destinasi[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mralat penukaran[0m"##),
    ],
}
}

/// Language ID: ms;
/// Map name: "set";
/// Description: Melayu, Latin, Malaysia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "jalur fail tidak sah.");
/// ```
pub(super) const fn get_ms_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Kandungan yang diubahsuai ** tidak akan disimpan kerana `--sv` tidak dipanggil."##),
        ("new-value", r##"nilai baru"##),
        ("invalid-path", r##"jalur fail tidak sah."##),
    ],
}
}

/// Language ID: ms;
/// Map name: "set_md";
/// Description: Melayu, Latin, Malaysia;
pub(super) const fn get_ms_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mKandungan yang diubahsuai ** tidak akan disimpan kerana [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m tidak dipanggil.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnilai baru[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mjalur fail tidak sah.[0m"##),
    ],
}
}

/// Language ID: ms;
/// Map name: "get";
/// Description: Melayu, Latin, Malaysia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format Destinasi");
/// ```
pub(super) const fn get_ms_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format Destinasi"##),
        ("src-fmt", r##"Format fail sumber"##),
    ],
}
}

/// ms: Melayu, Latin, Malaysia
pub(super) const fn get_ms_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ms_map_conversion),
        ("get", get_ms_map_get),
        ("set_md", get_ms_map_set_md),
        ("set", get_ms_map_set),
        ("conversion_md", get_ms_map_conversion_md),
    ],
}
}

/// Language ID: mt;
/// Map name: "conversion";
/// Description: Malti, Latin, Malta;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "naqset milli tiskopri awtomatikament il-format.Jekk jogħġbok speċifika manwalment.");
/// ```
pub(super) const fn get_mt_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Il-fajl mhuwiex format validu `JSON 1.0`, li jipprova parse bħala `json5` ..."##),
        ("not-included", r##"Dan il-binarju ** ma jinkludix ** il-funzjonalità ta 'konverżjoni għall-format rilevanti.
Ikollok bżonn li tippermetti l-karatteristika rilevanti fil-`Cargo.toml` u terġa 'tittieħed.
Jekk dan is-softwer ma jinkludix il-funzjonalità korrispondenti, jekk jogħġbok ibgħat kwistjoni."##),
        ("currently-supported", r##"Lista tal-Formati Appoġġjati bħalissa:"##),
        ("unsupported", r##"konverżjoni ta 'format mhux appoġġjat"##),
        ("auto-detection-failed", r##"naqset milli tiskopri awtomatikament il-format.Jekk jogħġbok speċifika manwalment."##),
        ("not-support-deser-sexp", r##"** Mhux appoġġjat għadu"##),
        ("unknown-fmt", r##"Format tal-fajl mhux magħruf"##),
        ("not-saved", r##"Il-kontenut li ġej ** mhux se jkun iffrankat minħabba li `--save` ma kienx imsejjaħ."##),
        ("dst", r##"Path tal-fajl tad-destinazzjoni"##),
        ("conv-error", r##"żball ta 'konverżjoni"##),
    ],
}
}

/// Language ID: mt;
/// Map name: "conversion_md";
/// Description: Malti, Latin, Malta;
pub(super) const fn get_mt_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mIl-fajl mhuwiex format validu [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, li jipprova parse bħala [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDan il-binarju ** ma jinkludix ** il-funzjonalità ta 'konverżjoni għall-format rilevanti.
[48;2;34;34;34m[38;2;255;255;255mIkollok bżonn li tippermetti l-karatteristika rilevanti fil-[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m u terġa 'tittieħed.
[48;2;34;34;34m[38;2;255;255;255mJekk dan is-softwer ma jinkludix il-funzjonalità korrispondenti, jekk jogħġbok ibgħat kwistjoni.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLista tal-Formati Appoġġjati bħalissa:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mkonverżjoni ta 'format mhux appoġġjat[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mnaqset milli tiskopri awtomatikament il-format.Jekk jogħġbok speċifika manwalment.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Mhux appoġġjat għadu[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormat tal-fajl mhux magħruf[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mIl-kontenut li ġej ** mhux se jkun iffrankat minħabba li [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ma kienx imsejjaħ.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mPath tal-fajl tad-destinazzjoni[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mżball ta 'konverżjoni[0m"##),
    ],
}
}

/// Language ID: mt;
/// Map name: "set";
/// Description: Malti, Latin, Malta;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Path tal-fajl invalidu.");
/// ```
pub(super) const fn get_mt_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Il-kontenut modifikat ** mhux se jkun iffrankat minħabba li `--sv` ma kienx imsejjaħ."##),
        ("new-value", r##"Valur Ġdid"##),
        ("invalid-path", r##"Path tal-fajl invalidu."##),
    ],
}
}

/// Language ID: mt;
/// Map name: "set_md";
/// Description: Malti, Latin, Malta;
pub(super) const fn get_mt_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mIl-kontenut modifikat ** mhux se jkun iffrankat minħabba li [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ma kienx imsejjaħ.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mValur Ġdid[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mPath tal-fajl invalidu.[0m"##),
    ],
}
}

/// Language ID: mt;
/// Map name: "get";
/// Description: Malti, Latin, Malta;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format tad-destinazzjoni");
/// ```
pub(super) const fn get_mt_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format tad-destinazzjoni"##),
        ("src-fmt", r##"Format tal-fajl tas-sors"##),
    ],
}
}

/// mt: Malti, Latin, Malta
pub(super) const fn get_mt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_mt_map_conversion),
        ("get", get_mt_map_get),
        ("set_md", get_mt_map_set_md),
        ("set", get_mt_map_set),
        ("conversion_md", get_mt_map_conversion_md),
    ],
}
}

/// Language ID: my;
/// Map name: "conversion";
/// Description: မြန်မာ, မြန်မာ, မြန်မာ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "format က\u{102d}\u{102f}အလ\u{102d}\u{102f}အလျောက\u{103a}စစ\u{103a}ဆေးရန\u{103a}ပျက\u{103a}က\u{103d}က\u{103a}ခ\u{1032}\u{1037}သည\u{103a}။ကျေးဇ\u{1030}းပြ\u{102f}ပြ\u{102e}းက\u{102d}\u{102f}ယ\u{103a}တ\u{102d}\u{102f}င\u{103a}က\u{102d}\u{102f}ယ\u{103a}ကျဖော\u{103a}ပြပါ");
/// ```
pub(super) const fn get_my_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** ** မထောက်ပံ့ရသေးပါ။"##),
        ("unknown-fmt", r##"မသိသောဖိုင်ပုံစံ"##),
        ("currently-supported", r##"လက်ရှိတွင်ထောက်ပံ့ထားသောပုံစံများစာရင်း -"##),
        ("auto-detection-failed", r##"format ကိုအလိုအလျောက်စစ်ဆေးရန်ပျက်ကွက်ခဲ့သည်။ကျေးဇူးပြုပြီးကိုယ်တိုင်ကိုယ်ကျဖော်ပြပါ"##),
        ("not-included", r##"ဤ binary ** သည်သက်ဆိုင်ရာပုံစံအတွက်ပြောင်းလဲခြင်းလုပ်ဆောင်နိုင်စွမ်းကိုမပါ 0 င်ပါ။
သင်၏ `Cargo.toml` တွင်သက်ဆိုင်ရာအင်္ဂါရပ်ကို enable လုပ်ရန်လိုအပ်သည်။
အကယ်. ဤဆော့ (ဖ်) ဝဲသည်သက်ဆိုင်ရာလုပ်ဆောင်နိုင်စွမ်းမပါရှိပါက ကျေးဇူးပြု. ပြ an နာကိုတင်ပါ။"##),
        ("conv-error", r##"ကူးပြောင်းခြင်းအမှား"##),
        ("dst", r##"destination file path"##),
        ("unsupported", r##"unsupported format ပြောင်းလဲခြင်း"##),
        ("invalid-json1.0", r##"ဖိုင်သည်ခိုင်လုံသော `JSON 1.0` format မဟုတ်ဘဲ `JSON5 `` `json5` asse ကိုခွဲခြမ်းစိတ်ဖြာရန်ကြိုးစားနေသည်။"##),
    ],
}
}

/// Language ID: my;
/// Map name: "conversion_md";
/// Description: မြန်မာ, မြန်မာ, မြန်မာ;
pub(super) const fn get_my_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ** မထောက်ပံ့ရသေးပါ။[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mမသိသောဖိုင်ပုံစံ[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mလက်ရှိတွင်ထောက်ပံ့ထားသောပုံစံများစာရင်း -[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mformat ကိုအလိုအလျောက်စစ်ဆေးရန်ပျက်ကွက်ခဲ့သည်။ကျေးဇူးပြုပြီးကိုယ်တိုင်ကိုယ်ကျဖော်ပြပါ[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mဤ binary ** သည်သက်ဆိုင်ရာပုံစံအတွက်ပြောင်းလဲခြင်းလုပ်ဆောင်နိုင်စွမ်းကိုမပါ 0 င်ပါ။
[48;2;34;34;34m[38;2;255;255;255mသင်၏ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m တွင်သက်ဆိုင်ရာအင်္ဂါရပ်ကို enable လုပ်ရန်လိုအပ်သည်။
[48;2;34;34;34m[38;2;255;255;255mအကယ်. ဤဆော့ (ဖ်) ဝဲသည်သက်ဆိုင်ရာလုပ်ဆောင်နိုင်စွမ်းမပါရှိပါက ကျေးဇူးပြု. ပြ an နာကိုတင်ပါ။[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mကူးပြောင်းခြင်းအမှား[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mdestination file path[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255munsupported format ပြောင်းလဲခြင်း[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mဖိုင်သည်ခိုင်လုံသော [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m format မဟုတ်ဘဲ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5 `` [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m asse ကိုခွဲခြမ်းစိတ်ဖြာရန်ကြိုးစားနေသည်။[0m"##),
    ],
}
}

/// Language ID: my;
/// Map name: "set";
/// Description: မြန်မာ, မြန်မာ, မြန်မာ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "မမ\u{103e}န\u{103a}ကန\u{103a}သောဖ\u{102d}\u{102f}င\u{103a}လမ\u{103a}းကြောင\u{103a}း။");
/// ```
pub(super) const fn get_my_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ပြုပြင်ထားသောအကြောင်းအရာ ** ** ** ကိုကယ်တင်ခြင်းမရှိသေးပါ။"##),
        ("new-value", r##"အသစ်တန်ဖိုး"##),
        ("invalid-path", r##"မမှန်ကန်သောဖိုင်လမ်းကြောင်း။"##),
    ],
}
}

/// Language ID: my;
/// Map name: "set_md";
/// Description: မြန်မာ, မြန်မာ, မြန်မာ;
pub(super) const fn get_my_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mပြုပြင်ထားသောအကြောင်းအရာ ** ** ** ကိုကယ်တင်ခြင်းမရှိသေးပါ။[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mအသစ်တန်ဖိုး[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mမမှန်ကန်သောဖိုင်လမ်းကြောင်း။[0m"##),
    ],
}
}

/// Language ID: my;
/// Map name: "get";
/// Description: မြန်မာ, မြန်မာ, မြန်မာ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ဦး တည\u{103a}ချက\u{103a}ပ\u{102f}\u{1036}စ\u{1036}");
/// ```
pub(super) const fn get_my_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ဦး တည်ချက်ပုံစံ"##),
        ("src-fmt", r##"Source ဖိုင်ပုံစံ"##),
    ],
}
}

/// my: မြန်မာ, မြန်မာ, မြန်မာ
pub(super) const fn get_my_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_my_map_conversion),
        ("get", get_my_map_get),
        ("set_md", get_my_map_set_md),
        ("set", get_my_map_set),
        ("conversion_md", get_my_map_conversion_md),
    ],
}
}

/// Language ID: ne;
/// Map name: "conversion";
/// Description: नेपाली, देवानागरी, नेपाल;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "स\u{94d}वचालित र\u{942}पमा ढा\u{901}चा पत\u{94d}ता लगाउन असफल भयो।क\u{943}पया म\u{948}न\u{94d}य\u{941}अल\u{94d}ली निर\u{94d}दिष\u{94d}ट गर\u{94d}न\u{941}होस\u{94d}।");
/// ```
pub(super) const fn get_ne_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** / * समर्थित छैन **: `लिट्स एस-अभिव्यक्तिबाट रूपान्तरण गर्दै - अन्य ढाँचाहरू"##),
        ("unknown-fmt", r##"अज्ञात फाइल ढाँचा"##),
        ("currently-supported", r##"हाल प्रशस्त फोरट्स सूची प्रदान गर्दछ:"##),
        ("auto-detection-failed", r##"स्वचालित रूपमा ढाँचा पत्ता लगाउन असफल भयो।कृपया मैन्युअल्ली निर्दिष्ट गर्नुहोस्।"##),
        ("not-included", r##"यो बाइनरी ** ** सान्दर्भिक ढाँचाको लागि रूपान्तरण कार्यक्षमता समावेश गर्दैन।
तपाइँ तपाइँको `Change.tomll` र replatile मा सान्दर्भिक सुविधा सक्षम गर्न आवश्यक छ।
यदि यो सफ्टवेयरले सम्बन्धित कार्यक्षमता समावेश गर्दैन भने कृपया एक मुद्दा बुझाउनुहोस्।"##),
        ("conv-error", r##"रूपान्तरण त्रुटि"##),
        ("dst", r##"गन्तव्य फाइल पथ"##),
        ("unsupported", r##"असमर्थित ढाँचा रूपान्तरण"##),
        ("invalid-json1.0", r##"फाईल मान्य `jon 1.0` ormant jstee,` JNON5`` को रूपमा पार्स गर्न कोशिस गर्दै छैन,"##),
    ],
}
}

/// Language ID: ne;
/// Map name: "conversion_md";
/// Description: नेपाली, देवानागरी, नेपाल;
pub(super) const fn get_ne_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** / * समर्थित छैन [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mलिट्स एस-अभिव्यक्तिबाट रूपान्तरण गर्दै - अन्य ढाँचाहरू[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mअज्ञात फाइल ढाँचा[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mहाल प्रशस्त फोरट्स सूची प्रदान गर्दछ:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mस्वचालित रूपमा ढाँचा पत्ता लगाउन असफल भयो।कृपया मैन्युअल्ली निर्दिष्ट गर्नुहोस्।[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mयो बाइनरी ** ** सान्दर्भिक ढाँचाको लागि रूपान्तरण कार्यक्षमता समावेश गर्दैन।
[48;2;34;34;34m[38;2;255;255;255mतपाइँ तपाइँको [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mChange.tomll[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m र replatile मा सान्दर्भिक सुविधा सक्षम गर्न आवश्यक छ।
[48;2;34;34;34m[38;2;255;255;255mयदि यो सफ्टवेयरले सम्बन्धित कार्यक्षमता समावेश गर्दैन भने कृपया एक मुद्दा बुझाउनुहोस्।[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mरूपान्तरण त्रुटि[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mगन्तव्य फाइल पथ[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mअसमर्थित ढाँचा रूपान्तरण[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mफाईल मान्य [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjon 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ormant jstee,[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m JNON5`` को रूपमा पार्स गर्न कोशिस गर्दै छैन,[0m"##),
    ],
}
}

/// Language ID: ne;
/// Map name: "set";
/// Description: नेपाली, देवानागरी, नेपाल;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "अव\u{948}ध फाईल मार\u{94d}ग।");
/// ```
pub(super) const fn get_ne_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"परिमार्जित सामग्री ** बचत गर्नुहोस् ** बचत गर्नुहोस् किनकि `- HVV` भनिने छैन।"##),
        ("new-value", r##"नयाँ मान"##),
        ("invalid-path", r##"अवैध फाईल मार्ग।"##),
    ],
}
}

/// Language ID: ne;
/// Map name: "set_md";
/// Description: नेपाली, देवानागरी, नेपाल;
pub(super) const fn get_ne_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mपरिमार्जित सामग्री ** बचत गर्नुहोस् ** बचत गर्नुहोस् किनकि [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m- HVV[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m भनिने छैन।[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mनयाँ मान[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mअवैध फाईल मार्ग।[0m"##),
    ],
}
}

/// Language ID: ne;
/// Map name: "get";
/// Description: नेपाली, देवानागरी, नेपाल;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "गन\u{94d}तव\u{94d}य ढा\u{901}चा");
/// ```
pub(super) const fn get_ne_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"गन्तव्य ढाँचा"##),
        ("src-fmt", r##"स्रोत फाइल ढाँचा"##),
    ],
}
}

/// ne: नेपाली, देवानागरी, नेपाल
pub(super) const fn get_ne_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ne_map_conversion),
        ("get", get_ne_map_get),
        ("set_md", get_ne_map_set_md),
        ("set", get_ne_map_set),
        ("conversion_md", get_ne_map_conversion_md),
    ],
}
}

/// Language ID: nl;
/// Map name: "conversion";
/// Description: Nederlands, Latijns, Nederland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Automatische herkenning van het formaat is mislukt. Geef dit alstublieft handmatig aan.");
/// ```
pub(super) const fn get_nl_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Het bestand is geen geldig `json 1.0` formaat, poging om te parsen als `json5`..."##),
        ("not-included", r##"Deze software **bevat niet** de functionaliteit voor de conversie van het relevante formaat.
U moet de juiste functies inschakelen in uw `Cargo.toml` en opnieuw compileren.
Als deze software de bijbehorende functionaliteit niet bevat, dient u een probleemrapport in."##),
        ("currently-supported", r##"Lijst met momenteel ondersteunde formaten:"##),
        ("unsupported", r##"Niet-ondersteunde formaatconversie"##),
        ("auto-detection-failed", r##"Automatische herkenning van het formaat is mislukt. Geef dit alstublieft handmatig aan."##),
        ("not-support-deser-sexp", r##"**Nog niet ondersteund**: converteren van `Lisp S-Expressie` naar `andere formaten`"##),
        ("unknown-fmt", r##"Onbekend bestandsformaat"##),
        ("not-saved", r##"De volgende inhoud wordt **niet** opgeslagen omdat `--save` niet is aangeroepen."##),
        ("dst", r##"Bestemmingsbestandpad"##),
        ("conv-error", r##"Conversiefout"##),
    ],
}
}

/// Language ID: nl;
/// Map name: "conversion_md";
/// Description: Nederlands, Latijns, Nederland;
pub(super) const fn get_nl_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mHet bestand is geen geldig [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m formaat, poging om te parsen als [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDeze software [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mbevat niet[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m de functionaliteit voor de conversie van het relevante formaat.
[48;2;34;34;34m[38;2;255;255;255mU moet de juiste functies inschakelen in uw [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m en opnieuw compileren.
[48;2;34;34;34m[38;2;255;255;255mAls deze software de bijbehorende functionaliteit niet bevat, dient u een probleemrapport in.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLijst met momenteel ondersteunde formaten:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNiet-ondersteunde formaatconversie[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mAutomatische herkenning van het formaat is mislukt. Geef dit alstublieft handmatig aan.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mNog niet ondersteund[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: converteren van [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expressie[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m naar [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mandere formaten[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mOnbekend bestandsformaat[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mDe volgende inhoud wordt [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mniet[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m opgeslagen omdat [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m niet is aangeroepen.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mBestemmingsbestandpad[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mConversiefout[0m"##),
    ],
}
}

/// Language ID: nl;
/// Map name: "set";
/// Description: Nederlands, Latijns, Nederland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "niet-opgeslagen-waarschuwing");
///
/// assert_eq!(msg, "De gewijzigde inhoud **wordt niet** opgeslagen omdat `--sv` niet is aangeroepen.");
/// ```
pub(super) const fn get_nl_map_set() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("niet-opgeslagen-waarschuwing", r##"De gewijzigde inhoud **wordt niet** opgeslagen omdat `--sv` niet is aangeroepen."##),
        ("nieuwe-waarde", r##"Nieuwe waarde"##),
        ("ongeldig-pad", r##"Ongeldig bestandspad."##),
    ],
}
}

/// Language ID: nl;
/// Map name: "set_md";
/// Description: Nederlands, Latijns, Nederland;
pub(super) const fn get_nl_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("niet-opgeslagen-waarschuwing", r##"[48;2;34;34;34m[38;2;255;255;255mDe gewijzigde inhoud [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mwordt niet[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m opgeslagen omdat [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m niet is aangeroepen.[0m"##),
        ("nieuwe-waarde", r##"[48;2;34;34;34m[38;2;255;255;255mNieuwe waarde[0m"##),
        ("ongeldig-pad", r##"[48;2;34;34;34m[38;2;255;255;255mOngeldig bestandspad.[0m"##),
    ],
}
}

/// Language ID: nl;
/// Map name: "get";
/// Description: Nederlands, Latijns, Nederland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Bestemmingsformaat");
/// ```
pub(super) const fn get_nl_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Bestemmingsformaat"##),
        ("src-fmt", r##"Bronbestandsformaat"##),
    ],
}
}

/// nl: Nederlands, Latijns, Nederland
pub(super) const fn get_nl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_nl_map_conversion),
        ("get", get_nl_map_get),
        ("set_md", get_nl_map_set_md),
        ("set", get_nl_map_set),
        ("conversion_md", get_nl_map_conversion_md),
    ],
}
}

/// Language ID: no;
/// Map name: "conversion";
/// Description: norsk, latinsk, Norge;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Kunne ikke oppdage formatet automatisk.Vennligst spesifiser manuelt.");
/// ```
pub(super) const fn get_no_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Filen er ikke et gyldig `JSON 1.0` -format, og prøver å analysere som `JSON5` ..."##),
        ("not-included", r##"Denne binære ** inkluderer ikke ** konverteringsfunksjonaliteten for det aktuelle formatet.
Du må aktivere den aktuelle funksjonen i "lastet. Toml` og kompilen.
Hvis denne programvaren ikke inkluderer den tilsvarende funksjonaliteten, vennligst send inn et problem."##),
        ("currently-supported", r##"For øyeblikket støttede formater Liste:"##),
        ("unsupported", r##"ikke støttet formatkonvertering"##),
        ("auto-detection-failed", r##"Kunne ikke oppdage formatet automatisk.Vennligst spesifiser manuelt."##),
        ("not-support-deser-sexp", r##"** Ikke støttet ennå **: Konvertere fra `Lisp S-Expression` til` andre formater`"##),
        ("unknown-fmt", r##"Ukjent filformat"##),
        ("not-saved", r##"Følgende innhold ** vil ikke ** lagres fordi `--save` ikke ble kalt."##),
        ("dst", r##"Destinasjonsfilbane"##),
        ("conv-error", r##"konverteringsfeil"##),
    ],
}
}

/// Language ID: no;
/// Map name: "conversion_md";
/// Description: norsk, latinsk, Norge;
pub(super) const fn get_no_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFilen er ikke et gyldig [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m -format, og prøver å analysere som [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDenne binære ** inkluderer ikke ** konverteringsfunksjonaliteten for det aktuelle formatet.
[48;2;34;34;34m[38;2;255;255;255mDu må aktivere den aktuelle funksjonen i "lastet. Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m og kompilen.
[48;2;34;34;34m[38;2;0;255;255mHvis denne programvaren ikke inkluderer den tilsvarende funksjonaliteten, vennligst send inn et problem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mFor øyeblikket støttede formater Liste:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mikke støttet formatkonvertering[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mKunne ikke oppdage formatet automatisk.Vennligst spesifiser manuelt.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Ikke støttet ennå [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Konvertere fra [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m til[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m andre formater[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mUkjent filformat[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mFølgende innhold ** vil ikke ** lagres fordi [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ikke ble kalt.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestinasjonsfilbane[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mkonverteringsfeil[0m"##),
    ],
}
}

/// Language ID: no;
/// Map name: "set";
/// Description: norsk, latinsk, Norge;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ugyldig filsti.");
/// ```
pub(super) const fn get_no_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Det modifiserte innholdet ** vil ikke ** lagres fordi `-SV` ikke ble kalt."##),
        ("new-value", r##"Ny verdi"##),
        ("invalid-path", r##"Ugyldig filsti."##),
    ],
}
}

/// Language ID: no;
/// Map name: "set_md";
/// Description: norsk, latinsk, Norge;
pub(super) const fn get_no_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDet modifiserte innholdet ** vil ikke ** lagres fordi [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m-SV[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ikke ble kalt.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNy verdi[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mUgyldig filsti.[0m"##),
    ],
}
}

/// Language ID: no;
/// Map name: "get";
/// Description: norsk, latinsk, Norge;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Destinasjonsformat");
/// ```
pub(super) const fn get_no_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Destinasjonsformat"##),
        ("src-fmt", r##"Kildefilformat"##),
    ],
}
}

/// no: norsk, latinsk, Norge
pub(super) const fn get_no_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_no_map_conversion),
        ("get", get_no_map_get),
        ("set_md", get_no_map_set_md),
        ("set", get_no_map_set),
        ("conversion_md", get_no_map_conversion_md),
    ],
}
}

/// Language ID: ny;
/// Map name: "conversion";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "...Chonde lingalirani pamanja.");
/// ```
pub(super) const fn get_ny_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("auto-detection-failed", r##"...Chonde lingalirani pamanja."##),
        ("not-included", r##"Binary iyi ** siyikuphatikiza ** kutembenuka koyenera kwa mtundu woyenera.
Muyenera kuti muthandizireni gawo loyenerera mu `chonyamula.toml` ndi recomple.
Ngati pulogalamuyi sinaphatikize magwiridwe antchito omwewo, chonde perekani vuto."##),
    ],
}
}

/// Language ID: ny;
/// Map name: "conversion_md";
pub(super) const fn get_ny_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255m...Chonde lingalirani pamanja.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinary iyi ** siyikuphatikiza ** kutembenuka koyenera kwa mtundu woyenera.
[48;2;34;34;34m[38;2;255;255;255mMuyenera kuti muthandizireni gawo loyenerera mu [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mchonyamula.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ndi recomple.
[48;2;34;34;34m[38;2;255;255;255mNgati pulogalamuyi sinaphatikize magwiridwe antchito omwewo, chonde perekani vuto.[0m"##),
    ],
}
}

pub(super) const fn get_ny_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}
}

pub(super) const fn get_ny_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
    ],
    entries: &[
    ],
}
}

/// Language ID: ny;
/// Map name: "get";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Mtundu Wopita");
/// ```
pub(super) const fn get_ny_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Mtundu Wopita"##),
        ("src-fmt", r##"Gwero la fayilo"##),
    ],
}
}

/// ny: ny-Latn-MW
pub(super) const fn get_ny_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ny_map_conversion),
        ("get", get_ny_map_get),
        ("set_md", get_ny_map_set_md),
        ("set", get_ny_map_set),
        ("conversion_md", get_ny_map_conversion_md),
    ],
}
}

/// Language ID: or;
/// Map name: "conversion";
/// Description: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ଫର\u{b4d}ମ\u{b3e}ଟ\u{b4d} ଚ\u{b3f}ହ\u{b4d}ନଟ କର\u{b3f}ବ\u{b3e}ରେ ବ\u{b3f}ଫଳ ହେଲ\u{b3e} |ଦୟ\u{b3e}କର\u{b3f} ହସ\u{b4d}ତକ\u{b43}ତ ଭ\u{b3e}ବରେ ନ\u{b3f}ର\u{b4d}ଦ\u{b4d}ଦ\u{b3f}ଷ\u{b4d}ଟ କରନ\u{b4d}ତ\u{b41} |");
/// ```
pub(super) const fn get_or_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (2, 1),
        (5, 0),
    ],
    entries: &[
        ("unknown-fmt", r##"ଅଜ୍ଞାତ ଫାଇଲ୍ ଫର୍ମାଟ୍ |"##),
        ("unsupported", r##"ଅସମର୍ଥିତ ଫର୍ମାଟ୍ ରୂପାନ୍ତର |"##),
        ("dst", r##"ଗନ୍ତବ୍ୟସ୍ଥଳ ଫାଇଲ୍ ପଥ |"##),
        ("invalid-json1.0", r##"ଫାଇଲଟି ଏକ ବ valid ଧ `json 1.0` ଫର୍ମାଟ୍ ନୁହେଁ, `JSON5` ଭାବରେ ବିଶ୍ଳେଷଣ କରିବାକୁ ଚେଷ୍ଟା କରୁଛି ..."##),
        ("not-included", r##"ଏହି ବାଇନାରୀ ** ଅନ୍ତର୍ଭୂକ୍ତ କରେ ନାହିଁ ** ପ୍ରଯୁଜ୍ୟ ଫର୍ମାଟ୍ ପାଇଁ ରୂପାନ୍ତର କାର୍ଯ୍ୟକାରିତା |
ତୁମର `କାର୍ଗୋ.ଟୋମୋଲରେ ସମ୍ପୃକ୍ତ ବ feature ଶିଷ୍ଟ୍ୟ ସକ୍ଷମ କରିବାକୁ ପଡିବ ଏବଂ ପୁନ ompmpililed |
ଯଦି ଏହି ସଫ୍ଟୱେର ନିର୍ଦ୍ଦିଷ୍ଟ କାର୍ଯ୍ୟକାରିତା ଅନ୍ତର୍ଭୂକ୍ତ କରେ ନାହିଁ, ଦୟାକରି ଏକ ସମସ୍ୟା ଦାଖଲ କରନ୍ତୁ |"##),
        ("auto-detection-failed", r##"ଫର୍ମାଟ୍ ଚିହ୍ନଟ କରିବାରେ ବିଫଳ ହେଲା |ଦୟାକରି ହସ୍ତକୃତ ଭାବରେ ନିର୍ଦ୍ଦିଷ୍ଟ କରନ୍ତୁ |"##),
        ("conv-error", r##"ରୂପାନ୍ତର ତ୍ରୁଟି |"##),
        ("currently-supported", r##"ସମ୍ପ୍ରଦାୟ ଫର୍ମାଟ୍ ତାଲିକା:"##),
    ],
}
}

/// Language ID: or;
/// Map name: "conversion_md";
/// Description: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ;
pub(super) const fn get_or_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 8694567506910003252,
    disps: &[
        (2, 1),
        (5, 0),
    ],
    entries: &[
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mଅଜ୍ଞାତ ଫାଇଲ୍ ଫର୍ମାଟ୍ [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mଅସମର୍ଥିତ ଫର୍ମାଟ୍ ରୂପାନ୍ତର [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mଗନ୍ତବ୍ୟସ୍ଥଳ ଫାଇଲ୍ ପଥ [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mଫାଇଲଟି ଏକ ବ valid ଧ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ଫର୍ମାଟ୍ ନୁହେଁ, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ଭାବରେ ବିଶ୍ଳେଷଣ କରିବାକୁ ଚେଷ୍ଟା କରୁଛି ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mଏହି ବାଇନାରୀ ** ଅନ୍ତର୍ଭୂକ୍ତ କରେ ନାହିଁ ** ପ୍ରଯୁଜ୍ୟ ଫର୍ମାଟ୍ ପାଇଁ ରୂପାନ୍ତର କାର୍ଯ୍ୟକାରିତା |
[48;2;34;34;34m[38;2;255;255;255mତୁମର [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mକାର୍ଗୋ.ଟୋମୋଲରେ ସମ୍ପୃକ୍ତ ବ feature ଶିଷ୍ଟ୍ୟ ସକ୍ଷମ କରିବାକୁ ପଡିବ ଏବଂ ପୁନ ompmpililed |
[48;2;34;34;34m[38;2;0;255;255mଯଦି ଏହି ସଫ୍ଟୱେର ନିର୍ଦ୍ଦିଷ୍ଟ କାର୍ଯ୍ୟକାରିତା ଅନ୍ତର୍ଭୂକ୍ତ କରେ ନାହିଁ, ଦୟାକରି ଏକ ସମସ୍ୟା ଦାଖଲ କରନ୍ତୁ |[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mଫର୍ମାଟ୍ ଚିହ୍ନଟ କରିବାରେ ବିଫଳ ହେଲା [48;2;34;34;34m[38;2;255;255;255m|[48;2;34;34;34m[38;2;255;255;255mଦୟାକରି ହସ୍ତକୃତ ଭାବରେ ନିର୍ଦ୍ଦିଷ୍ଟ କରନ୍ତୁ [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mରୂପାନ୍ତର ତ୍ରୁଟି [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mସମ୍ପ୍ରଦାୟ ଫର୍ମାଟ୍ ତାଲିକା:[0m"##),
    ],
}
}

/// Language ID: or;
/// Map name: "set";
/// Description: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ଅବ val ଧ ଫ\u{b3e}ଇଲ\u{b4d} ପଥ |");
/// ```
pub(super) const fn get_or_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ପରିବର୍ତ୍ତିତ ବିଷୟବସ୍ତୁ ** ସଞ୍ଚୟ ହୁଏ ନାହିଁ ** ସଞ୍ଚୟ ହେବ ନାହିଁ କାରଣ `--sv` କୁହାଯାଇ ନଥିଲା |"##),
        ("new-value", r##"ନୂତନ ମୂଲ୍ୟ |"##),
        ("invalid-path", r##"ଅବ val ଧ ଫାଇଲ୍ ପଥ |"##),
    ],
}
}

/// Language ID: or;
/// Map name: "set_md";
/// Description: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ;
pub(super) const fn get_or_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mପରିବର୍ତ୍ତିତ ବିଷୟବସ୍ତୁ ** ସଞ୍ଚୟ ହୁଏ ନାହିଁ ** ସଞ୍ଚୟ ହେବ ନାହିଁ କାରଣ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m କୁହାଯାଇ ନଥିଲା [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mନୂତନ ମୂଲ୍ୟ [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mଅବ val ଧ ଫାଇଲ୍ ପଥ [48;2;34;34;34m[38;2;255;255;255m|[0m"##),
    ],
}
}

/// Language ID: or;
/// Map name: "get";
/// Description: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ଗନ\u{b4d}ତବ\u{b4d}ୟସ\u{b4d}ଥଳ ଫର\u{b4d}ମ\u{b3e}ଟ\u{b4d} |");
/// ```
pub(super) const fn get_or_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ଗନ୍ତବ୍ୟସ୍ଥଳ ଫର୍ମାଟ୍ |"##),
        ("src-fmt", r##"ଉତ୍ସ ଫାଇଲ୍ ଫର୍ମାଟ୍ |"##),
    ],
}
}

/// or: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ
pub(super) const fn get_or_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_or_map_conversion),
        ("get", get_or_map_get),
        ("set_md", get_or_map_set_md),
        ("set", get_or_map_set),
        ("conversion_md", get_or_map_conversion_md),
    ],
}
}

/// Language ID: pa;
/// Map name: "conversion";
/// Description: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ਆਟ\u{a4b}ਮ\u{a48}ਟਿਕਲੀ ਫਾਰਮ\u{a48}ਟ ਨ\u{a42}\u{a70} ਆਪਣ\u{a47} ਆਪ ਖ\u{a4b}ਜਣ ਵਿ\u{a71}ਚ ਅਸਫਲ.ਕਿਰਪਾ ਕਰਕ\u{a47} ਹ\u{a71}ਥੀ\u{a02} ਨਿਰਧਾਰਤ ਕਰ\u{a4b}.");
/// ```
pub(super) const fn get_pa_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** ਅਜੇ ਤੱਕ ਸਹਿਯੋਗੀ ਨਹੀਂ ਹੈ **: `ਲਿਸਪ ਐਸ-ਸਮੀਕਰਨ` ਤੋਂ` ਹੋਰ ਫਾਰਮੈਟਾਂ ਵਿੱਚ ਤਬਦੀਲ ਕਰਨਾ"##),
        ("unknown-fmt", r##"ਅਣਜਾਣ ਫਾਇਲ ਫਾਰਮੈਟ"##),
        ("currently-supported", r##"ਮੌਜੂਦਾ ਸਮੇਂ ਸਹਿਯੋਗੀ ਫਾਰਮੈਟ ਸੂਚੀ:"##),
        ("auto-detection-failed", r##"ਆਟੋਮੈਟਿਕਲੀ ਫਾਰਮੈਟ ਨੂੰ ਆਪਣੇ ਆਪ ਖੋਜਣ ਵਿੱਚ ਅਸਫਲ.ਕਿਰਪਾ ਕਰਕੇ ਹੱਥੀਂ ਨਿਰਧਾਰਤ ਕਰੋ."##),
        ("not-included", r##"ਇਸ ਬਾਈਨਰੀ ** ਸ਼ਾਮਲ ਨਹੀਂ ਕਰਦਾ ** ਸੰਬੰਧਿਤ ਫਾਰਮੈਟ ਲਈ ਪਰਿਵਰਤਨ ਕਾਰਜਸ਼ੀਲਤਾ.
ਤੁਹਾਨੂੰ ਆਪਣੇ `ਕਾਰਗੋ.ਟੌਮਲ` ਅਤੇ ਮੁੜ ਕੰਪਾਇਲ ਵਿੱਚ ਸੰਬੰਧਿਤ ਵਿਸ਼ੇਸ਼ਤਾ ਨੂੰ ਸਮਰੱਥ ਕਰਨ ਦੀ ਜ਼ਰੂਰਤ ਹੈ.
ਜੇ ਇਸ ਸਾੱਫਟਵੇਅਰ ਵਿੱਚ ਅਨੁਸਾਰੀ ਕਾਰਜਸ਼ੀਲਤਾ ਸ਼ਾਮਲ ਨਹੀਂ ਹੈ, ਕਿਰਪਾ ਕਰਕੇ ਇੱਕ ਮੁੱਦਾ ਪੇਸ਼ ਕਰੋ."##),
        ("conv-error", r##"ਤਬਦੀਲੀ ਦੀ ਗਲਤੀ"##),
        ("dst", r##"ਟਿਕਾਣਾ ਫਾਈਲ ਮਾਰਗ"##),
        ("unsupported", r##"ਅਸਮਰਥਿਤ ਫਾਰਮੈਟ ਪਰਿਵਰਤਨ"##),
        ("invalid-json1.0", r##"ਫਾਈਲ ਇੱਕ ਵੈਧ `jonn 1.0 ਦਾ ਫਾਰਮੈਟ ਨਹੀਂ ਹੈ, `json5` ਦੇ ਪਾਰਸ ਕਰਨ ਦੀ ਕੋਸ਼ਿਸ਼ ਕਰ ਰਿਹਾ ਹੈ ..."##),
    ],
}
}

/// Language ID: pa;
/// Map name: "conversion_md";
/// Description: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ;
pub(super) const fn get_pa_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ਅਜੇ ਤੱਕ ਸਹਿਯੋਗੀ ਨਹੀਂ ਹੈ [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mਲਿਸਪ ਐਸ-ਸਮੀਕਰਨ[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m ਤੋਂ[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ਹੋਰ ਫਾਰਮੈਟਾਂ ਵਿੱਚ ਤਬਦੀਲ ਕਰਨਾ[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mਅਣਜਾਣ ਫਾਇਲ ਫਾਰਮੈਟ[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mਮੌਜੂਦਾ ਸਮੇਂ ਸਹਿਯੋਗੀ ਫਾਰਮੈਟ ਸੂਚੀ:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mਆਟੋਮੈਟਿਕਲੀ ਫਾਰਮੈਟ ਨੂੰ ਆਪਣੇ ਆਪ ਖੋਜਣ ਵਿੱਚ ਅਸਫਲ.ਕਿਰਪਾ ਕਰਕੇ ਹੱਥੀਂ ਨਿਰਧਾਰਤ ਕਰੋ.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mਇਸ ਬਾਈਨਰੀ ** ਸ਼ਾਮਲ ਨਹੀਂ ਕਰਦਾ ** ਸੰਬੰਧਿਤ ਫਾਰਮੈਟ ਲਈ ਪਰਿਵਰਤਨ ਕਾਰਜਸ਼ੀਲਤਾ.
[48;2;34;34;34m[38;2;255;255;255mਤੁਹਾਨੂੰ ਆਪਣੇ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mਕਾਰਗੋ.ਟੌਮਲ[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ਅਤੇ ਮੁੜ ਕੰਪਾਇਲ ਵਿੱਚ ਸੰਬੰਧਿਤ ਵਿਸ਼ੇਸ਼ਤਾ ਨੂੰ ਸਮਰੱਥ ਕਰਨ ਦੀ ਜ਼ਰੂਰਤ ਹੈ.
[48;2;34;34;34m[38;2;255;255;255mਜੇ ਇਸ ਸਾੱਫਟਵੇਅਰ ਵਿੱਚ ਅਨੁਸਾਰੀ ਕਾਰਜਸ਼ੀਲਤਾ ਸ਼ਾਮਲ ਨਹੀਂ ਹੈ, ਕਿਰਪਾ ਕਰਕੇ ਇੱਕ ਮੁੱਦਾ ਪੇਸ਼ ਕਰੋ.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mਤਬਦੀਲੀ ਦੀ ਗਲਤੀ[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mਟਿਕਾਣਾ ਫਾਈਲ ਮਾਰਗ[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mਅਸਮਰਥਿਤ ਫਾਰਮੈਟ ਪਰਿਵਰਤਨ[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mਫਾਈਲ ਇੱਕ ਵੈਧ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjonn 1.0 ਦਾ ਫਾਰਮੈਟ ਨਹੀਂ ਹੈ, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ਦੇ ਪਾਰਸ ਕਰਨ ਦੀ ਕੋਸ਼ਿਸ਼ ਕਰ ਰਿਹਾ ਹੈ ...[0m"##),
    ],
}
}

/// Language ID: pa;
/// Map name: "set";
/// Description: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ਅਯ\u{a4b}ਗ ਫਾਈਲ ਮਾਰਗ.");
/// ```
pub(super) const fn get_pa_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ਸੋਧੀ ਹੋਈ ਸਮੱਗਰੀ ** ** ਨਹੀਂ ਬਚਾਈ ਜਾਏਗੀ ਕਿਉਂਕਿ `ਸ਼ਵੀ = ਨੂੰ ਨਹੀਂ ਬੁਲਾਇਆ ਜਾਏਗਾ."##),
        ("new-value", r##"ਨਵਾਂ ਮੁੱਲ"##),
        ("invalid-path", r##"ਅਯੋਗ ਫਾਈਲ ਮਾਰਗ."##),
    ],
}
}

/// Language ID: pa;
/// Map name: "set_md";
/// Description: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ;
pub(super) const fn get_pa_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mਸੋਧੀ ਹੋਈ ਸਮੱਗਰੀ ** ** ਨਹੀਂ ਬਚਾਈ ਜਾਏਗੀ ਕਿਉਂਕਿ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mਸ਼ਵੀ = ਨੂੰ ਨਹੀਂ ਬੁਲਾਇਆ ਜਾਏਗਾ.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mਨਵਾਂ ਮੁੱਲ[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mਅਯੋਗ ਫਾਈਲ ਮਾਰਗ.[0m"##),
    ],
}
}

/// Language ID: pa;
/// Map name: "get";
/// Description: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ਮ\u{a70}ਜ\u{a3c}ਿਲ ਦਾ ਫਾਰਮ\u{a48}ਟ");
/// ```
pub(super) const fn get_pa_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ਮੰਜ਼ਿਲ ਦਾ ਫਾਰਮੈਟ"##),
        ("src-fmt", r##"ਸਰੋਤ ਫਾਇਲ ਫਾਰਮੈਟ"##),
    ],
}
}

/// pa: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ
pub(super) const fn get_pa_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_pa_map_conversion),
        ("get", get_pa_map_get),
        ("set_md", get_pa_map_set_md),
        ("set", get_pa_map_set),
        ("conversion_md", get_pa_map_conversion_md),
    ],
}
}

/// Language ID: pl;
/// Map name: "conversion";
/// Description: polski, łacińskie, Polska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Nie udało się automatycznie wykryć formatu. Proszę określić go ręcznie.");
/// ```
pub(super) const fn get_pl_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Ten plik nie jest w prawidłowym formacie `json 1.0`. Próba parsowania jako `json5`..."##),
        ("not-included", r##"To narzędzie **nie zawiera** funkcjonalności konwersji dla danego formatu.
Musisz włączyć odpowiednią funkcję w pliku `Cargo.toml` i ponownie skompilować.
Jeśli to oprogramowanie nie zawiera odpowiedniej funkcjonalności, zgłoś problem."##),
        ("currently-supported", r##"Aktualnie obsługiwane formaty:"##),
        ("unsupported", r##"Konwersja nieobsługiwanego formatu."##),
        ("auto-detection-failed", r##"Nie udało się automatycznie wykryć formatu. Proszę określić go ręcznie."##),
        ("not-support-deser-sexp", r##"**Jeszcze nieobsługiwane**: konwersja z `Lisp S-Expression` na `inne formaty`."##),
        ("unknown-fmt", r##"Nieznany format pliku."##),
        ("not-saved", r##"Następująca zawartość **nie zostanie** zapisana, ponieważ nie została wywołana opcja `--save`."##),
        ("dst", r##"Ścieżka docelowego pliku"##),
        ("conv-error", r##"Błąd konwersji."##),
    ],
}
}

/// Language ID: pl;
/// Map name: "conversion_md";
/// Description: polski, łacińskie, Polska;
pub(super) const fn get_pl_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mTen plik nie jest w prawidłowym formacie [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m. Próba parsowania jako [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mTo narzędzie [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnie zawiera[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m funkcjonalności konwersji dla danego formatu.
[48;2;34;34;34m[38;2;255;255;255mMusisz włączyć odpowiednią funkcję w pliku [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m i ponownie skompilować.
[48;2;34;34;34m[38;2;255;255;255mJeśli to oprogramowanie nie zawiera odpowiedniej funkcjonalności, zgłoś problem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mAktualnie obsługiwane formaty:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mKonwersja nieobsługiwanego formatu.[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNie udało się automatycznie wykryć formatu. Proszę określić go ręcznie.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mJeszcze nieobsługiwane[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: konwersja z [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m na [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255minne formaty[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNieznany format pliku.[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNastępująca zawartość [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnie zostanie[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m zapisana, ponieważ nie została wywołana opcja [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mŚcieżka docelowego pliku[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mBłąd konwersji.[0m"##),
    ],
}
}

/// Language ID: pl;
/// Map name: "set";
/// Description: polski, łacińskie, Polska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Nieprawidłowa ścieżka pliku.");
/// ```
pub(super) const fn get_pl_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Zmodyfikowana zawartość **nie zostanie** zapisana, ponieważ nie została wywołana opcja `--sv`."##),
        ("new-value", r##"Nowa wartość"##),
        ("invalid-path", r##"Nieprawidłowa ścieżka pliku."##),
    ],
}
}

/// Language ID: pl;
/// Map name: "set_md";
/// Description: polski, łacińskie, Polska;
pub(super) const fn get_pl_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mZmodyfikowana zawartość [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnie zostanie[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m zapisana, ponieważ nie została wywołana opcja [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNowa wartość[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNieprawidłowa ścieżka pliku.[0m"##),
    ],
}
}

/// Language ID: pl;
/// Map name: "get";
/// Description: polski, łacińskie, Polska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format docelowy");
/// ```
pub(super) const fn get_pl_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format docelowy"##),
        ("src-fmt", r##"Format pliku źródłowego"##),
    ],
}
}

/// pl: polski, łacińskie, Polska
pub(super) const fn get_pl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_pl_map_conversion),
        ("get", get_pl_map_get),
        ("set_md", get_pl_map_set_md),
        ("set", get_pl_map_set),
        ("conversion_md", get_pl_map_conversion_md),
    ],
}
}

/// Language ID: ps;
/// Map name: "conversion";
/// Description: پښتو, عربي, افغانستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "په اوتومات ډول یې ب format ه کشف کول ونشول.مهرباني وکړئ په لاسي ډول مشخص کړئ.");
/// ```
pub(super) const fn get_ps_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"Whoc نه دی ملاتړ **: د `لیپس ایس ایس ایس ای څرګندول 'او نور بمونه"##),
        ("unknown-fmt", r##"نامعلومه دوتنه"##),
        ("currently-supported", r##"اوس مهال د بیمې ملاتړ شوي فارمیټونو لیست:"##),
        ("auto-detection-failed", r##"په اوتومات ډول یې ب format ه کشف کول ونشول.مهرباني وکړئ په لاسي ډول مشخص کړئ."##),
        ("not-included", r##"دا بائنری ** د اړونده ب format ې لپاره د تبادلې فعالیت کې شامل ندي ** د تبادلې فعالیت.
تاسو اړتیا لرئ په خپل `کارګو ایوټول او بیا تنظیمولو کې اړونده ب feature ه وړ کړئ.
که چیرې دا سافټویر ورته فعالیت نه وي، مهرباني وکړئ یوه مسله وسپارئ."##),
        ("conv-error", r##"د بدلولو تېروتنه"##),
        ("dst", r##"منزل دوتنې لار"##),
        ("unsupported", r##"د ناملاتړی نښه"##),
        ("invalid-json1.0", r##"فایل د اعتبار وړ `JSon 1.0` ب format ه نه ده، هڅه کوي د `json5```````` ... ..."##),
    ],
}
}

/// Language ID: ps;
/// Map name: "conversion_md";
/// Description: پښتو, عربي, افغانستان;
pub(super) const fn get_ps_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255mWhoc نه دی ملاتړ [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: د [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mلیپس ایس ایس ایس ای څرګندول 'او نور بمونه[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mنامعلومه دوتنه[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mاوس مهال د بیمې ملاتړ شوي فارمیټونو لیست:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mپه اوتومات ډول یې ب format ه کشف کول ونشول.مهرباني وکړئ په لاسي ډول مشخص کړئ.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mدا بائنری ** د اړونده ب format ې لپاره د تبادلې فعالیت کې شامل ندي ** د تبادلې فعالیت.
[48;2;34;34;34m[38;2;255;255;255mتاسو اړتیا لرئ په خپل [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mکارګو ایوټول او بیا تنظیمولو کې اړونده ب feature ه وړ کړئ.
[48;2;34;34;34m[38;2;0;255;255mکه چیرې دا سافټویر ورته فعالیت نه وي، مهرباني وکړئ یوه مسله وسپارئ.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mد بدلولو تېروتنه[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mمنزل دوتنې لار[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mد ناملاتړی نښه[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mفایل د اعتبار وړ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSon 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ب format ه نه ده، هڅه کوي د [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5```````` ... ...[0m"##),
    ],
}
}

/// Language ID: ps;
/// Map name: "set";
/// Description: پښتو, عربي, افغانستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "غلطه فایل لاره.");
/// ```
pub(super) const fn get_ps_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"تغیر شوی مینځپانګه ** نه پاکه شوې ځکه چې `- - - - - - - - - - - -"##),
        ("new-value", r##"نوی ارزښت"##),
        ("invalid-path", r##"غلطه فایل لاره."##),
    ],
}
}

/// Language ID: ps;
/// Map name: "set_md";
/// Description: پښتو, عربي, افغانستان;
pub(super) const fn get_ps_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mتغیر شوی مینځپانګه ** نه پاکه شوې ځکه چې [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m- - - - - - - - - - - -[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mنوی ارزښت[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mغلطه فایل لاره.[0m"##),
    ],
}
}

/// Language ID: ps;
/// Map name: "get";
/// Description: پښتو, عربي, افغانستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "د منزل ب .ه");
/// ```
pub(super) const fn get_ps_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"د منزل ب .ه"##),
        ("src-fmt", r##"د سرچینې دوسیه ب .ه"##),
    ],
}
}

/// ps: پښتو, عربي, افغانستان
pub(super) const fn get_ps_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ps_map_conversion),
        ("get", get_ps_map_get),
        ("set_md", get_ps_map_set_md),
        ("set", get_ps_map_set),
        ("conversion_md", get_ps_map_conversion_md),
    ],
}
}

/// Language ID: pt;
/// Map name: "conversion";
/// Description: português, latim, Brasil;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Falha na detecção automática do formato. Por favor especifique manualmente.");
/// ```
pub(super) const fn get_pt_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"O arquivo não está em formato `json 1.0` válido, tentando analisá-lo como `json5`..."##),
        ("not-included", r##"Este binário **não inclui** a funcionalidade de conversão para o formato correspondente.
Você precisa ativar a feature relevante no seu `Cargo.toml` e recompilar.
Se este software não incluir a funcionalidade correspondente, por favor, envie um problema."##),
        ("currently-supported", r##"Lista de formatos atualmente suportados:"##),
        ("unsupported", r##"Conversão de formato não suportada"##),
        ("auto-detection-failed", r##"Falha na detecção automática do formato. Por favor especifique manualmente."##),
        ("not-support-deser-sexp", r##"**Ainda não suportado**: conversão de `Lisp S-Expression` para `outros formatos`"##),
        ("unknown-fmt", r##"Formato de arquivo desconhecido"##),
        ("not-saved", r##"O seguinte conteúdo **não será** salvo porque `--save` não foi chamado."##),
        ("dst", r##"Caminho do arquivo de destino"##),
        ("conv-error", r##"Erro de conversão"##),
    ],
}
}

/// Language ID: pt;
/// Map name: "conversion_md";
/// Description: português, latim, Brasil;
pub(super) const fn get_pt_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mO arquivo não está em formato [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m válido, tentando analisá-lo como [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mEste binário [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnão inclui[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m a funcionalidade de conversão para o formato correspondente.
[48;2;34;34;34m[38;2;255;255;255mVocê precisa ativar a feature relevante no seu [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m e recompilar.
[48;2;34;34;34m[38;2;255;255;255mSe este software não incluir a funcionalidade correspondente, por favor, envie um problema.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLista de formatos atualmente suportados:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mConversão de formato não suportada[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mFalha na detecção automática do formato. Por favor especifique manualmente.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mAinda não suportado[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m: conversão de [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m para [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255moutros formatos[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormato de arquivo desconhecido[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mO seguinte conteúdo [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnão será[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m salvo porque [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m não foi chamado.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mCaminho do arquivo de destino[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mErro de conversão[0m"##),
    ],
}
}

/// Language ID: pt;
/// Map name: "get";
/// Description: português, latim, Brasil;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Formato de destino");
/// ```
pub(super) const fn get_pt_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Formato de destino"##),
        ("src-fmt", r##"Formato do arquivo de origem"##),
    ],
}
}

/// Language ID: pt;
/// Map name: "set";
/// Description: português, latim, Brasil;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Caminho de arquivo inválido.");
/// ```
pub(super) const fn get_pt_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"O conteúdo modificado **não será** salvo porque `--save` não foi chamado."##),
        ("new-value", r##"Novo valor"##),
        ("invalid-path", r##"Caminho de arquivo inválido."##),
    ],
}
}

/// Language ID: pt;
/// Map name: "set_md";
/// Description: português, latim, Brasil;
pub(super) const fn get_pt_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mO conteúdo modificado [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mnão será[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m salvo porque [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m não foi chamado.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNovo valor[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mCaminho de arquivo inválido.[0m"##),
    ],
}
}

/// pt: português, latim, Brasil
pub(super) const fn get_pt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_pt_map_conversion),
        ("get", get_pt_map_get),
        ("set_md", get_pt_map_set_md),
        ("set", get_pt_map_set),
        ("conversion_md", get_pt_map_conversion_md),
    ],
}
}

/// Language ID: ro;
/// Map name: "conversion";
/// Description: română, latină, România;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "nu a reușit să detecteze automat formatul.Vă rugăm să specificați manual.");
/// ```
pub(super) const fn get_ro_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Fișierul nu este un format valabil „JSON 1.0`, încercând să analizeze ca `JSON5` ..."##),
        ("not-included", r##"Acest binar ** nu include ** funcționalitatea de conversie pentru formatul relevant.
Trebuie să activați funcția relevantă în „Cargo.toml” și să vă recompilați.
Dacă acest software nu include funcționalitatea corespunzătoare, vă rugăm să trimiteți o problemă."##),
        ("currently-supported", r##"Lista formatelor acceptate în prezent:"##),
        ("unsupported", r##"conversia formatului neacceptat"##),
        ("auto-detection-failed", r##"nu a reușit să detecteze automat formatul.Vă rugăm să specificați manual."##),
        ("not-support-deser-sexp", r##"** Nu este acceptat încă **: Conversia de la `Lisp S-Expresie` în` Alte formate`"##),
        ("unknown-fmt", r##"Format de fișier necunoscut"##),
        ("not-saved", r##"Următorul conținut nu va fi salvat pentru că „`--save` nu a fost apelat."##),
        ("dst", r##"Calea fișierului de destinație"##),
        ("conv-error", r##"eroare de conversie"##),
    ],
}
}

/// Language ID: ro;
/// Map name: "conversion_md";
/// Description: română, latină, România;
pub(super) const fn get_ro_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFișierul nu este un format valabil „JSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m, încercând să analizeze ca [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mAcest binar ** nu include ** funcționalitatea de conversie pentru formatul relevant.
[48;2;34;34;34m[38;2;255;255;255mTrebuie să activați funcția relevantă în „Cargo.toml” și să vă recompilați.
[48;2;34;34;34m[38;2;255;255;255mDacă acest software nu include funcționalitatea corespunzătoare, vă rugăm să trimiteți o problemă.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLista formatelor acceptate în prezent:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mconversia formatului neacceptat[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mnu a reușit să detecteze automat formatul.Vă rugăm să specificați manual.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Nu este acceptat încă [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Conversia de la [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expresie[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m în[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m Alte formate[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormat de fișier necunoscut[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mUrmătorul conținut nu va fi salvat pentru că „[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nu a fost apelat.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mCalea fișierului de destinație[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255meroare de conversie[0m"##),
    ],
}
}

/// Language ID: ro;
/// Map name: "set";
/// Description: română, latină, România;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "calea de fișier nevalid.");
/// ```
pub(super) const fn get_ro_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Conținutul modificat ** nu va fi salvat pentru că `--sv` nu a fost apelat."##),
        ("new-value", r##"Valoare nouă"##),
        ("invalid-path", r##"calea de fișier nevalid."##),
    ],
}
}

/// Language ID: ro;
/// Map name: "set_md";
/// Description: română, latină, România;
pub(super) const fn get_ro_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mConținutul modificat ** nu va fi salvat pentru că [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nu a fost apelat.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mValoare nouă[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mcalea de fișier nevalid.[0m"##),
    ],
}
}

/// Language ID: ro;
/// Map name: "get";
/// Description: română, latină, România;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format de destinație");
/// ```
pub(super) const fn get_ro_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format de destinație"##),
        ("src-fmt", r##"Format de fișier sursă"##),
    ],
}
}

/// ro: română, latină, România
pub(super) const fn get_ro_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ro_map_conversion),
        ("get", get_ro_map_get),
        ("set_md", get_ro_map_set_md),
        ("set", get_ro_map_set),
        ("conversion_md", get_ro_map_conversion_md),
    ],
}
}

/// Language ID: ru;
/// Map name: "conversion";
/// Description: русский, кириллица, Россия;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Не удалось автоматически обнаружить формат.Пожалуйста, укажите вручную.");
/// ```
pub(super) const fn get_ru_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Файл не является действительным форматом `json 1.0`, пытаясь анализировать как `json5` ..."##),
        ("not-included", r##"Этот двоичный ** не включает ** функциональность преобразования для соответствующего формата.
Вам необходимо включить соответствующую функцию в вашем `Cargo.toml` и перекомпилировать.
Если это программное обеспечение не включает соответствующую функциональность, отправьте проблему."##),
        ("currently-supported", r##"Список форматов поддерживаемых в настоящее время:"##),
        ("unsupported", r##"Неподдерживаемое преобразование формата"##),
        ("auto-detection-failed", r##"Не удалось автоматически обнаружить формат.Пожалуйста, укажите вручную."##),
        ("not-support-deser-sexp", r##"** еще не поддерживается **: преобразование из `lisp s-expression` в другие форматы"##),
        ("unknown-fmt", r##"Неизвестный формат файла"##),
        ("not-saved", r##"Следующий содержимое ** не будет сохранено, потому что `--save` не был вызван."##),
        ("dst", r##"Путь файла назначения"##),
        ("conv-error", r##"ошибка преобразования"##),
    ],
}
}

/// Language ID: ru;
/// Map name: "conversion_md";
/// Description: русский, кириллица, Россия;
pub(super) const fn get_ru_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mФайл не является действительным форматом [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, пытаясь анализировать как [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mЭтот двоичный ** не включает ** функциональность преобразования для соответствующего формата.
[48;2;34;34;34m[38;2;255;255;255mВам необходимо включить соответствующую функцию в вашем [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m и перекомпилировать.
[48;2;34;34;34m[38;2;255;255;255mЕсли это программное обеспечение не включает соответствующую функциональность, отправьте проблему.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mСписок форматов поддерживаемых в настоящее время:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mНеподдерживаемое преобразование формата[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mНе удалось автоматически обнаружить формат.Пожалуйста, укажите вручную.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** еще не поддерживается [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: преобразование из [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m в другие форматы[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mНеизвестный формат файла[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mСледующий содержимое ** не будет сохранено, потому что [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m не был вызван.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mПуть файла назначения[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mошибка преобразования[0m"##),
    ],
}
}

/// Language ID: ru;
/// Map name: "set";
/// Description: русский, кириллица, Россия;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Неверный путь к файлу.");
/// ```
pub(super) const fn get_ru_map_set() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"Неверный путь к файлу."##),
        ("new-value", r##"Новое значение"##),
        ("unsave-warn", r##"Измененное содержимое **не будет** сохранено, потому что не была использована опция `--sv`."##),
        ("type", r##"Тип"##),
    ],
}
}

/// Language ID: ru;
/// Map name: "set_md";
/// Description: русский, кириллица, Россия;
pub(super) const fn get_ru_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mНеверный путь к файлу.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mНовое значение[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mИзмененное содержимое [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mне будет[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m сохранено, потому что не была использована опция [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255mТип[0m"##),
    ],
}
}

/// Language ID: ru;
/// Map name: "get";
/// Description: русский, кириллица, Россия;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Формат назначения");
/// ```
pub(super) const fn get_ru_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Формат назначения"##),
        ("src-fmt", r##"Формат исходного файла"##),
    ],
}
}

/// ru: русский, кириллица, Россия
pub(super) const fn get_ru_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ru_map_conversion),
        ("get", get_ru_map_get),
        ("set_md", get_ru_map_set_md),
        ("set", get_ru_map_set),
        ("conversion_md", get_ru_map_conversion_md),
    ],
}
}

/// Language ID: sd;
/// Map name: "conversion";
/// Description: سنڌي, عربي, پاڪستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "خودڪار طريقي سان ترتيب ڏيڻ ۾ ناڪام.مهرباني ڪري دستي طور تي وضاحت ڪريو.");
/// ```
pub(super) const fn get_sd_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** اڃا تائين سپورٽ ناهي ڪئي وئي **: ليس ايس اظهار "کي" ٻين شڪلن کي ""##),
        ("unknown-fmt", r##"نامعلوم فائل فارميٽ"##),
        ("currently-supported", r##"في الحال سپورٽ ٿيل فارميٽس لسٽ:"##),
        ("auto-detection-failed", r##"خودڪار طريقي سان ترتيب ڏيڻ ۾ ناڪام.مهرباني ڪري دستي طور تي وضاحت ڪريو."##),
        ("not-included", r##"هي بائنري ** شامل نه آهي ** لاڳاپيل شڪل لاء تبديلي جي ڪارڪردگي.
توهان کي توهان جي "ڪارگو تي لاڳاپيل فيچر کي فعال ڪرڻ جي ضرورت آهي ۽ ٻيهر حاصل ڪيو.
جيڪڏهن هن سافٽ ويئر کي ملندڙ ڪارڪردگي شامل نه هجي، مهرباني ڪري هڪ مسئلو جمع ڪرايو."##),
        ("conv-error", r##"تبديلي جي غلطي"##),
        ("dst", r##"منزل فائل جو رستو"##),
        ("unsupported", r##"سهڪار واري فارميٽ جي تبديلي"##),
        ("invalid-json1.0", r##"فائل هڪ صحيح نه آهي `JSON 1.0` فارميٽ، JSON5 "جي ڪوشش ڪندي ..."##),
    ],
}
}

/// Language ID: sd;
/// Map name: "conversion_md";
/// Description: سنڌي, عربي, پاڪستان;
pub(super) const fn get_sd_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** اڃا تائين سپورٽ ناهي ڪئي وئي [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: ليس ايس اظهار "کي" ٻين شڪلن کي "[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mنامعلوم فائل فارميٽ[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mفي الحال سپورٽ ٿيل فارميٽس لسٽ:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mخودڪار طريقي سان ترتيب ڏيڻ ۾ ناڪام.مهرباني ڪري دستي طور تي وضاحت ڪريو.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mهي بائنري ** شامل نه آهي ** لاڳاپيل شڪل لاء تبديلي جي ڪارڪردگي.
[48;2;34;34;34m[38;2;255;255;255mتوهان کي توهان جي "ڪارگو تي لاڳاپيل فيچر کي فعال ڪرڻ جي ضرورت آهي ۽ ٻيهر حاصل ڪيو.
[48;2;34;34;34m[38;2;255;255;255mجيڪڏهن هن سافٽ ويئر کي ملندڙ ڪارڪردگي شامل نه هجي، مهرباني ڪري هڪ مسئلو جمع ڪرايو.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mتبديلي جي غلطي[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mمنزل فائل جو رستو[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mسهڪار واري فارميٽ جي تبديلي[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mفائل هڪ صحيح نه آهي [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m فارميٽ، JSON5 "جي ڪوشش ڪندي ...[0m"##),
    ],
}
}

/// Language ID: sd;
/// Map name: "set";
/// Description: سنڌي, عربي, پاڪستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "غلط فائل جو رستو.");
/// ```
pub(super) const fn get_sd_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"تبديل ٿيل مواد ** ** ** محفوظ نه ٿيندو، ڇاڪاڻ ته - ايس وي "نه سڏيو ويو."##),
        ("new-value", r##"نئين قيمت"##),
        ("invalid-path", r##"غلط فائل جو رستو."##),
    ],
}
}

/// Language ID: sd;
/// Map name: "set_md";
/// Description: سنڌي, عربي, پاڪستان;
pub(super) const fn get_sd_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mتبديل ٿيل مواد ** ** ** محفوظ نه ٿيندو، ڇاڪاڻ ته - ايس وي "نه سڏيو ويو.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mنئين قيمت[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mغلط فائل جو رستو.[0m"##),
    ],
}
}

/// Language ID: sd;
/// Map name: "get";
/// Description: سنڌي, عربي, پاڪستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "منزل جي شڪل");
/// ```
pub(super) const fn get_sd_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"منزل جي شڪل"##),
        ("src-fmt", r##"ماخذ فائل فارميٽ"##),
    ],
}
}

/// sd: سنڌي, عربي, پاڪستان
pub(super) const fn get_sd_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sd_map_conversion),
        ("get", get_sd_map_get),
        ("set_md", get_sd_map_set_md),
        ("set", get_sd_map_set),
        ("conversion_md", get_sd_map_conversion_md),
    ],
}
}

/// Language ID: si;
/// Map name: "conversion";
/// Description: සිංහල, සිංහල, ශ්‍රී ලංකාව;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ආකෘත\u{dd2}ය ස\u{dca}වයංක\u{dca}ර\u{dd3}යව හඳ\u{dd4}න\u{dcf} ගැන\u{dd3}මට අපොහොසත\u{dca} ව\u{dd2}ය.කර\u{dd4}ණ\u{dcf}කර අත\u{dd2}න\u{dca} සඳහන\u{dca} කරන\u{dca}න.");
/// ```
pub(super) const fn get_si_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** තවමත් සහය නොදක්වා නැත **:: lisp s ප්රකාශනය 'වෙතින් පරිවර්තනය කිරීම `වෙනත් ආකෘති'"##),
        ("unknown-fmt", r##"නොදන්නා ගොනු ආකෘතිය"##),
        ("currently-supported", r##"දැනට සහාය දක්වන ආකෘති ලැයිස්තුව:"##),
        ("auto-detection-failed", r##"ආකෘතිය ස්වයංක්රීයව හඳුනා ගැනීමට අපොහොසත් විය.කරුණාකර අතින් සඳහන් කරන්න."##),
        ("not-included", r##"මෙම ද්විමය ** අදාළ ආකෘතිය සඳහා පරිවර්තන ක්රියාකාරිත්වය ඇතුළත් නොවේ.
ඔබගේ `CARGO.OML`L හි අදාළ අංගය ඔබට සක්රීය කර නැවත සකස් කරන්න.
මෙම මෘදුකාංගයට අනුරූප ක්රියාකාරිත්වය ඇතුළත් නොවේ නම්, කරුණාකර ගැටලුවක් ඉදිරිපත් කරන්න."##),
        ("conv-error", r##"පරිවර්තන දෝෂයකි"##),
        ("dst", r##"ගමනාන්ත ගොනු මාර්ගය"##),
        ("unsupported", r##"සහාය නොදක්වන ආකෘතිය පරිවර්තනය"##),
        ("invalid-json1.0", r##"ගොනුව වලංගු `JSON 1.0` ආකෘතියක් නොවන අතර එය json5` ලෙස විග්රහ කිරීමට උත්සාහ කරයි."##),
    ],
}
}

/// Language ID: si;
/// Map name: "conversion_md";
/// Description: සිංහල, සිංහල, ශ්‍රී ලංකාව;
pub(super) const fn get_si_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** තවමත් සහය නොදක්වා නැත [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m:: lisp s ප්රකාශනය 'වෙතින් පරිවර්තනය කිරීම [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mවෙනත් ආකෘති'[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mනොදන්නා ගොනු ආකෘතිය[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mදැනට සහාය දක්වන ආකෘති ලැයිස්තුව:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mආකෘතිය ස්වයංක්රීයව හඳුනා ගැනීමට අපොහොසත් විය.කරුණාකර අතින් සඳහන් කරන්න.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mමෙම ද්විමය ** අදාළ ආකෘතිය සඳහා පරිවර්තන ක්රියාකාරිත්වය ඇතුළත් නොවේ.
[48;2;34;34;34m[38;2;255;255;255mඔබගේ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCARGO.OML[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mL හි අදාළ අංගය ඔබට සක්රීය කර නැවත සකස් කරන්න.
[48;2;34;34;34m[38;2;255;255;255mමෙම මෘදුකාංගයට අනුරූප ක්රියාකාරිත්වය ඇතුළත් නොවේ නම්, කරුණාකර ගැටලුවක් ඉදිරිපත් කරන්න.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mපරිවර්තන දෝෂයකි[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mගමනාන්ත ගොනු මාර්ගය[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mසහාය නොදක්වන ආකෘතිය පරිවර්තනය[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mගොනුව වලංගු [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ආකෘතියක් නොවන අතර එය json5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ලෙස විග්රහ කිරීමට උත්සාහ කරයි.[0m"##),
    ],
}
}

/// Language ID: si;
/// Map name: "set";
/// Description: සිංහල, සිංහල, ශ්‍රී ලංකාව;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "අවලංග\u{dd4} ගොන\u{dd4} ම\u{dcf}ර\u{dca}ගයක\u{dca}.");
/// ```
pub(super) const fn get_si_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"නවීකරණය කරන ලද අන්තර්ගතය *** `--sv` නොකියා" ** ගැළවෙන්නේ නැත."##),
        ("new-value", r##"නව අගය"##),
        ("invalid-path", r##"අවලංගු ගොනු මාර්ගයක්."##),
    ],
}
}

/// Language ID: si;
/// Map name: "set_md";
/// Description: සිංහල, සිංහල, ශ්‍රී ලංකාව;
pub(super) const fn get_si_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mනවීකරණය කරන ලද අන්තර්ගතය *** [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m නොකියා" ** ගැළවෙන්නේ නැත.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mනව අගය[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mඅවලංගු ගොනු මාර්ගයක්.[0m"##),
    ],
}
}

/// Language ID: si;
/// Map name: "get";
/// Description: සිංහල, සිංහල, ශ්‍රී ලංකාව;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ගමන\u{dcf}න\u{dca}ත ආකෘත\u{dd2}ය");
/// ```
pub(super) const fn get_si_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"ගමනාන්ත ආකෘතිය"##),
        ("src-fmt", r##"ප්රභව ගොනු ආකෘතිය"##),
    ],
}
}

/// si: සිංහල, සිංහල, ශ්‍රී ලංකාව
pub(super) const fn get_si_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_si_map_conversion),
        ("get", get_si_map_get),
        ("set_md", get_si_map_set_md),
        ("set", get_si_map_set),
        ("conversion_md", get_si_map_conversion_md),
    ],
}
}

/// Language ID: sk;
/// Map name: "conversion";
/// Description: slovenčina, latinka, Slovensko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Nepodarilo sa automaticky zistiť formát.Zadajte ručne.");
/// ```
pub(super) const fn get_sk_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Súbor nie je platný formát `json 1.0`, ktorý sa snaží analyzovať ako `json5` ..."##),
        ("not-included", r##"Tento binárny ** nezahŕňa ** funkčnosť konverzie pre príslušný formát.
Musíte povoliť relevantnú funkciu vo svojom `CARGO.Toml` a rekompilovať.
Ak tento softvér nezahŕňa zodpovedajúcu funkčnosť, predložte problém."##),
        ("currently-supported", r##"V súčasnosti podporovaný zoznam formátov:"##),
        ("unsupported", r##"Nepodporovaný konverzia formátu"##),
        ("auto-detection-failed", r##"Nepodarilo sa automaticky zistiť formát.Zadajte ručne."##),
        ("not-support-deser-sexp", r##"** ešte nie je podporovaný **: Konverzia z `lisp s-expression` na` ďalšie formáty`"##),
        ("unknown-fmt", r##"Formát neznámeho súboru"##),
        ("not-saved", r##"Nasledujúci obsah ** sa neuloží **, pretože `--save` nebol zavolaný."##),
        ("dst", r##"Cesta cieľového súboru"##),
        ("conv-error", r##"Chyba konverzie"##),
    ],
}
}

/// Language ID: sk;
/// Map name: "conversion_md";
/// Description: slovenčina, latinka, Slovensko;
pub(super) const fn get_sk_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mSúbor nie je platný formát [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, ktorý sa snaží analyzovať ako [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mTento binárny ** nezahŕňa ** funkčnosť konverzie pre príslušný formát.
[48;2;34;34;34m[38;2;255;255;255mMusíte povoliť relevantnú funkciu vo svojom [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCARGO.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m a rekompilovať.
[48;2;34;34;34m[38;2;255;255;255mAk tento softvér nezahŕňa zodpovedajúcu funkčnosť, predložte problém.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mV súčasnosti podporovaný zoznam formátov:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mNepodporovaný konverzia formátu[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNepodarilo sa automaticky zistiť formát.Zadajte ručne.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ešte nie je podporovaný [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Konverzia z [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m na[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ďalšie formáty[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormát neznámeho súboru[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNasledujúci obsah ** sa neuloží [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m, pretože [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m nebol zavolaný.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mCesta cieľového súboru[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mChyba konverzie[0m"##),
    ],
}
}

/// Language ID: sk;
/// Map name: "set";
/// Description: slovenčina, latinka, Slovensko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Neplatná cesta súborov.");
/// ```
pub(super) const fn get_sk_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Upravený obsah ** nebude uložený, pretože `--sv` sa nezavolal."##),
        ("new-value", r##"nová hodnota"##),
        ("invalid-path", r##"Neplatná cesta súborov."##),
    ],
}
}

/// Language ID: sk;
/// Map name: "set_md";
/// Description: slovenčina, latinka, Slovensko;
pub(super) const fn get_sk_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mUpravený obsah ** nebude uložený, pretože [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m sa nezavolal.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnová hodnota[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNeplatná cesta súborov.[0m"##),
    ],
}
}

/// Language ID: sk;
/// Map name: "get";
/// Description: slovenčina, latinka, Slovensko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Cieľový formát");
/// ```
pub(super) const fn get_sk_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Cieľový formát"##),
        ("src-fmt", r##"Zdrojový formát súboru"##),
    ],
}
}

/// sk: slovenčina, latinka, Slovensko
pub(super) const fn get_sk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sk_map_conversion),
        ("get", get_sk_map_get),
        ("set_md", get_sk_map_set_md),
        ("set", get_sk_map_set),
        ("conversion_md", get_sk_map_conversion_md),
    ],
}
}

/// Language ID: sl;
/// Map name: "conversion";
/// Description: slovenščina, latinica, Slovenija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ni uspelo samodejno zaznati oblike.Prosimo, navedite ročno.");
/// ```
pub(super) const fn get_sl_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Datoteka ni veljavna oblika `json 1.0`, ki poskuša razčleniti kot `json5` ..."##),
        ("not-included", r##"Ta binarni ** ne vključuje ** funkcionalnosti pretvorbe za ustrezno obliko.
Ustrezno funkcijo morate omogočiti v svojem `tovoru.Toml` in prekucljati.
Če ta programska oprema ne vključuje ustrezne funkcionalnosti, pošljite težavo."##),
        ("currently-supported", r##"Trenutno podprti seznam formati:"##),
        ("unsupported", r##"Pretvorba nepodprte oblike"##),
        ("auto-detection-failed", r##"ni uspelo samodejno zaznati oblike.Prosimo, navedite ročno."##),
        ("not-support-deser-sexp", r##"** še ni podprta **: pretvorba iz `lisp s-expression" v `drugi formati`"##),
        ("unknown-fmt", r##"Neznana oblika datoteke"##),
        ("not-saved", r##"Naslednja vsebina ** ne bo ** shranjena, ker `--save` ni bilo poklicano."##),
        ("dst", r##"pot ciljne datoteke"##),
        ("conv-error", r##"Napaka pretvorbe"##),
    ],
}
}

/// Language ID: sl;
/// Map name: "conversion_md";
/// Description: slovenščina, latinica, Slovenija;
pub(super) const fn get_sl_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mDatoteka ni veljavna oblika [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, ki poskuša razčleniti kot [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mTa binarni ** ne vključuje ** funkcionalnosti pretvorbe za ustrezno obliko.
[48;2;34;34;34m[38;2;255;255;255mUstrezno funkcijo morate omogočiti v svojem [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mtovoru.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m in prekucljati.
[48;2;34;34;34m[38;2;255;255;255mČe ta programska oprema ne vključuje ustrezne funkcionalnosti, pošljite težavo.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mTrenutno podprti seznam formati:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mPretvorba nepodprte oblike[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mni uspelo samodejno zaznati oblike.Prosimo, navedite ročno.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** še ni podprta [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: pretvorba iz [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression" v [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114mdrugi formati[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNeznana oblika datoteke[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNaslednja vsebina ** ne bo ** shranjena, ker [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ni bilo poklicano.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mpot ciljne datoteke[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mNapaka pretvorbe[0m"##),
    ],
}
}

/// Language ID: sl;
/// Map name: "set";
/// Description: slovenščina, latinica, Slovenija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Neveljavna pot datoteke.");
/// ```
pub(super) const fn get_sl_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Spremenjena vsebina ** ne bo ** shranjena, ker `--sv` ni bil poklican."##),
        ("new-value", r##"nova vrednost"##),
        ("invalid-path", r##"Neveljavna pot datoteke."##),
    ],
}
}

/// Language ID: sl;
/// Map name: "set_md";
/// Description: slovenščina, latinica, Slovenija;
pub(super) const fn get_sl_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mSpremenjena vsebina ** ne bo ** shranjena, ker [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ni bil poklican.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnova vrednost[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNeveljavna pot datoteke.[0m"##),
    ],
}
}

/// Language ID: sl;
/// Map name: "get";
/// Description: slovenščina, latinica, Slovenija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Ciljni format");
/// ```
pub(super) const fn get_sl_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Ciljni format"##),
        ("src-fmt", r##"Oblika izvorne datoteke"##),
    ],
}
}

/// sl: slovenščina, latinica, Slovenija
pub(super) const fn get_sl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sl_map_conversion),
        ("get", get_sl_map_get),
        ("set_md", get_sl_map_set_md),
        ("set", get_sl_map_set),
        ("conversion_md", get_sl_map_conversion_md),
    ],
}
}

/// Language ID: sm;
/// Map name: "conversion";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Ua le mafai ona otometi iloa le foliga.Faʻamolemole faʻamanino le lima.");
/// ```
pub(super) const fn get_sm_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"Suiga sese"##),
        ("not-support-deser-sexp", r##"** E le lagolagoina A **: Liliu mai le `O Lisp S-Exp` of` o isi ituaiga o"##),
        ("dst", r##"Nofoaga faila faila"##),
        ("unsupported", r##"le lagolagoina o le suiga"##),
        ("auto-detection-failed", r##"Ua le mafai ona otometi iloa le foliga.Faʻamolemole faʻamanino le lima."##),
        ("currently-supported", r##"O loʻo lagolagoina nei pepa faʻavae:"##),
        ("not-included", r##"O lenei binary ** e le aofia ai ** o le tagata faʻatau galuega mo le talafeagai lelei.
E manaʻomia ona e faʻatagaina le mea talafeagai i lau `` Cargo.toml `reco
Afai e le aofia ai le polokalama lea e fesoʻotaʻi ma le talafeagai, faʻamolemole aumai se mataupu."##),
        ("unknown-fmt", r##"le le iloa faila faila"##),
    ],
}
}

/// Language ID: sm;
/// Map name: "conversion_md";
pub(super) const fn get_sm_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mSuiga sese[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** E le lagolagoina A [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Liliu mai le [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mO Lisp S-Exp[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m of[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m o isi ituaiga o[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mNofoaga faila faila[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mle lagolagoina o le suiga[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mUa le mafai ona otometi iloa le foliga.Faʻamolemole faʻamanino le lima.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mO loʻo lagolagoina nei pepa faʻavae:[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mO lenei binary ** e le aofia ai ** o le tagata faʻatau galuega mo le talafeagai lelei.
[48;2;34;34;34m[38;2;255;255;255mE manaʻomia ona e faʻatagaina le mea talafeagai i lau [48;2;34;34;34m[38;2;0;255;255m``[48;2;34;34;34m[38;2;0;255;255m Cargo.toml `reco
[48;2;34;34;34m[38;2;0;255;255mAfai e le aofia ai le polokalama lea e fesoʻotaʻi ma le talafeagai, faʻamolemole aumai se mataupu.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mle le iloa faila faila[0m"##),
    ],
}
}

/// Language ID: sm;
/// Map name: "set";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "le aoga faila o le faila.");
/// ```
pub(super) const fn get_sm_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"o le toe faʻaleleia o mea ** o le a leai **"##),
        ("new-value", r##"Taua fou"##),
        ("invalid-path", r##"le aoga faila o le faila."##),
    ],
}
}

/// Language ID: sm;
/// Map name: "set_md";
pub(super) const fn get_sm_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mo le toe faʻaleleia o mea ** o le a leai **[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mTaua fou[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mle aoga faila o le faila.[0m"##),
    ],
}
}

/// Language ID: sm;
/// Map name: "get";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Nofoaga Taunuuga");
/// ```
pub(super) const fn get_sm_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Nofoaga Taunuuga"##),
        ("src-fmt", r##"Punaoa faila faila"##),
    ],
}
}

/// sm: sm-Latn-WS
pub(super) const fn get_sm_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sm_map_conversion),
        ("get", get_sm_map_get),
        ("set_md", get_sm_map_set_md),
        ("set", get_sm_map_set),
        ("conversion_md", get_sm_map_conversion_md),
    ],
}
}

/// Language ID: sn;
/// Map name: "conversion";
/// Description: chiShona, Latn, Zimbabwe;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Yakundikana kuti uzivewo chimiro.Ndokumbira utaure nemaoko.");
/// ```
pub(super) const fn get_sn_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"* isingatsigirwe parizvino **: Kushandurwa kubva ku `lisp S-Expression` kune mamwe mafomati`"##),
        ("unknown-fmt", r##"isingazivikanwe faira fomati"##),
        ("currently-supported", r##"parizvino inotsigirwa fomu rondedzero:"##),
        ("auto-detection-failed", r##"Yakundikana kuti uzivewo chimiro.Ndokumbira utaure nemaoko."##),
        ("not-included", r##"Iyi binary ** haina kusanganisira iyo yekushandura mashandiro emhando yakakodzera.
Iwe unofanirwa kugonesa chinhu chakakodzera mune yako `cargo.Toml` uye kudzoreredza.
Kana software iyi isingasanganisiri zvinowirirana kushanda, ndapota tumira nyaya."##),
        ("conv-error", r##"Kukanganisa Kukanganisa"##),
        ("dst", r##"nzira yekuenda faira"##),
        ("unsupported", r##"Isingatauriki Fomati Kutendeuka"##),
        ("invalid-json1.0", r##"faira haisi yechokwadi `json 1.0` fomati, kuyedza parse se `json5` ..."##),
    ],
}
}

/// Language ID: sn;
/// Map name: "conversion_md";
/// Description: chiShona, Latn, Zimbabwe;
pub(super) const fn get_sn_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;119;119;119m*[48;2;34;34;34m[38;2;255;255;255m [48;2;34;34;34m[38;2;255;255;255misingatsigirwe parizvino [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Kushandurwa kubva ku [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m kune mamwe mafomati[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255misingazivikanwe faira fomati[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mparizvino inotsigirwa fomu rondedzero:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mYakundikana kuti uzivewo chimiro.Ndokumbira utaure nemaoko.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mIyi binary ** haina kusanganisira iyo yekushandura mashandiro emhando yakakodzera.
[48;2;34;34;34m[38;2;255;255;255mIwe unofanirwa kugonesa chinhu chakakodzera mune yako [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mcargo.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m uye kudzoreredza.
[48;2;34;34;34m[38;2;255;255;255mKana software iyi isingasanganisiri zvinowirirana kushanda, ndapota tumira nyaya.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKukanganisa Kukanganisa[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mnzira yekuenda faira[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mIsingatauriki Fomati Kutendeuka[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mfaira haisi yechokwadi [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m fomati, kuyedza parse se [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
    ],
}
}

/// Language ID: sn;
/// Map name: "set";
/// Description: chiShona, Latn, Zimbabwe;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Isingabvumirwe Faira nzira.");
/// ```
pub(super) const fn get_sn_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Zvinyorwa zvakagadziridzwa"##),
        ("new-value", r##"New Kukosha"##),
        ("invalid-path", r##"Isingabvumirwe Faira nzira."##),
    ],
}
}

/// Language ID: sn;
/// Map name: "set_md";
/// Description: chiShona, Latn, Zimbabwe;
pub(super) const fn get_sn_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mZvinyorwa zvakagadziridzwa[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mNew Kukosha[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mIsingabvumirwe Faira nzira.[0m"##),
    ],
}
}

/// Language ID: sn;
/// Map name: "get";
/// Description: chiShona, Latn, Zimbabwe;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Kuenda kuFomments");
/// ```
pub(super) const fn get_sn_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Kuenda kuFomments"##),
        ("src-fmt", r##"Tsime faira fomati"##),
    ],
}
}

/// sn: chiShona, Latn, Zimbabwe
pub(super) const fn get_sn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sn_map_conversion),
        ("get", get_sn_map_get),
        ("set_md", get_sn_map_set_md),
        ("set", get_sn_map_set),
        ("conversion_md", get_sn_map_conversion_md),
    ],
}
}

/// Language ID: so;
/// Map name: "conversion";
/// Description: Soomaali, Laatiin, Soomaaliya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Ku guuldareystay in si otomaatig ah loo ogaado qaabka.Fadlan gacanta ku sheeg.");
/// ```
pub(super) const fn get_so_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"Waxyaabaha soo socda ee soo socda ** Ma badbaadin doono sababtoo ah `--save` weli lama wicin."##),
        ("unknown-fmt", r##"Qaabka feylasha aan la aqoon"##),
        ("currently-supported", r##"Hadda la taageeray liiska qaabab qaabab:"##),
        ("auto-detection-failed", r##"Ku guuldareystay in si otomaatig ah loo ogaado qaabka.Fadlan gacanta ku sheeg."##),
        ("not-included", r##"Binary-gu ** kuma jiraan ** waxqabadka beddelka ee qaabka ay khusayso.
Waxaad u baahan tahay inaad awood u yeelatid muuqaalka ku habboon 'xamuul'
Haddii software-kan uusan ku darin waxqabadka u dhigma, fadlan soo gudbi arin."##),
        ("conv-error", r##"Khaladka is-beddelka"##),
        ("dst", r##"Jidka faylka loo aado"##),
        ("unsupported", r##"Samee loo beddelo qaab aan la taageerin"##),
        ("invalid-json1.0", r##"Faylku ma aha mid ansax ah `JSON 1.0` qaabka, isku dayga inuu yahay sida `JSON5` ..."##),
    ],
}
}

/// Language ID: so;
/// Map name: "conversion_md";
/// Description: Soomaali, Laatiin, Soomaaliya;
pub(super) const fn get_so_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mWaxyaabaha soo socda ee soo socda ** Ma badbaadin doono sababtoo ah [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m weli lama wicin.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mQaabka feylasha aan la aqoon[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mHadda la taageeray liiska qaabab qaabab:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mKu guuldareystay in si otomaatig ah loo ogaado qaabka.Fadlan gacanta ku sheeg.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinary-gu ** kuma jiraan ** waxqabadka beddelka ee qaabka ay khusayso.
[48;2;34;34;34m[38;2;255;255;255mWaxaad u baahan tahay inaad awood u yeelatid muuqaalka ku habboon 'xamuul'
[48;2;34;34;34m[38;2;255;255;255mHaddii software-kan uusan ku darin waxqabadka u dhigma, fadlan soo gudbi arin.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKhaladka is-beddelka[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mJidka faylka loo aado[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mSamee loo beddelo qaab aan la taageerin[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFaylku ma aha mid ansax ah [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m qaabka, isku dayga inuu yahay sida [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
    ],
}
}

/// Language ID: so;
/// Map name: "set";
/// Description: Soomaali, Laatiin, Soomaaliya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Jid aan sax ahayn.");
/// ```
pub(super) const fn get_so_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Waxyaabaha wax laga beddelay **Ma badbaadin** doono maxaa yeelay `--sv` lama wicin."##),
        ("new-value", r##"Qiimo cusub"##),
        ("invalid-path", r##"Jid aan sax ahayn."##),
    ],
}
}

/// Language ID: so;
/// Map name: "set_md";
/// Description: Soomaali, Laatiin, Soomaaliya;
pub(super) const fn get_so_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mWaxyaabaha wax laga beddelay [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114mMa badbaadin[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m doono maxaa yeelay [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m lama wicin.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mQiimo cusub[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mJid aan sax ahayn.[0m"##),
    ],
}
}

/// Language ID: so;
/// Map name: "get";
/// Description: Soomaali, Laatiin, Soomaaliya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Qaabka loo socdo");
/// ```
pub(super) const fn get_so_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Qaabka loo socdo"##),
        ("src-fmt", r##"Qaabka faylka ee isha"##),
    ],
}
}

/// so: Soomaali, Laatiin, Soomaaliya
pub(super) const fn get_so_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_so_map_conversion),
        ("get", get_so_map_get),
        ("set_md", get_so_map_set_md),
        ("set", get_so_map_set),
        ("conversion_md", get_so_map_conversion_md),
    ],
}
}

/// Language ID: sq;
/// Map name: "conversion";
/// Description: shqip, latin, Shqipëri;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Nuk arriti të zbulojë automatikisht formatin.Ju lutemi specifikoni manualisht.");
/// ```
pub(super) const fn get_sq_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Skedari nuk është një format i vlefshëm `JSON 1.0`, duke u përpjekur të analizojë si `JSON5` ..."##),
        ("not-included", r##"Ky binar ** nuk përfshin ** funksionalitetin e konvertimit për formatin përkatës.
Ju duhet të aktivizoni veçorinë përkatëse në `Cargo.Toml` tuaj dhe të rikompiloni.
Nëse ky softuer nuk përfshin funksionalitetin përkatës, ju lutemi paraqisni një çështje."##),
        ("currently-supported", r##"Lista e formateve të mbështetur aktualisht:"##),
        ("unsupported", r##"Konvertimi i formatit të pambështetur"##),
        ("auto-detection-failed", r##"Nuk arriti të zbulojë automatikisht formatin.Ju lutemi specifikoni manualisht."##),
        ("not-support-deser-sexp", r##"** Nuk është mbështetur akoma **: Konvertimi nga `lisp s-shprehje` në` formate të tjera '"##),
        ("unknown-fmt", r##"Formati i panjohur i skedarit"##),
        ("not-saved", r##"përmbajtja e mëposhtme ** nuk do të ruhet sepse `--save` nuk u thirr."##),
        ("dst", r##"Rruga e skedarit të destinacionit"##),
        ("conv-error", r##"Gabim i Konvertimit"##),
    ],
}
}

/// Language ID: sq;
/// Map name: "conversion_md";
/// Description: shqip, latin, Shqipëri;
pub(super) const fn get_sq_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mSkedari nuk është një format i vlefshëm [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, duke u përpjekur të analizojë si [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mKy binar ** nuk përfshin ** funksionalitetin e konvertimit për formatin përkatës.
[48;2;34;34;34m[38;2;255;255;255mJu duhet të aktivizoni veçorinë përkatëse në [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m tuaj dhe të rikompiloni.
[48;2;34;34;34m[38;2;255;255;255mNëse ky softuer nuk përfshin funksionalitetin përkatës, ju lutemi paraqisni një çështje.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLista e formateve të mbështetur aktualisht:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mKonvertimi i formatit të pambështetur[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mNuk arriti të zbulojë automatikisht formatin.Ju lutemi specifikoni manualisht.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Nuk është mbështetur akoma [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Konvertimi nga [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-shprehje[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m në[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m formate të tjera '[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFormati i panjohur i skedarit[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mpërmbajtja e mëposhtme ** nuk do të ruhet sepse [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nuk u thirr.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mRruga e skedarit të destinacionit[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mGabim i Konvertimit[0m"##),
    ],
}
}

/// Language ID: sq;
/// Map name: "set";
/// Description: shqip, latin, Shqipëri;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Rruga e pavlefshme e skedarit.");
/// ```
pub(super) const fn get_sq_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Përmbajtja e modifikuar ** nuk do të ruhet sepse `--sv` nuk u thirr."##),
        ("new-value", r##"Vlera e Re"##),
        ("invalid-path", r##"Rruga e pavlefshme e skedarit."##),
    ],
}
}

/// Language ID: sq;
/// Map name: "set_md";
/// Description: shqip, latin, Shqipëri;
pub(super) const fn get_sq_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mPërmbajtja e modifikuar ** nuk do të ruhet sepse [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m nuk u thirr.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mVlera e Re[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mRruga e pavlefshme e skedarit.[0m"##),
    ],
}
}

/// Language ID: sq;
/// Map name: "get";
/// Description: shqip, latin, Shqipëri;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format i destinacionit");
/// ```
pub(super) const fn get_sq_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format i destinacionit"##),
        ("src-fmt", r##"Format i skedarit burimor"##),
    ],
}
}

/// sq: shqip, latin, Shqipëri
pub(super) const fn get_sq_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sq_map_conversion),
        ("get", get_sq_map_get),
        ("set_md", get_sq_map_set_md),
        ("set", get_sq_map_set),
        ("conversion_md", get_sq_map_conversion_md),
    ],
}
}

/// Language ID: sr;
/// Map name: "conversion";
/// Description: српски, ћирилица, Србија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Није успело аутоматски открити формат.Молимо наведите ручно.");
/// ```
pub(super) const fn get_sr_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Датотека није валидна формата `Јсон 1.0`, покушавајући да анализира као` ЈСОН5` ..."##),
        ("not-included", r##"Овај бинарни ** не укључује ** функционалност конверзије за одговарајући формат.
Морате да омогућите релевантну карактеристику у вашем `Царго.Томл` и поновној композицији.
Ако овај софтвер не укључује одговарајућу функционалност, молимо да поднесете проблем."##),
        ("currently-supported", r##"Тренутно подржана листа формата:"##),
        ("unsupported", r##"Неподржани формат формат"##),
        ("auto-detection-failed", r##"Није успело аутоматски открити формат.Молимо наведите ручно."##),
        ("not-support-deser-sexp", r##"** Још није подржана **: Претварање из `ЛИСП С-експресије` у` остале формате`"##),
        ("unknown-fmt", r##"Непознати формат датотеке"##),
        ("not-saved", r##"Следећи садржај ** неће ** бити сачуван јер `--save` није звао."##),
        ("dst", r##"Стаза одредишта одредишта"##),
        ("conv-error", r##"Грешка у конверзији"##),
    ],
}
}

/// Language ID: sr;
/// Map name: "conversion_md";
/// Description: српски, ћирилица, Србија;
pub(super) const fn get_sr_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mДатотека није валидна формата [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mЈсон 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, покушавајући да анализира као[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ЈСОН5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mОвај бинарни ** не укључује ** функционалност конверзије за одговарајући формат.
[48;2;34;34;34m[38;2;255;255;255mМорате да омогућите релевантну карактеристику у вашем [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mЦарго.Томл[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m и поновној композицији.
[48;2;34;34;34m[38;2;255;255;255mАко овај софтвер не укључује одговарајућу функционалност, молимо да поднесете проблем.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mТренутно подржана листа формата:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mНеподржани формат формат[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mНије успело аутоматски открити формат.Молимо наведите ручно.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Још није подржана [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Претварање из [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mЛИСП С-експресије[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m у[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m остале формате[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mНепознати формат датотеке[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mСледећи садржај ** неће ** бити сачуван јер [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m није звао.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mСтаза одредишта одредишта[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mГрешка у конверзији[0m"##),
    ],
}
}

/// Language ID: sr;
/// Map name: "set";
/// Description: српски, ћирилица, Србија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Неважећи пут датотека.");
/// ```
pub(super) const fn get_sr_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Модификовани садржај ** неће се сачувати јер `-Св` није звао."##),
        ("new-value", r##"Нова вредност"##),
        ("invalid-path", r##"Неважећи пут датотека."##),
    ],
}
}

/// Language ID: sr;
/// Map name: "set_md";
/// Description: српски, ћирилица, Србија;
pub(super) const fn get_sr_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mМодификовани садржај ** неће се сачувати јер [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m-Св[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m није звао.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mНова вредност[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mНеважећи пут датотека.[0m"##),
    ],
}
}

/// Language ID: sr;
/// Map name: "get";
/// Description: српски, ћирилица, Србија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Одредишни формат");
/// ```
pub(super) const fn get_sr_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Одредишни формат"##),
        ("src-fmt", r##"Изворни формат датотеке"##),
    ],
}
}

/// sr: српски, ћирилица, Србија
pub(super) const fn get_sr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sr_map_conversion),
        ("get", get_sr_map_get),
        ("set_md", get_sr_map_set_md),
        ("set", get_sr_map_set),
        ("conversion_md", get_sr_map_conversion_md),
    ],
}
}

/// Language ID: st;
/// Map name: "conversion";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "E ile ea hloleha ho fumana sebopeho.Ka kopo bolela ka boomo.");
/// ```
pub(super) const fn get_st_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Faele ha se sebopeho se sebetsang sa `Json 1."##),
        ("not-included", r##"Binary ena ** ha e kenyeletse ** ts'ebetso ea phetoho bakeng sa sebopeho se amehang.
U hloka ho nolofalletsa karolo e loketseng ho `cego.toml` le hape.
Haeba software ena e sa kenyeletse ts'ebetso e lumellanang, ka kopo fana ka bothata."##),
        ("currently-supported", r##"Hajoale lenane la lifoto tsa formats:"##),
        ("unsupported", r##"Ho fetola sebopeho sa sebopeho sa"##),
        ("auto-detection-failed", r##"E ile ea hloleha ho fumana sebopeho.Ka kopo bolela ka boomo."##),
        ("not-support-deser-sexp", r##"** ha e tšehelitsoe empa ** ha e tšehelitsoe"##),
        ("unknown-fmt", r##"Forama e sa tsejoeng"##),
        ("not-saved", r##"Mathata a latelang ** a ke ke a pholoha hobane `--save` e ne e sa bitsoa."##),
        ("dst", r##"Tsela ea faele ea faele"##),
        ("conv-error", r##"Phoso ea ho soka"##),
    ],
}
}

/// Language ID: st;
/// Map name: "conversion_md";
pub(super) const fn get_st_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFaele ha se sebopeho se sebetsang sa [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJson 1.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinary ena ** ha e kenyeletse ** ts'ebetso ea phetoho bakeng sa sebopeho se amehang.
[48;2;34;34;34m[38;2;255;255;255mU hloka ho nolofalletsa karolo e loketseng ho [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mcego.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m le hape.
[48;2;34;34;34m[38;2;255;255;255mHaeba software ena e sa kenyeletse ts'ebetso e lumellanang, ka kopo fana ka bothata.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mHajoale lenane la lifoto tsa formats:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mHo fetola sebopeho sa sebopeho sa[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mE ile ea hloleha ho fumana sebopeho.Ka kopo bolela ka boomo.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ha e tšehelitsoe empa ** ha e tšehelitsoe[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mForama e sa tsejoeng[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mMathata a latelang ** a ke ke a pholoha hobane [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m e ne e sa bitsoa.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mTsela ea faele ea faele[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mPhoso ea ho soka[0m"##),
    ],
}
}

/// Language ID: st;
/// Map name: "set";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Tsela ea faele e sa sebetseng.");
/// ```
pub(super) const fn get_st_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Likahare tse fetotsoeng ** li ke ke tsa pholosoa hobane `--sv li ne li sa bitsoa."##),
        ("new-value", r##"boleng bo bocha"##),
        ("invalid-path", r##"Tsela ea faele e sa sebetseng."##),
    ],
}
}

/// Language ID: st;
/// Map name: "set_md";
pub(super) const fn get_st_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mLikahare tse fetotsoeng ** li ke ke tsa pholosoa hobane [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv li ne li sa bitsoa.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mboleng bo bocha[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mTsela ea faele e sa sebetseng.[0m"##),
    ],
}
}

/// Language ID: st;
/// Map name: "get";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Sebopeho sa ho ea");
/// ```
pub(super) const fn get_st_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Sebopeho sa ho ea"##),
        ("src-fmt", r##"Sebopeho sa faele sa Mohloli"##),
    ],
}
}

/// st: st-Latn-ZA
pub(super) const fn get_st_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_st_map_conversion),
        ("get", get_st_map_get),
        ("set_md", get_st_map_set_md),
        ("set", get_st_map_set),
        ("conversion_md", get_st_map_conversion_md),
    ],
}
}

/// Language ID: su;
/// Map name: "conversion";
/// Description: Basa Sunda, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Gagal sacara otomatis pikeun ngadeteksi format.Punten tulis sacara manual.");
/// ```
pub(super) const fn get_su_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (4, 5),
        (4, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"** moal dirojong **: Ngarobih tina `Inkputi S-Unprési"##),
        ("unknown-fmt", r##"format file anu teu dipikanyaho"##),
        ("not-included", r##"Binary ** moal kalebet ** Séktipsi konvensional pikeun format anu relevan.
Anjeun kedah ngaktipkeun fitur anu relevan dina `kargo anjeun.Toml` sareng recomplile.
Upami perangkat lunak ieu henteu kalebet fungsionalitas anu pakait, punten ngalebetkeun masalah."##),
        ("conv-error", r##"Kasalahan konvérsi"##),
        ("unsupported", r##"konvérsi format"##),
        ("auto-detection-failed", r##"Gagal sacara otomatis pikeun ngadeteksi format.Punten tulis sacara manual."##),
        ("dst", r##"jalur file tujuan"##),
    ],
}
}

/// Language ID: su;
/// Map name: "conversion_md";
/// Description: Basa Sunda, Latin, Indonesia;
pub(super) const fn get_su_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (4, 5),
        (4, 0),
    ],
    entries: &[
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** moal dirojong [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Ngarobih tina [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mInkputi S-Unprési[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mformat file anu teu dipikanyaho[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinary ** moal kalebet ** Séktipsi konvensional pikeun format anu relevan.
[48;2;34;34;34m[38;2;255;255;255mAnjeun kedah ngaktipkeun fitur anu relevan dina [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mkargo anjeun.Toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m sareng recomplile.
[48;2;34;34;34m[38;2;255;255;255mUpami perangkat lunak ieu henteu kalebet fungsionalitas anu pakait, punten ngalebetkeun masalah.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKasalahan konvérsi[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mkonvérsi format[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mGagal sacara otomatis pikeun ngadeteksi format.Punten tulis sacara manual.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mjalur file tujuan[0m"##),
    ],
}
}

/// Language ID: su;
/// Map name: "set";
/// Description: Basa Sunda, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "jalur file anu salah.");
/// ```
pub(super) const fn get_su_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Eusi anu dirobih ** moal ** bakal disimpen kusabab `--sv` henteu disebut."##),
        ("new-value", r##"nilai anyar"##),
        ("invalid-path", r##"jalur file anu salah."##),
    ],
}
}

/// Language ID: su;
/// Map name: "set_md";
/// Description: Basa Sunda, Latin, Indonesia;
pub(super) const fn get_su_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mEusi anu dirobih ** moal ** bakal disimpen kusabab [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m henteu disebut.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnilai anyar[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mjalur file anu salah.[0m"##),
    ],
}
}

/// Language ID: su;
/// Map name: "get";
/// Description: Basa Sunda, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Format tujuan");
/// ```
pub(super) const fn get_su_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Format tujuan"##),
        ("src-fmt", r##"Format file sumber"##),
    ],
}
}

/// su: Basa Sunda, Latin, Indonesia
pub(super) const fn get_su_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_su_map_conversion),
        ("get", get_su_map_get),
        ("set_md", get_su_map_set_md),
        ("set", get_su_map_set),
        ("conversion_md", get_su_map_conversion_md),
    ],
}
}

/// Language ID: sv;
/// Map name: "conversion";
/// Description: svenska, latinska, Sverige;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Det gick inte automatiskt att upptäcka formatet.Vänligen ange manuellt.");
/// ```
pub(super) const fn get_sv_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Filen är inte ett giltigt `JSON 1.0` -format, försöker analysera som `json5` ..."##),
        ("not-included", r##"Denna binära ** inkluderar inte ** Konverteringsfunktionen för det relevanta formatet.
Du måste aktivera den relevanta funktionen i din `Cargo.toml` och kompilera om.
Om den här programvaran inte inkluderar motsvarande funktionalitet, skicka ett problem."##),
        ("currently-supported", r##"För närvarande stödda formatlista:"##),
        ("unsupported", r##"Osledat formatkonvertering"##),
        ("auto-detection-failed", r##"Det gick inte automatiskt att upptäcka formatet.Vänligen ange manuellt."##),
        ("not-support-deser-sexp", r##"** Stöds ännu inte **: Konvertering från `lisp s-expression` till` andra format`"##),
        ("unknown-fmt", r##"okänt filformat"##),
        ("not-saved", r##"Följande innehåll ** kommer inte ** att sparas eftersom `--save` inte kallades."##),
        ("dst", r##"Destinationsfilväg"##),
        ("conv-error", r##"Konverteringsfel"##),
    ],
}
}

/// Language ID: sv;
/// Map name: "conversion_md";
/// Description: svenska, latinska, Sverige;
pub(super) const fn get_sv_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFilen är inte ett giltigt [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m -format, försöker analysera som [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mDenna binära ** inkluderar inte ** Konverteringsfunktionen för det relevanta formatet.
[48;2;34;34;34m[38;2;255;255;255mDu måste aktivera den relevanta funktionen i din [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m och kompilera om.
[48;2;34;34;34m[38;2;255;255;255mOm den här programvaran inte inkluderar motsvarande funktionalitet, skicka ett problem.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mFör närvarande stödda formatlista:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mOsledat formatkonvertering[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mDet gick inte automatiskt att upptäcka formatet.Vänligen ange manuellt.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Stöds ännu inte [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Konvertering från [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp s-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m till[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m andra format[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mokänt filformat[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mFöljande innehåll ** kommer inte ** att sparas eftersom [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m inte kallades.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mDestinationsfilväg[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonverteringsfel[0m"##),
    ],
}
}

/// Language ID: sv;
/// Map name: "set";
/// Description: svenska, latinska, Sverige;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ogiltig filväg.");
/// ```
pub(super) const fn get_sv_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Det modifierade innehållet ** kommer inte ** att sparas eftersom `--sv` inte kallades."##),
        ("new-value", r##"nytt värde"##),
        ("invalid-path", r##"Ogiltig filväg."##),
    ],
}
}

/// Language ID: sv;
/// Map name: "set_md";
/// Description: svenska, latinska, Sverige;
pub(super) const fn get_sv_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDet modifierade innehållet ** kommer inte ** att sparas eftersom [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m inte kallades.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mnytt värde[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mOgiltig filväg.[0m"##),
    ],
}
}

/// Language ID: sv;
/// Map name: "get";
/// Description: svenska, latinska, Sverige;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Destinationsformat");
/// ```
pub(super) const fn get_sv_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Destinationsformat"##),
        ("src-fmt", r##"Källfilformat"##),
    ],
}
}

/// sv: svenska, latinska, Sverige
pub(super) const fn get_sv_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sv_map_conversion),
        ("get", get_sv_map_get),
        ("set_md", get_sv_map_set_md),
        ("set", get_sv_map_set),
        ("conversion_md", get_sv_map_conversion_md),
    ],
}
}

/// Language ID: sw;
/// Map name: "conversion";
/// Description: Kiswahili, Kilatini, Tanzania;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Imeshindwa kugundua kiotomatiki muundo.Tafadhali taja mwenyewe.");
/// ```
pub(super) const fn get_sw_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"Yaliyomo ** hayataokolewa kwa sababu `--save` haikuitwa."##),
        ("unknown-fmt", r##"Fomati ya faili isiyojulikana"##),
        ("currently-supported", r##"Orodha ya Fomati inayoungwa mkono kwa sasa:"##),
        ("auto-detection-failed", r##"Imeshindwa kugundua kiotomatiki muundo.Tafadhali taja mwenyewe."##),
        ("not-included", r##"Hii binary ** haijumuishi ** utendaji wa ubadilishaji kwa muundo husika.
Unahitaji kuwezesha kipengee kinachofaa katika `Cargo.toml` yako na kurudisha.
Ikiwa programu hii haijumuishi utendaji unaolingana, tafadhali wasilisha suala."##),
        ("conv-error", r##"kosa la ubadilishaji"##),
        ("dst", r##"Njia ya faili ya marudio"##),
        ("unsupported", r##"Ubadilishaji wa muundo ambao haujasaidiwa"##),
        ("invalid-json1.0", r##"Faili sio muundo halali wa `json 1.0`, kujaribu kujaribu kama `json5` ..."##),
    ],
}
}

/// Language ID: sw;
/// Map name: "conversion_md";
/// Description: Kiswahili, Kilatini, Tanzania;
pub(super) const fn get_sw_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mYaliyomo ** hayataokolewa kwa sababu [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m haikuitwa.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mFomati ya faili isiyojulikana[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mOrodha ya Fomati inayoungwa mkono kwa sasa:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mImeshindwa kugundua kiotomatiki muundo.Tafadhali taja mwenyewe.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mHii binary ** haijumuishi ** utendaji wa ubadilishaji kwa muundo husika.
[48;2;34;34;34m[38;2;255;255;255mUnahitaji kuwezesha kipengee kinachofaa katika [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m yako na kurudisha.
[48;2;34;34;34m[38;2;255;255;255mIkiwa programu hii haijumuishi utendaji unaolingana, tafadhali wasilisha suala.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mkosa la ubadilishaji[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mNjia ya faili ya marudio[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mUbadilishaji wa muundo ambao haujasaidiwa[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFaili sio muundo halali wa [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, kujaribu kujaribu kama [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
    ],
}
}

/// Language ID: sw;
/// Map name: "set";
/// Description: Kiswahili, Kilatini, Tanzania;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Njia batili ya faili.");
/// ```
pub(super) const fn get_sw_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Yaliyorekebishwa ** hayataokolewa kwa sababu `--sv` haikuitwa."##),
        ("new-value", r##"Thamani mpya"##),
        ("invalid-path", r##"Njia batili ya faili."##),
    ],
}
}

/// Language ID: sw;
/// Map name: "set_md";
/// Description: Kiswahili, Kilatini, Tanzania;
pub(super) const fn get_sw_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mYaliyorekebishwa ** hayataokolewa kwa sababu [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m haikuitwa.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mThamani mpya[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNjia batili ya faili.[0m"##),
    ],
}
}

/// Language ID: sw;
/// Map name: "get";
/// Description: Kiswahili, Kilatini, Tanzania;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Muundo wa marudio");
/// ```
pub(super) const fn get_sw_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Muundo wa marudio"##),
        ("src-fmt", r##"Muundo wa faili ya chanzo"##),
    ],
}
}

/// sw: Kiswahili, Kilatini, Tanzania
pub(super) const fn get_sw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_sw_map_conversion),
        ("get", get_sw_map_get),
        ("set_md", get_sw_map_set_md),
        ("set", get_sw_map_set),
        ("conversion_md", get_sw_map_conversion_md),
    ],
}
}

/// Language ID: ta;
/// Map name: "conversion";
/// Description: தமிழ், தமிழ், இந்தியா;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "வடிவமைப\u{bcd}பை த\u{bbe}ன\u{bbe}கவே கண\u{bcd}டறியத\u{bcd} தவறிவிட\u{bcd}டது.கைமுறைய\u{bbe}க குறிப\u{bcd}பிடவும\u{bcd}.");
/// ```
pub(super) const fn get_ta_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"கோப்பு சரியான `JSON 1.0` வடிவம் அல்ல, `json5` என அலச முயற்சிக்கிறது ..."##),
        ("not-included", r##"இந்த பைனரி ** ** தொடர்புடைய வடிவமைப்பிற்கான மாற்று செயல்பாடு இல்லை.
உங்கள் `சரக்கு.டோம்ல்` இல் தொடர்புடைய அம்சத்தை நீங்கள் இயக்க வேண்டும்.
இந்த மென்பொருளில் தொடர்புடைய செயல்பாடு இல்லை என்றால், தயவுசெய்து ஒரு சிக்கலைச் சமர்ப்பிக்கவும்."##),
        ("currently-supported", r##"தற்போது ஆதரிக்கப்படும் வடிவங்கள் பட்டியல்:"##),
        ("unsupported", r##"ஆதரிக்கப்படாத வடிவமைப்பு மாற்றம்"##),
        ("auto-detection-failed", r##"வடிவமைப்பை தானாகவே கண்டறியத் தவறிவிட்டது.கைமுறையாக குறிப்பிடவும்."##),
        ("not-support-deser-sexp", r##"** இன்னும் ஆதரிக்கப்படவில்லை **: `லிஸ்ப் எஸ்-எக்ஸ்பிரஷன்` இலிருந்து` பிற வடிவங்களுக்கு` மாற்றுகிறது"##),
        ("unknown-fmt", r##"தெரியாத கோப்பு வடிவம்"##),
        ("not-saved", r##"பின்வரும் உள்ளடக்கம் ** ** சேமிக்கப்படாது, ஏனெனில் `--save` அழைக்கப்படவில்லை."##),
        ("dst", r##"இலக்கு கோப்பு பாதை"##),
        ("conv-error", r##"மாற்று பிழை"##),
    ],
}
}

/// Language ID: ta;
/// Map name: "conversion_md";
/// Description: தமிழ், தமிழ், இந்தியா;
pub(super) const fn get_ta_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mகோப்பு சரியான [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m வடிவம் அல்ல, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m என அலச முயற்சிக்கிறது ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mஇந்த பைனரி ** ** தொடர்புடைய வடிவமைப்பிற்கான மாற்று செயல்பாடு இல்லை.
[48;2;34;34;34m[38;2;255;255;255mஉங்கள் [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mசரக்கு.டோம்ல்[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m இல் தொடர்புடைய அம்சத்தை நீங்கள் இயக்க வேண்டும்.
[48;2;34;34;34m[38;2;255;255;255mஇந்த மென்பொருளில் தொடர்புடைய செயல்பாடு இல்லை என்றால், தயவுசெய்து ஒரு சிக்கலைச் சமர்ப்பிக்கவும்.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mதற்போது ஆதரிக்கப்படும் வடிவங்கள் பட்டியல்:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mஆதரிக்கப்படாத வடிவமைப்பு மாற்றம்[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mவடிவமைப்பை தானாகவே கண்டறியத் தவறிவிட்டது.கைமுறையாக குறிப்பிடவும்.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** இன்னும் ஆதரிக்கப்படவில்லை [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mலிஸ்ப் எஸ்-எக்ஸ்பிரஷன்[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m இலிருந்து[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m பிற வடிவங்களுக்கு[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m மாற்றுகிறது[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mதெரியாத கோப்பு வடிவம்[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mபின்வரும் உள்ளடக்கம் ** ** சேமிக்கப்படாது, ஏனெனில் [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m அழைக்கப்படவில்லை.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mஇலக்கு கோப்பு பாதை[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mமாற்று பிழை[0m"##),
    ],
}
}

/// Language ID: ta;
/// Map name: "set";
/// Description: தமிழ், தமிழ், இந்தியா;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "தவற\u{bbe}ன கோப\u{bcd}பு ப\u{bbe}தை.");
/// ```
pub(super) const fn get_ta_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"மாற்றியமைக்கப்பட்ட உள்ளடக்கம் ** ** சேமிக்கப்படாது, ஏனெனில் `--sv` அழைக்கப்படவில்லை."##),
        ("new-value", r##"புதிய மதிப்பு"##),
        ("invalid-path", r##"தவறான கோப்பு பாதை."##),
    ],
}
}

/// Language ID: ta;
/// Map name: "set_md";
/// Description: தமிழ், தமிழ், இந்தியா;
pub(super) const fn get_ta_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mமாற்றியமைக்கப்பட்ட உள்ளடக்கம் ** ** சேமிக்கப்படாது, ஏனெனில் [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m அழைக்கப்படவில்லை.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mபுதிய மதிப்பு[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mதவறான கோப்பு பாதை.[0m"##),
    ],
}
}

/// Language ID: ta;
/// Map name: "get";
/// Description: தமிழ், தமிழ், இந்தியா;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "இலக\u{bcd}கு வடிவம\u{bcd}");
/// ```
pub(super) const fn get_ta_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"இலக்கு வடிவம்"##),
        ("src-fmt", r##"மூல கோப்பு வடிவம்"##),
    ],
}
}

/// ta: தமிழ், தமிழ், இந்தியா
pub(super) const fn get_ta_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ta_map_conversion),
        ("get", get_ta_map_get),
        ("set_md", get_ta_map_set_md),
        ("set", get_ta_map_set),
        ("conversion_md", get_ta_map_conversion_md),
    ],
}
}

/// Language ID: te;
/// Map name: "conversion";
/// Description: తెలుగు, తెలుగు, భారతదేశం;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ఆకృత\u{c3f}న\u{c3f} స\u{c4d}వయంచ\u{c3e}లకంగ\u{c3e} గుర\u{c4d}త\u{c3f}ంచడంల\u{c4b} వ\u{c3f}ఫలమ\u{c48}ంద\u{c3f}.దయచ\u{c47}స\u{c3f} మ\u{c3e}నవ\u{c40}యంగ\u{c3e} ప\u{c47}ర\u{c4d}క\u{c4a}నండ\u{c3f}.");
/// ```
pub(super) const fn get_te_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 6),
        (6, 0),
    ],
    entries: &[
        ("not-included", r##"ఈ బైనరీ ** ** సంబంధిత ఫార్మాట్ కోసం మార్పిడి కార్యాచరణను కలిగి లేదు.
మీరు మీ `Cargo.toml` లో సంబంధిత లక్షణాన్ని ప్రారంభించాలి మరియు తిరిగి కంపైల్ చేయాలి.
ఈ సాఫ్ట్‌వేర్ సంబంధిత కార్యాచరణను కలిగి ఉండకపోతే, దయచేసి సమస్యను సమర్పించండి."##),
        ("auto-detection-failed", r##"ఆకృతిని స్వయంచాలకంగా గుర్తించడంలో విఫలమైంది.దయచేసి మానవీయంగా పేర్కొనండి."##),
        ("not-support-deser-sexp", r##"** ఇంకా మద్దతు లేదు **: `లిస్ప్ ఎస్-ఎక్స్‌ప్రెషన్` నుండి` ఇతర ఫార్మాట్‌లకు మార్చడం"##),
        ("dst", r##"గమ్యం ఫైల్ మార్గం"##),
        ("unsupported", r##"మద్దతు లేని ఫార్మాట్ మార్పిడి"##),
        ("currently-supported", r##"ప్రస్తుతం మద్దతు ఉన్న ఫార్మాట్ల జాబితా:"##),
        ("not-saved", r##"కింది కంటెంట్ ** ** సేవ్ చేయబడదు ఎందుకంటే `--save` అని పిలవబడలేదు."##),
        ("unknown-fmt", r##"తెలియని ఫైల్ ఫార్మాట్"##),
        ("conv-error", r##"మార్పిడి లోపం"##),
    ],
}
}

/// Language ID: te;
/// Map name: "conversion_md";
/// Description: తెలుగు, తెలుగు, భారతదేశం;
pub(super) const fn get_te_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 6),
        (6, 0),
    ],
    entries: &[
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mఈ బైనరీ ** ** సంబంధిత ఫార్మాట్ కోసం మార్పిడి కార్యాచరణను కలిగి లేదు.
[48;2;34;34;34m[38;2;255;255;255mమీరు మీ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m లో సంబంధిత లక్షణాన్ని ప్రారంభించాలి మరియు తిరిగి కంపైల్ చేయాలి.
[48;2;34;34;34m[38;2;255;255;255mఈ సాఫ్ట్‌వేర్ సంబంధిత కార్యాచరణను కలిగి ఉండకపోతే, దయచేసి సమస్యను సమర్పించండి.[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mఆకృతిని స్వయంచాలకంగా గుర్తించడంలో విఫలమైంది.దయచేసి మానవీయంగా పేర్కొనండి.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ఇంకా మద్దతు లేదు [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mలిస్ప్ ఎస్-ఎక్స్‌ప్రెషన్[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m నుండి[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ఇతర ఫార్మాట్‌లకు మార్చడం[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mగమ్యం ఫైల్ మార్గం[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mమద్దతు లేని ఫార్మాట్ మార్పిడి[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mప్రస్తుతం మద్దతు ఉన్న ఫార్మాట్ల జాబితా:[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mకింది కంటెంట్ ** ** సేవ్ చేయబడదు ఎందుకంటే [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m అని పిలవబడలేదు.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mతెలియని ఫైల్ ఫార్మాట్[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mమార్పిడి లోపం[0m"##),
    ],
}
}

/// Language ID: te;
/// Map name: "set";
/// Description: తెలుగు, తెలుగు, భారతదేశం;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "చ\u{c46}ల\u{c4d}లన\u{c3f} ఫ\u{c48}ల\u{c4d} మ\u{c3e}ర\u{c4d}గం.");
/// ```
pub(super) const fn get_te_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"సవరించిన కంటెంట్ ** ** సేవ్ చేయబడదు ఎందుకంటే `--sv` అని పిలవబడలేదు."##),
        ("new-value", r##"క్రొత్త విలువ"##),
        ("invalid-path", r##"చెల్లని ఫైల్ మార్గం."##),
    ],
}
}

/// Language ID: te;
/// Map name: "set_md";
/// Description: తెలుగు, తెలుగు, భారతదేశం;
pub(super) const fn get_te_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mసవరించిన కంటెంట్ ** ** సేవ్ చేయబడదు ఎందుకంటే [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m అని పిలవబడలేదు.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mక్రొత్త విలువ[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mచెల్లని ఫైల్ మార్గం.[0m"##),
    ],
}
}

/// Language ID: te;
/// Map name: "get";
/// Description: తెలుగు, తెలుగు, భారతదేశం;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "గమ\u{c4d}యం ఆకృత\u{c3f}");
/// ```
pub(super) const fn get_te_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"గమ్యం ఆకృతి"##),
        ("src-fmt", r##"మూల ఫైల్ ఫార్మాట్"##),
    ],
}
}

/// te: తెలుగు, తెలుగు, భారతదేశం
pub(super) const fn get_te_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_te_map_conversion),
        ("get", get_te_map_get),
        ("set_md", get_te_map_set_md),
        ("set", get_te_map_set),
        ("conversion_md", get_te_map_conversion_md),
    ],
}
}

/// Language ID: tg;
/// Map name: "conversion";
/// Description: тоҷикӣ, Кириллӣ, Тоҷикистон;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Набудани формат ба таври худкор муайян карда нашуд.Лутфан дастӣ муайян кунед.");
/// ```
pub(super) const fn get_tg_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
        (0, 0),
    ],
    entries: &[
        ("currently-supported", r##"Рӯйхати форматҳои форматро дар айни замон дастгирӣ мекунад:"##),
        ("unsupported", r##"Табдили формати дастгирӣнашаванда"##),
        ("unknown-fmt", r##"Файзи файли номаълум"##),
        ("not-support-deser-sexp", r##"** Ҳанӯз дастгирӣ намешавад **: Табдилдиҳӣ аз `lisp S-Mounce's 'ба' дигар форматҳо»"##),
        ("conv-error", r##"Хатои конверсия"##),
        ("invalid-json1.0", r##"Файл формати "Json 1.0" нест"##),
        ("auto-detection-failed", r##"Набудани формат ба таври худкор муайян карда нашуд.Лутфан дастӣ муайян кунед."##),
    ],
}
}

/// Language ID: tg;
/// Map name: "conversion_md";
/// Description: тоҷикӣ, Кириллӣ, Тоҷикистон;
pub(super) const fn get_tg_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
        (0, 0),
    ],
    entries: &[
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mРӯйхати форматҳои форматро дар айни замон дастгирӣ мекунад:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mТабдили формати дастгирӣнашаванда[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mФайзи файли номаълум[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Ҳанӯз дастгирӣ намешавад [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Табдилдиҳӣ аз [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mlisp S-Mounce's 'ба' дигар форматҳо»[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mХатои конверсия[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mФайл формати "Json 1.0" нест[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mНабудани формат ба таври худкор муайян карда нашуд.Лутфан дастӣ муайян кунед.[0m"##),
    ],
}
}

/// Language ID: tg;
/// Map name: "set";
/// Description: тоҷикӣ, Кириллӣ, Тоҷикистон;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Танзимоти файлии нодурусти файл.");
/// ```
pub(super) const fn get_tg_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Мӯҳтавои тағирёфта ** нахоҳад шуд ** наҷот дода намешавад, зеро `--sv` номида намешавад."##),
        ("new-value", r##"Арзиши нав"##),
        ("invalid-path", r##"Танзимоти файлии нодурусти файл."##),
    ],
}
}

/// Language ID: tg;
/// Map name: "set_md";
/// Description: тоҷикӣ, Кириллӣ, Тоҷикистон;
pub(super) const fn get_tg_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mМӯҳтавои тағирёфта ** нахоҳад шуд ** наҷот дода намешавад, зеро [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m номида намешавад.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mАрзиши нав[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mТанзимоти файлии нодурусти файл.[0m"##),
    ],
}
}

/// Language ID: tg;
/// Map name: "get";
/// Description: тоҷикӣ, Кириллӣ, Тоҷикистон;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Формати таъинот");
/// ```
pub(super) const fn get_tg_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Формати таъинот"##),
        ("src-fmt", r##"Формати дискҳо манбаъ"##),
    ],
}
}

/// tg: тоҷикӣ, Кириллӣ, Тоҷикистон
pub(super) const fn get_tg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_tg_map_conversion),
        ("get", get_tg_map_get),
        ("set_md", get_tg_map_set_md),
        ("set", get_tg_map_set),
        ("conversion_md", get_tg_map_conversion_md),
    ],
}
}

/// Language ID: th;
/// Map name: "conversion";
/// Description: ไทย, ไทย, ไทย;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ไม\u{e48}สามารถตรวจจ\u{e31}บร\u{e39}ปแบบได\u{e49}โดยอ\u{e31}ตโนม\u{e31}ต\u{e34}โปรดระบ\u{e38}ด\u{e49}วยตนเอง");
/// ```
pub(super) const fn get_th_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"ไฟล์ไม่ได้เป็นรูปแบบ `JSON 1.0` ที่ถูกต้องพยายามแยกวิเคราะห์เป็น `json5` ..."##),
        ("not-included", r##"ไบนารี ** นี้ไม่รวม ** ฟังก์ชั่นการแปลงสำหรับรูปแบบที่เกี่ยวข้อง
คุณต้องเปิดใช้งานคุณสมบัติที่เกี่ยวข้องใน `Cargo.toml` และ repompile
หากซอฟต์แวร์นี้ไม่รวมฟังก์ชั่นที่เกี่ยวข้องโปรดส่งปัญหา"##),
        ("currently-supported", r##"รายการรูปแบบที่รองรับในปัจจุบัน:"##),
        ("unsupported", r##"การแปลงรูปแบบที่ไม่ได้รับการสนับสนุน"##),
        ("auto-detection-failed", r##"ไม่สามารถตรวจจับรูปแบบได้โดยอัตโนมัติโปรดระบุด้วยตนเอง"##),
        ("not-support-deser-sexp", r##"** ยังไม่รองรับ **: การแปลงจาก `Lisp S-Expression` เป็น` รูปแบบอื่น ๆ '"##),
        ("unknown-fmt", r##"รูปแบบไฟล์ที่ไม่รู้จัก"##),
        ("not-saved", r##"เนื้อหาต่อไปนี้ ** จะไม่ถูกบันทึกไว้เพราะ `--save` ไม่ถูกเรียก"##),
        ("dst", r##"เส้นทางไฟล์ปลายทาง"##),
        ("conv-error", r##"ข้อผิดพลาดการแปลง"##),
    ],
}
}

/// Language ID: th;
/// Map name: "conversion_md";
/// Description: ไทย, ไทย, ไทย;
pub(super) const fn get_th_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mไฟล์ไม่ได้เป็นรูปแบบ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ที่ถูกต้องพยายามแยกวิเคราะห์เป็น [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mไบนารี ** นี้ไม่รวม ** ฟังก์ชั่นการแปลงสำหรับรูปแบบที่เกี่ยวข้อง
[48;2;34;34;34m[38;2;255;255;255mคุณต้องเปิดใช้งานคุณสมบัติที่เกี่ยวข้องใน [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m และ repompile
[48;2;34;34;34m[38;2;255;255;255mหากซอฟต์แวร์นี้ไม่รวมฟังก์ชั่นที่เกี่ยวข้องโปรดส่งปัญหา[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mรายการรูปแบบที่รองรับในปัจจุบัน:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mการแปลงรูปแบบที่ไม่ได้รับการสนับสนุน[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mไม่สามารถตรวจจับรูปแบบได้โดยอัตโนมัติโปรดระบุด้วยตนเอง[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ยังไม่รองรับ [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: การแปลงจาก [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m เป็น[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m รูปแบบอื่น ๆ '[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mรูปแบบไฟล์ที่ไม่รู้จัก[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mเนื้อหาต่อไปนี้ ** จะไม่ถูกบันทึกไว้เพราะ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ไม่ถูกเรียก[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mเส้นทางไฟล์ปลายทาง[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mข้อผิดพลาดการแปลง[0m"##),
    ],
}
}

/// Language ID: th;
/// Map name: "set";
/// Description: ไทย, ไทย, ไทย;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "เส\u{e49}นทางไฟล\u{e4c}ท\u{e35}\u{e48}ไม\u{e48}ถ\u{e39}กต\u{e49}อง");
/// ```
pub(super) const fn get_th_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"เนื้อหาที่แก้ไข ** จะไม่ถูกบันทึกเพราะ `--sv` ไม่ถูกเรียก"##),
        ("new-value", r##"ค่าใหม่"##),
        ("invalid-path", r##"เส้นทางไฟล์ที่ไม่ถูกต้อง"##),
    ],
}
}

/// Language ID: th;
/// Map name: "set_md";
/// Description: ไทย, ไทย, ไทย;
pub(super) const fn get_th_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mเนื้อหาที่แก้ไข ** จะไม่ถูกบันทึกเพราะ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ไม่ถูกเรียก[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mค่าใหม่[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mเส้นทางไฟล์ที่ไม่ถูกต้อง[0m"##),
    ],
}
}

/// Language ID: th;
/// Map name: "get";
/// Description: ไทย, ไทย, ไทย;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "ร\u{e39}ปแบบปลายทาง");
/// ```
pub(super) const fn get_th_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"รูปแบบปลายทาง"##),
        ("src-fmt", r##"รูปแบบไฟล์ต้นฉบับ"##),
    ],
}
}

/// th: ไทย, ไทย, ไทย
pub(super) const fn get_th_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_th_map_conversion),
        ("get", get_th_map_get),
        ("set_md", get_th_map_set_md),
        ("set", get_th_map_set),
        ("conversion_md", get_th_map_conversion_md),
    ],
}
}

/// Language ID: tr;
/// Map name: "conversion";
/// Description: Türkçe, Latin, Türkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Biçimi otomatik olarak algılayamadı.Lütfen manuel olarak belirtin.");
/// ```
pub(super) const fn get_tr_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Dosya geçerli bir `JSON 1.0` biçimi değil, `json5 'olarak ayrıştırmaya çalışan ..."##),
        ("not-included", r##"Bu ikili ilgili format için dönüşüm işlevini içermez.
`Cargo.toml` ve yeniden derlemenizdeki ilgili özelliği etkinleştirmeniz gerekir.
Bu yazılım ilgili işlevleri içermiyorsa, lütfen bir sorun gönderin."##),
        ("currently-supported", r##"Şu anda Desteklenen Formatlar Listesi:"##),
        ("unsupported", r##"Desteklenmemiş Biçim Dönüşümü"##),
        ("auto-detection-failed", r##"Biçimi otomatik olarak algılayamadı.Lütfen manuel olarak belirtin."##),
        ("not-support-deser-sexp", r##"** Henüz desteklenmedi **: Lisp S-ekspresyonundan `` Diğer Biçimler '' e dönüştürülme"##),
        ("unknown-fmt", r##"Bilinmeyen dosya biçimi"##),
        ("not-saved", r##"Aşağıdaki içerik ** `--save` çağrılmadığı için ** kaydedilmez."##),
        ("dst", r##"Hedef Dosya Yolu"##),
        ("conv-error", r##"dönüşüm hatası"##),
    ],
}
}

/// Language ID: tr;
/// Map name: "conversion_md";
/// Description: Türkçe, Latin, Türkiye;
pub(super) const fn get_tr_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mDosya geçerli bir [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m biçimi değil, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5 'olarak ayrıştırmaya çalışan ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBu ikili ilgili format için dönüşüm işlevini içermez.
[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ve yeniden derlemenizdeki ilgili özelliği etkinleştirmeniz gerekir.
[48;2;34;34;34m[38;2;255;255;255mBu yazılım ilgili işlevleri içermiyorsa, lütfen bir sorun gönderin.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mŞu anda Desteklenen Formatlar Listesi:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mDesteklenmemiş Biçim Dönüşümü[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mBiçimi otomatik olarak algılayamadı.Lütfen manuel olarak belirtin.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Henüz desteklenmedi [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Lisp S-ekspresyonundan [48;2;34;34;34m[38;2;0;255;255m``[48;2;34;34;34m[38;2;0;255;255m Diğer Biçimler '' e dönüştürülme[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mBilinmeyen dosya biçimi[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mAşağıdaki içerik ** [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m çağrılmadığı için ** kaydedilmez.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mHedef Dosya Yolu[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mdönüşüm hatası[0m"##),
    ],
}
}

/// Language ID: tr;
/// Map name: "set";
/// Description: Türkçe, Latin, Türkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Geçersiz dosya yolu.");
/// ```
pub(super) const fn get_tr_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Değiştirilmiş içerik **, `---sv` çağrılmadığından ** kaydedilmez."##),
        ("new-value", r##"yeni değer"##),
        ("invalid-path", r##"Geçersiz dosya yolu."##),
    ],
}
}

/// Language ID: tr;
/// Map name: "set_md";
/// Description: Türkçe, Latin, Türkiye;
pub(super) const fn get_tr_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mDeğiştirilmiş içerik [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m, [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m---sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m çağrılmadığından ** kaydedilmez.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255myeni değer[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mGeçersiz dosya yolu.[0m"##),
    ],
}
}

/// Language ID: tr;
/// Map name: "get";
/// Description: Türkçe, Latin, Türkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Hedef biçimi");
/// ```
pub(super) const fn get_tr_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Hedef biçimi"##),
        ("src-fmt", r##"Kaynak Dosya Biçimi"##),
    ],
}
}

/// tr: Türkçe, Latin, Türkiye
pub(super) const fn get_tr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_tr_map_conversion),
        ("get", get_tr_map_get),
        ("set_md", get_tr_map_set_md),
        ("set", get_tr_map_set),
        ("conversion_md", get_tr_map_conversion_md),
    ],
}
}

/// Language ID: ug;
/// Map name: "conversion";
/// Description: ئۇيغۇرچە, ئەرەب, جۇڭگو;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "فورماتىنى ئاپتوماتىك بايقاش مەغلۇب بولدى.قولدا بەلگىلەڭ.");
/// ```
pub(super) const fn get_ug_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (5, 6),
    ],
    entries: &[
        ("invalid-json1.0", r##"ھۆججەت ئىناۋەتلىك ` `JSON 1.0 فورمات ئەمەس, «JSONEN5`» دەپ تەرىۋالماقچى."##),
        ("not-saved", r##"تۆۋەندىكى مەزمۇن *** نى **** نى ساقلىمايدۇ ** ساقلانمايدۇ."##),
        ("dst", r##"نىشان ھۆججەت يولى"##),
        ("not-included", r##"بۇ ئىككىلىك ** ** سۆزنى ئۆز ئىچىگە ئالمايدۇ ** مۇناسىۋەتلىك فورماتنىڭ ئايلانما ئىقتىدارىنى ئۆز ئىچىگە ئالمايدۇ.
«يۈك ساندۇقىڭىز بىلەن مۇناسىۋەتلىك ئىقتىدارنى قوزغىتىشىڭىز كېرەك. ۋە قايتا يېزىلغان.
ئەگەر بۇ يۇمشاق دېتال مۇناسىپ ئىقتىدارنى ئۆز ئىچىگە ئالمىسا, بىر مەسىلىنى يوللاڭ."##),
        ("unknown-fmt", r##"نامەلۇم ھۆججەت شەكلى"##),
        ("unsupported", r##"قوللىمايدىغان فورمات ئۆزگەرتىش"##),
        ("conv-error", r##"ئايلاندۇرۇش خاتالىقى"##),
        ("auto-detection-failed", r##"فورماتىنى ئاپتوماتىك بايقاش مەغلۇب بولدى.قولدا بەلگىلەڭ."##),
    ],
}
}

/// Language ID: ug;
/// Map name: "conversion_md";
/// Description: ئۇيغۇرچە, ئەرەب, جۇڭگو;
pub(super) const fn get_ug_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (5, 6),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mھۆججەت ئىناۋەتلىك [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mJSON 1.0 فورمات ئەمەس, «JSONEN5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m» دەپ تەرىۋالماقچى.[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mتۆۋەندىكى مەزمۇن *** نى **** نى ساقلىمايدۇ ** ساقلانمايدۇ.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mنىشان ھۆججەت يولى[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mبۇ ئىككىلىك ** ** سۆزنى ئۆز ئىچىگە ئالمايدۇ ** مۇناسىۋەتلىك فورماتنىڭ ئايلانما ئىقتىدارىنى ئۆز ئىچىگە ئالمايدۇ.
[48;2;34;34;34m[38;2;255;255;255m«يۈك ساندۇقىڭىز بىلەن مۇناسىۋەتلىك ئىقتىدارنى قوزغىتىشىڭىز كېرەك. ۋە قايتا يېزىلغان.
[48;2;34;34;34m[38;2;255;255;255mئەگەر بۇ يۇمشاق دېتال مۇناسىپ ئىقتىدارنى ئۆز ئىچىگە ئالمىسا, بىر مەسىلىنى يوللاڭ.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mنامەلۇم ھۆججەت شەكلى[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mقوللىمايدىغان فورمات ئۆزگەرتىش[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mئايلاندۇرۇش خاتالىقى[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mفورماتىنى ئاپتوماتىك بايقاش مەغلۇب بولدى.قولدا بەلگىلەڭ.[0m"##),
    ],
}
}

/// Language ID: ug;
/// Map name: "set";
/// Description: ئۇيغۇرچە, ئەرەب, جۇڭگو;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "ئىناۋەتسىز ھۆججەت يولى.");
/// ```
pub(super) const fn get_ug_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ئۆزگەرتىلگەن مەزمۇن **** ساقلانمايدۇ ***** ساقلانمايدۇ."##),
        ("new-value", r##"يېڭى قىممەت"##),
        ("invalid-path", r##"ئىناۋەتسىز ھۆججەت يولى."##),
    ],
}
}

/// Language ID: ug;
/// Map name: "set_md";
/// Description: ئۇيغۇرچە, ئەرەب, جۇڭگو;
pub(super) const fn get_ug_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mئۆزگەرتىلگەن مەزمۇن **** ساقلانمايدۇ ***** ساقلانمايدۇ.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mيېڭى قىممەت[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mئىناۋەتسىز ھۆججەت يولى.[0m"##),
    ],
}
}

/// Language ID: ug;
/// Map name: "get";
/// Description: ئۇيغۇرچە, ئەرەب, جۇڭگو;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "مەنزىل شەكلى");
/// ```
pub(super) const fn get_ug_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"مەنزىل شەكلى"##),
        ("src-fmt", r##"مەنبە ھۆججەت شەكلى"##),
    ],
}
}

/// ug: ئۇيغۇرچە, ئەرەب, جۇڭگو
pub(super) const fn get_ug_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ug_map_conversion),
        ("get", get_ug_map_get),
        ("set_md", get_ug_map_set_md),
        ("set", get_ug_map_set),
        ("conversion_md", get_ug_map_conversion_md),
    ],
}
}

/// Language ID: uk;
/// Map name: "conversion";
/// Description: українська, кирилиця, Україна;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Не вдалося автоматично виявити формат.Будь ласка, вкажіть вручну.");
/// ```
pub(super) const fn get_uk_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Файл не є дійсним форматом json 1.0`, намагаючись розібратися як `json5` ..."##),
        ("not-included", r##"Цей двійковий ** не включає ** функціональність перетворення для відповідного формату.
Вам потрібно ввімкнути відповідну функцію у вашому `Cargo.toml` та перекомпілювати.
Якщо це програмне забезпечення не включає відповідну функціональність, надішліть проблему."##),
        ("currently-supported", r##"Список форматів, що підтримуються в даний час:"##),
        ("unsupported", r##"Небудова перетворення формату"##),
        ("auto-detection-failed", r##"Не вдалося автоматично виявити формат.Будь ласка, вкажіть вручну."##),
        ("not-support-deser-sexp", r##"4"##),
        ("unknown-fmt", r##"Невідомий формат файлу"##),
        ("not-saved", r##"Наступний вміст ** не буде збережено, оскільки `--save` не викликали."##),
        ("dst", r##"шлях призначення"##),
        ("conv-error", r##"Помилка перетворення"##),
    ],
}
}

/// Language ID: uk;
/// Map name: "conversion_md";
/// Description: українська, кирилиця, Україна;
pub(super) const fn get_uk_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mФайл не є дійсним форматом json 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m, намагаючись розібратися як [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mЦей двійковий ** не включає ** функціональність перетворення для відповідного формату.
[48;2;34;34;34m[38;2;255;255;255mВам потрібно ввімкнути відповідну функцію у вашому [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m та перекомпілювати.
[48;2;34;34;34m[38;2;255;255;255mЯкщо це програмне забезпечення не включає відповідну функціональність, надішліть проблему.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mСписок форматів, що підтримуються в даний час:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mНебудова перетворення формату[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mНе вдалося автоматично виявити формат.Будь ласка, вкажіть вручну.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m4[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mНевідомий формат файлу[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mНаступний вміст ** не буде збережено, оскільки [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m не викликали.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mшлях призначення[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mПомилка перетворення[0m"##),
    ],
}
}

/// Language ID: uk;
/// Map name: "set";
/// Description: українська, кирилиця, Україна;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Недійсний шлях файлу.");
/// ```
pub(super) const fn get_uk_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Змінений вміст ** не буде збережено, оскільки `--sv` не викликали."##),
        ("new-value", r##"нове значення"##),
        ("invalid-path", r##"Недійсний шлях файлу."##),
    ],
}
}

/// Language ID: uk;
/// Map name: "set_md";
/// Description: українська, кирилиця, Україна;
pub(super) const fn get_uk_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mЗмінений вміст ** не буде збережено, оскільки [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m не викликали.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mнове значення[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mНедійсний шлях файлу.[0m"##),
    ],
}
}

/// Language ID: uk;
/// Map name: "get";
/// Description: українська, кирилиця, Україна;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Формат призначення");
/// ```
pub(super) const fn get_uk_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Формат призначення"##),
        ("src-fmt", r##"Формат вихідного файлу"##),
    ],
}
}

/// uk: українська, кирилиця, Україна
pub(super) const fn get_uk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_uk_map_conversion),
        ("get", get_uk_map_get),
        ("set_md", get_uk_map_set_md),
        ("set", get_uk_map_set),
        ("conversion_md", get_uk_map_conversion_md),
    ],
}
}

/// Language ID: ur;
/// Map name: "conversion";
/// Description: اردو, عربی, پاکستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "فارمیٹ کا خود بخود پتہ لگانے میں ناکام۔براہ کرم دستی طور پر وضاحت کریں۔");
/// ```
pub(super) const fn get_ur_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"فائل ایک درست `JSON 1.0` فارمیٹ نہیں ہے ، جو `JSON5` کے بطور تجزیہ کرنے کی کوشش کر رہی ہے ..."##),
        ("not-included", r##"اس بائنری ** میں ** متعلقہ فارمیٹ کے لئے تبادلوں کی فعالیت شامل نہیں ہے۔
آپ کو اپنے `Cargo.toml` اور recompile میں متعلقہ خصوصیت کو قابل بنانے کی ضرورت ہے۔
اگر اس سافٹ ویئر میں متعلقہ فعالیت شامل نہیں ہے تو ، براہ کرم کوئی مسئلہ پیش کریں۔"##),
        ("currently-supported", r##"فی الحال معاون فارمیٹس کی فہرست:"##),
        ("unsupported", r##"غیر تعاون یافتہ فارمیٹ تبادلوں"##),
        ("auto-detection-failed", r##"فارمیٹ کا خود بخود پتہ لگانے میں ناکام۔براہ کرم دستی طور پر وضاحت کریں۔"##),
        ("not-support-deser-sexp", r##"** ابھی تک تعاون نہیں کیا گیا **: `LISP S-expression` سے` دوسرے فارمیٹس میں تبدیل کرنا"##),
        ("unknown-fmt", r##"نامعلوم فائل فارمیٹ"##),
        ("not-saved", r##"مندرجہ ذیل مواد ** ** کو محفوظ نہیں کیا جائے گا کیونکہ `--save` کو نہیں بلایا گیا تھا۔"##),
        ("dst", r##"منزل مقصود کا راستہ"##),
        ("conv-error", r##"تبادلوں کی خرابی"##),
    ],
}
}

/// Language ID: ur;
/// Map name: "conversion_md";
/// Description: اردو, عربی, پاکستان;
pub(super) const fn get_ur_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mفائل ایک درست [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m فارمیٹ نہیں ہے ، جو [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m کے بطور تجزیہ کرنے کی کوشش کر رہی ہے ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mاس بائنری ** میں ** متعلقہ فارمیٹ کے لئے تبادلوں کی فعالیت شامل نہیں ہے۔
[48;2;34;34;34m[38;2;255;255;255mآپ کو اپنے [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m اور recompile میں متعلقہ خصوصیت کو قابل بنانے کی ضرورت ہے۔
[48;2;34;34;34m[38;2;255;255;255mاگر اس سافٹ ویئر میں متعلقہ فعالیت شامل نہیں ہے تو ، براہ کرم کوئی مسئلہ پیش کریں۔[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mفی الحال معاون فارمیٹس کی فہرست:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mغیر تعاون یافتہ فارمیٹ تبادلوں[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mفارمیٹ کا خود بخود پتہ لگانے میں ناکام۔براہ کرم دستی طور پر وضاحت کریں۔[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ابھی تک تعاون نہیں کیا گیا [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLISP S-expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m سے[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m دوسرے فارمیٹس میں تبدیل کرنا[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mنامعلوم فائل فارمیٹ[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mمندرجہ ذیل مواد ** ** کو محفوظ نہیں کیا جائے گا کیونکہ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m کو نہیں بلایا گیا تھا۔[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mمنزل مقصود کا راستہ[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mتبادلوں کی خرابی[0m"##),
    ],
}
}

/// Language ID: ur;
/// Map name: "set";
/// Description: اردو, عربی, پاکستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "غلط فائل کا راستہ۔");
/// ```
pub(super) const fn get_ur_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"ترمیم شدہ مواد ** ** کو محفوظ نہیں کیا جائے گا کیونکہ `-SV` کو نہیں بلایا گیا تھا۔"##),
        ("new-value", r##"نئی قیمت"##),
        ("invalid-path", r##"غلط فائل کا راستہ۔"##),
    ],
}
}

/// Language ID: ur;
/// Map name: "set_md";
/// Description: اردو, عربی, پاکستان;
pub(super) const fn get_ur_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mترمیم شدہ مواد ** ** کو محفوظ نہیں کیا جائے گا کیونکہ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m-SV[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m کو نہیں بلایا گیا تھا۔[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mنئی قیمت[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mغلط فائل کا راستہ۔[0m"##),
    ],
}
}

/// Language ID: ur;
/// Map name: "get";
/// Description: اردو, عربی, پاکستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "منزل مقصود");
/// ```
pub(super) const fn get_ur_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"منزل مقصود"##),
        ("src-fmt", r##"ماخذ فائل کی شکل"##),
    ],
}
}

/// ur: اردو, عربی, پاکستان
pub(super) const fn get_ur_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_ur_map_conversion),
        ("get", get_ur_map_get),
        ("set_md", get_ur_map_set_md),
        ("set", get_ur_map_set),
        ("conversion_md", get_ur_map_conversion_md),
    ],
}
}

/// Language ID: uz;
/// Map name: "conversion";
/// Description: o‘zbek, lotin, Oʻzbekiston;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Formatni avtomatik ravishda aniqlab bo'lmadi.Iltimos, qo'lda belgilang.");
/// ```
pub(super) const fn get_uz_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"Konversiya xato"##),
        ("not-support-deser-sexp", r##"Qo'llab-quvvatlanmaydi **: "lisp s-ifoda" dan "boshqa forss" gacha o'zgarishi"##),
        ("dst", r##"Belgilangan fayl yo'li"##),
        ("unsupported", r##"Qo'llab-quvvatlanmaydigan formatni o'zgartirish"##),
        ("auto-detection-failed", r##"Formatni avtomatik ravishda aniqlab bo'lmadi.Iltimos, qo'lda belgilang."##),
        ("currently-supported", r##"Xursand qilingan formatlar ro'yxati:"##),
        ("not-included", r##"Ushbu ikkilik ** tegishli format uchun konversiya funktsiyalarini o'z ichiga olmaydi.
Siz "yuk tashish" va qayta ishlashda tegishli xususiyatni yoqishingiz kerak.
Agar ushbu dastur mos funktsiyani o'z ichiga olmasa, iltimos, muammoni taqdim eting."##),
        ("unknown-fmt", r##"Noma'lum fayl formati"##),
    ],
}
}

/// Language ID: uz;
/// Map name: "conversion_md";
/// Description: o‘zbek, lotin, Oʻzbekiston;
pub(super) const fn get_uz_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 3),
        (2, 0),
    ],
    entries: &[
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mKonversiya xato[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255mQo'llab-quvvatlanmaydi [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: "lisp s-ifoda" dan "boshqa forss" gacha o'zgarishi[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mBelgilangan fayl yo'li[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mQo'llab-quvvatlanmaydigan formatni o'zgartirish[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mFormatni avtomatik ravishda aniqlab bo'lmadi.Iltimos, qo'lda belgilang.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mXursand qilingan formatlar ro'yxati:[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mUshbu ikkilik ** tegishli format uchun konversiya funktsiyalarini o'z ichiga olmaydi.
[48;2;34;34;34m[38;2;255;255;255mSiz "yuk tashish" va qayta ishlashda tegishli xususiyatni yoqishingiz kerak.
[48;2;34;34;34m[38;2;255;255;255mAgar ushbu dastur mos funktsiyani o'z ichiga olmasa, iltimos, muammoni taqdim eting.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mNoma'lum fayl formati[0m"##),
    ],
}
}

/// Language ID: uz;
/// Map name: "set";
/// Description: o‘zbek, lotin, Oʻzbekiston;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Noto'g'ri fayl yo'li.");
/// ```
pub(super) const fn get_uz_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"O'zgartirilgan tarkibni saqlab qolmaydi ** "- -sv" deb nomlanmagan."##),
        ("new-value", r##"Yangilik"##),
        ("invalid-path", r##"Noto'g'ri fayl yo'li."##),
    ],
}
}

/// Language ID: uz;
/// Map name: "set_md";
/// Description: o‘zbek, lotin, Oʻzbekiston;
pub(super) const fn get_uz_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mO'zgartirilgan tarkibni saqlab qolmaydi ** "- -sv" deb nomlanmagan.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mYangilik[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mNoto'g'ri fayl yo'li.[0m"##),
    ],
}
}

/// Language ID: uz;
/// Map name: "get";
/// Description: o‘zbek, lotin, Oʻzbekiston;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Belgilangan format");
/// ```
pub(super) const fn get_uz_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Belgilangan format"##),
        ("src-fmt", r##"Manba fayl formati"##),
    ],
}
}

/// uz: o‘zbek, lotin, Oʻzbekiston
pub(super) const fn get_uz_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_uz_map_conversion),
        ("get", get_uz_map_get),
        ("set_md", get_uz_map_set_md),
        ("set", get_uz_map_set),
        ("conversion_md", get_uz_map_conversion_md),
    ],
}
}

/// Language ID: vi;
/// Map name: "conversion";
/// Description: Tiếng Việt, Chữ La tinh, Việt Nam;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Không thể tự động phát hiện định dạng.Vui lòng chỉ định thủ công.");
/// ```
pub(super) const fn get_vi_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Tệp không phải là định dạng `json 1.0` hợp lệ, cố gắng phân tích phân tích thành `json5` ..."##),
        ("not-included", r##"Binary ** này không bao gồm ** Chức năng chuyển đổi cho định dạng có liên quan.
Bạn cần bật tính năng có liên quan trong `Cargo.toml` và Recompile.
Nếu phần mềm này không bao gồm chức năng tương ứng, vui lòng gửi một vấn đề."##),
        ("currently-supported", r##"Danh sách các định dạng hiện được hỗ trợ:"##),
        ("unsupported", r##"Chuyển đổi định dạng không được hỗ trợ"##),
        ("auto-detection-failed", r##"Không thể tự động phát hiện định dạng.Vui lòng chỉ định thủ công."##),
        ("not-support-deser-sexp", r##"** Chưa được hỗ trợ"##),
        ("unknown-fmt", r##"Định dạng tệp không xác định"##),
        ("not-saved", r##"Nội dung sau ** sẽ không ** được lưu vì `--save` không được gọi."##),
        ("dst", r##"Đường dẫn tệp đích"##),
        ("conv-error", r##"Lỗi chuyển đổi"##),
    ],
}
}

/// Language ID: vi;
/// Map name: "conversion_md";
/// Description: Tiếng Việt, Chữ La tinh, Việt Nam;
pub(super) const fn get_vi_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mTệp không phải là định dạng [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m hợp lệ, cố gắng phân tích phân tích thành [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mBinary ** này không bao gồm ** Chức năng chuyển đổi cho định dạng có liên quan.
[48;2;34;34;34m[38;2;255;255;255mBạn cần bật tính năng có liên quan trong [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m và Recompile.
[48;2;34;34;34m[38;2;255;255;255mNếu phần mềm này không bao gồm chức năng tương ứng, vui lòng gửi một vấn đề.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mDanh sách các định dạng hiện được hỗ trợ:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mChuyển đổi định dạng không được hỗ trợ[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mKhông thể tự động phát hiện định dạng.Vui lòng chỉ định thủ công.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** Chưa được hỗ trợ[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mĐịnh dạng tệp không xác định[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mNội dung sau ** sẽ không ** được lưu vì [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m không được gọi.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mĐường dẫn tệp đích[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mLỗi chuyển đổi[0m"##),
    ],
}
}

/// Language ID: vi;
/// Map name: "set";
/// Description: Tiếng Việt, Chữ La tinh, Việt Nam;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Đường dẫn tệp không hợp lệ.");
/// ```
pub(super) const fn get_vi_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Nội dung được sửa đổi ** sẽ không được lưu ** vì `--sv` không được gọi."##),
        ("new-value", r##"Giá trị mới"##),
        ("invalid-path", r##"Đường dẫn tệp không hợp lệ."##),
    ],
}
}

/// Language ID: vi;
/// Map name: "set_md";
/// Description: Tiếng Việt, Chữ La tinh, Việt Nam;
pub(super) const fn get_vi_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mNội dung được sửa đổi ** sẽ không được lưu ** vì [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m không được gọi.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mGiá trị mới[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mĐường dẫn tệp không hợp lệ.[0m"##),
    ],
}
}

/// Language ID: vi;
/// Map name: "get";
/// Description: Tiếng Việt, Chữ La tinh, Việt Nam;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Định dạng đích");
/// ```
pub(super) const fn get_vi_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Định dạng đích"##),
        ("src-fmt", r##"Định dạng tệp nguồn"##),
    ],
}
}

/// vi: Tiếng Việt, Chữ La tinh, Việt Nam
pub(super) const fn get_vi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_vi_map_conversion),
        ("get", get_vi_map_get),
        ("set_md", get_vi_map_set_md),
        ("set", get_vi_map_set),
        ("conversion_md", get_vi_map_conversion_md),
    ],
}
}

/// Language ID: xh;
/// Map name: "conversion";
/// Description: IsiXhosa, IsiLatin, EMzantsi Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Ayiphumelelanga ukufumanisa ifomathi.Nceda uchaze ngesandla.");
/// ```
pub(super) const fn get_xh_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 6),
        (6, 0),
    ],
    entries: &[
        ("not-included", r##"Le binary ** ayibandakanyi umsebenzi wokuguqula yefomathi efanelekileyo.
Kuya kufuneka ukuba wenze into efanelekileyo kwi-`Cargo.toml` kwaye uphinde.
Ukuba le software ayibandakanyi ukusebenza ngokuchanekileyo, nceda ungenise umba."##),
        ("auto-detection-failed", r##"Ayiphumelelanga ukufumanisa ifomathi.Nceda uchaze ngesandla."##),
        ("not-support-deser-sexp", r##"*** ayixhaswanga kodwa **: Ukuguqula ukusuka kwi-Lisp s"##),
        ("dst", r##"indlela yefayile yokufika"##),
        ("unsupported", r##"Ifomathi engacwangciswanga"##),
        ("currently-supported", r##"LOLULEKILE OLEKILEYO FRAATS:"##),
        ("not-saved", r##"Lo mxholo ulandelayo ** awuyi kuba usindiswe kuba `--save` yayingabizwanga."##),
        ("unknown-fmt", r##"Ifomati engaziwayo yefayile"##),
        ("conv-error", r##"Impazamo yokuguqula"##),
    ],
}
}

/// Language ID: xh;
/// Map name: "conversion_md";
/// Description: IsiXhosa, IsiLatin, EMzantsi Afrika;
pub(super) const fn get_xh_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 6),
        (6, 0),
    ],
    entries: &[
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mLe binary ** ayibandakanyi umsebenzi wokuguqula yefomathi efanelekileyo.
[48;2;34;34;34m[38;2;255;255;255mKuya kufuneka ukuba wenze into efanelekileyo kwi-[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m kwaye uphinde.
[48;2;34;34;34m[38;2;255;255;255mUkuba le software ayibandakanyi ukusebenza ngokuchanekileyo, nceda ungenise umba.[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mAyiphumelelanga ukufumanisa ifomathi.Nceda uchaze ngesandla.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m*** ayixhaswanga kodwa [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: Ukuguqula ukusuka kwi-Lisp s[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mindlela yefayile yokufika[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mIfomathi engacwangciswanga[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLOLULEKILE OLEKILEYO FRAATS:[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mLo mxholo ulandelayo ** awuyi kuba usindiswe kuba [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m yayingabizwanga.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mIfomati engaziwayo yefayile[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mImpazamo yokuguqula[0m"##),
    ],
}
}

/// Language ID: xh;
/// Map name: "set";
/// Description: IsiXhosa, IsiLatin, EMzantsi Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "indlela engasebenziyo yefayile.");
/// ```
pub(super) const fn get_xh_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Umxholo oguqulweyo ** awuyi kugcinwa kuba `--sv`"##),
        ("new-value", r##"Ixabiso elitsha"##),
        ("invalid-path", r##"indlela engasebenziyo yefayile."##),
    ],
}
}

/// Language ID: xh;
/// Map name: "set_md";
/// Description: IsiXhosa, IsiLatin, EMzantsi Afrika;
pub(super) const fn get_xh_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mUmxholo oguqulweyo ** awuyi kugcinwa kuba [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mIxabiso elitsha[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mindlela engasebenziyo yefayile.[0m"##),
    ],
}
}

/// Language ID: xh;
/// Map name: "get";
/// Description: IsiXhosa, IsiLatin, EMzantsi Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Ifomathi yendawo oya kuyo");
/// ```
pub(super) const fn get_xh_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Ifomathi yendawo oya kuyo"##),
        ("src-fmt", r##"Ifomati yefayile yomthombo"##),
    ],
}
}

/// xh: IsiXhosa, IsiLatin, EMzantsi Afrika
pub(super) const fn get_xh_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_xh_map_conversion),
        ("get", get_xh_map_get),
        ("set_md", get_xh_map_set_md),
        ("set", get_xh_map_set),
        ("conversion_md", get_xh_map_conversion_md),
    ],
}
}

/// Language ID: yi;
/// Map name: "conversion";
/// Description: ייִדיש, העברעיש, וועלט;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "ניט א\u{5b7}נדערש צו דעטא\u{5b7}ילס די פ\u{5bf}א\u{5b8}רמא\u{5b7}ט.ביטע ספ\u{5bc}עציפיצירן מא\u{5b7}ניוא\u{5b7}לי.");
/// ```
pub(super) const fn get_yi_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
        (2, 1),
    ],
    entries: &[
        ("unsupported", r##"ניט-סופּפּאָרטעד פֿאָרמאַט קאַנווערזשאַן"##),
        ("conv-error", r##"קאַנווערזשאַן טעות"##),
        ("not-support-deser-sexp", r##"** ניט געשטיצט נאָך **: קאַנווערטינג פון `ליספּ ס-אויסדרוק` צו` אנדערע פאָרמאַץ`"##),
        ("invalid-json1.0", r##"דער טעקע איז נישט אַ גילטיק `json 1.0` פֿאָרמאַט, טריינג צו פּאַרס ווי `json5` ..."##),
        ("unknown-fmt", r##"אומבאַקאַנט טעקע פֿאָרמאַט"##),
        ("auto-detection-failed", r##"ניט אַנדערש צו דעטאַילס די פֿאָרמאַט.ביטע ספּעציפיצירן מאַניואַלי."##),
        ("currently-supported", r##"דערווייַל געשטיצט פֿאָרמאַטירונגען רשימה:"##),
        ("dst", r##"דעסטיניישאַן טעקע דרך"##),
    ],
}
}

/// Language ID: yi;
/// Map name: "conversion_md";
/// Description: ייִדיש, העברעיש, וועלט;
pub(super) const fn get_yi_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
        (2, 1),
    ],
    entries: &[
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mניט-סופּפּאָרטעד פֿאָרמאַט קאַנווערזשאַן[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mקאַנווערזשאַן טעות[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ניט געשטיצט נאָך [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m: קאַנווערטינג פון [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mליספּ ס-אויסדרוק[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;249;38;114m צו[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m אנדערע פאָרמאַץ[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mדער טעקע איז נישט אַ גילטיק [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m פֿאָרמאַט, טריינג צו פּאַרס ווי [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mאומבאַקאַנט טעקע פֿאָרמאַט[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mניט אַנדערש צו דעטאַילס די פֿאָרמאַט.ביטע ספּעציפיצירן מאַניואַלי.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mדערווייַל געשטיצט פֿאָרמאַטירונגען רשימה:[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mדעסטיניישאַן טעקע דרך[0m"##),
    ],
}
}

/// Language ID: yi;
/// Map name: "set";
/// Description: ייִדיש, העברעיש, וועלט;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "פא\u{5b7}רקריפ\u{5bc}לט טעקע דרך.");
/// ```
pub(super) const fn get_yi_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"די מאַדאַפייד אינהאַלט ** וועט ניט ** זיין געראטעוועט ווייַל `- - ס 'איז נישט גערופן."##),
        ("new-value", r##"ניו ווערט"##),
        ("invalid-path", r##"פאַרקריפּלט טעקע דרך."##),
    ],
}
}

/// Language ID: yi;
/// Map name: "set_md";
/// Description: ייִדיש, העברעיש, וועלט;
pub(super) const fn get_yi_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mדי מאַדאַפייד אינהאַלט ** וועט ניט ** זיין געראטעוועט ווייַל [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m- - ס 'איז נישט גערופן.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mניו ווערט[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mפאַרקריפּלט טעקע דרך.[0m"##),
    ],
}
}

/// Language ID: yi;
/// Map name: "get";
/// Description: ייִדיש, העברעיש, וועלט;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "דעסטיניישא\u{5b7}ן פ\u{5bf}א\u{5b8}רמא\u{5b7}ט");
/// ```
pub(super) const fn get_yi_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"דעסטיניישאַן פֿאָרמאַט"##),
        ("src-fmt", r##"מקור טעקע פֿאָרמאַט"##),
    ],
}
}

/// yi: ייִדיש, העברעיש, וועלט
pub(super) const fn get_yi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_yi_map_conversion),
        ("get", get_yi_map_get),
        ("set_md", get_yi_map_set_md),
        ("set", get_yi_map_set),
        ("conversion_md", get_yi_map_conversion_md),
    ],
}
}

/// Language ID: yo;
/// Map name: "conversion";
/// Description: Èdè Yorùbá, Èdè Látìn, Nàìjíríà;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Kuna lati wa taara ọna kika.Jọwọ ṣalaye pẹlu ọwọ.");
/// ```
pub(super) const fn get_yo_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"Awọn akoonu atẹle kii yoo ni igbala nitori `--save` ko pe."##),
        ("unknown-fmt", r##"Ọna kika faili aimọ"##),
        ("currently-supported", r##"Lọwọlọwọ awọn ọna kika ọna kika lọwọlọwọ:"##),
        ("auto-detection-failed", r##"Kuna lati wa taara ọna kika.Jọwọ ṣalaye pẹlu ọwọ."##),
        ("not-included", r##"Alakomeji ** ko pẹlu ** Awọn iṣẹ iyipada fun ọna kika to wulo.
O nilo lati ṣiṣẹ ẹya ti o yẹ ni rẹ `Cargo.toml` ati commpile.
Ti sọfitiwia yii ko pẹlu iṣẹ ti o baamu, jọwọ gbe oro kan."##),
        ("conv-error", r##"Aṣiṣe iyipada"##),
        ("dst", r##"Ọna faili ibina"##),
        ("unsupported", r##"Iyipada ọna kika ti ko ni iṣiro"##),
        ("invalid-json1.0", r##"Faili naa ko si ni ọna kika `json 1.0` to wulo, o ngbiyanju lati tutumọ si `json5`..."##),
    ],
}
}

/// Language ID: yo;
/// Map name: "conversion_md";
/// Description: Èdè Yorùbá, Èdè Látìn, Nàìjíríà;
pub(super) const fn get_yo_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (6, 0),
        (1, 0),
    ],
    entries: &[
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mAwọn akoonu atẹle kii yoo ni igbala nitori [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ko pe.[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mỌna kika faili aimọ[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mLọwọlọwọ awọn ọna kika ọna kika lọwọlọwọ:[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mKuna lati wa taara ọna kika.Jọwọ ṣalaye pẹlu ọwọ.[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mAlakomeji ** ko pẹlu ** Awọn iṣẹ iyipada fun ọna kika to wulo.
[48;2;34;34;34m[38;2;255;255;255mO nilo lati ṣiṣẹ ẹya ti o yẹ ni rẹ [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ati commpile.
[48;2;34;34;34m[38;2;255;255;255mTi sọfitiwia yii ko pẹlu iṣẹ ti o baamu, jọwọ gbe oro kan.[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mAṣiṣe iyipada[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mỌna faili ibina[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mIyipada ọna kika ti ko ni iṣiro[0m"##),
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mFaili naa ko si ni ọna kika [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m to wulo, o ngbiyanju lati tutumọ si [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
    ],
}
}

/// Language ID: yo;
/// Map name: "set";
/// Description: Èdè Yorùbá, Èdè Látìn, Nàìjíríà;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Ọna faili ti ko wulo.");
/// ```
pub(super) const fn get_yo_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Akoonu ti a tunṣe ** kii yoo ni igbala nitori `sv` a ko pe."##),
        ("new-value", r##"Iye tuntun"##),
        ("invalid-path", r##"Ọna faili ti ko wulo."##),
    ],
}
}

/// Language ID: yo;
/// Map name: "set_md";
/// Description: Èdè Yorùbá, Èdè Látìn, Nàìjíríà;
pub(super) const fn get_yo_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mAkoonu ti a tunṣe ** kii yoo ni igbala nitori [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255msv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m a ko pe.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mIye tuntun[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mỌna faili ti ko wulo.[0m"##),
    ],
}
}

/// Language ID: yo;
/// Map name: "get";
/// Description: Èdè Yorùbá, Èdè Látìn, Nàìjíríà;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Ọna kika");
/// ```
pub(super) const fn get_yo_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Ọna kika"##),
        ("src-fmt", r##"Ọna kika orisun"##),
    ],
}
}

/// yo: Èdè Yorùbá, Èdè Látìn, Nàìjíríà
pub(super) const fn get_yo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_yo_map_conversion),
        ("get", get_yo_map_get),
        ("set_md", get_yo_map_set_md),
        ("set", get_yo_map_set),
        ("conversion_md", get_yo_map_conversion_md),
    ],
}
}

/// Language ID: zh;
/// Map name: "conversion";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "自动检测格式失败，请手动指定。");
/// ```
pub(super) const fn get_zh_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"该文件不是有效的 `json 1.0` 格式, 正在尝试解析为 `json5`..."##),
        ("not-included", r##"此二进制文件 **不包含** 相关格式的转换功能。
您需要在 `Cargo.toml` 里启用相关的功能，并重新编译。
如果此软件不包含相关功能，请提交 issue。"##),
        ("currently-supported", r##"当前支持的格式列表"##),
        ("unsupported", r##"不支持的格式转换"##),
        ("auto-detection-failed", r##"自动检测格式失败，请手动指定。"##),
        ("not-support-deser-sexp", r##"**暂不支持** 将 `Lisp S-Expression` 转换为 `其他格式`"##),
        ("unknown-fmt", r##"未知的文件格式"##),
        ("not-saved", r##"由于未调用 `--save`, 故以下内容 **不会** 被保存。"##),
        ("dst", r##"目标文件路径"##),
        ("conv-error", r##"转换错误"##),
    ],
}
}

/// Language ID: zh;
/// Map name: "conversion_md";
/// Description: 简体中文, 中国;
pub(super) const fn get_zh_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255m该文件不是有效的 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 格式, 正在尝试解析为 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255m此二进制文件 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不包含[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 相关格式的转换功能。
[48;2;34;34;34m[38;2;255;255;255m您需要在 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 里启用相关的功能，并重新编译。
[48;2;34;34;34m[38;2;255;255;255m如果此软件不包含相关功能，请提交 issue。[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255m当前支持的格式列表[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255m不支持的格式转换[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255m自动检测格式失败，请手动指定。[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m暂不支持[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 将 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 转换为 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m其他格式[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255m未知的文件格式[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255m由于未调用 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, 故以下内容 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不会[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 被保存。[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255m目标文件路径[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255m转换错误[0m"##),
    ],
}
}

/// Language ID: zh;
/// Map name: "get";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "目标格式");
/// ```
pub(super) const fn get_zh_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"目标格式"##),
        ("src-fmt", r##"源文件格式"##),
    ],
}
}

/// Language ID: zh;
/// Map name: "set";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "无效的文件路径");
/// ```
pub(super) const fn get_zh_map_set() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (4, 0),
    ],
    entries: &[
        ("value", r##"值(value)"##),
        ("type", r##"类型"##),
        ("unsave-warn", r##"由于未调用 `--save`, 故已修改的内容 **不会** 被保存。"##),
        ("invalid-path", r##"无效的文件路径"##),
        ("new-value", r##"新值(value)"##),
    ],
}
}

/// Language ID: zh;
/// Map name: "set_md";
/// Description: 简体中文, 中国;
pub(super) const fn get_zh_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (4, 0),
    ],
    entries: &[
        ("value", r##"[48;2;34;34;34m[38;2;255;255;255m值(value)[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255m类型[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255m由于未调用 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, 故已修改的内容 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不会[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 被保存。[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255m无效的文件路径[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255m新值(value)[0m"##),
    ],
}
}

/// zh: 简体中文, 中国
pub(super) const fn get_zh_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_zh_map_conversion),
        ("get", get_zh_map_get),
        ("set_md", get_zh_map_set_md),
        ("set", get_zh_map_set),
        ("conversion_md", get_zh_map_conversion_md),
    ],
}
}

/// Language ID: zh-Hant;
/// Map name: "conversion";
/// Description: 正體中文, 中國台灣;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "自動檢測格式失敗，請手動指定。");
/// ```
pub(super) const fn get_zh_hant_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"該檔案不是有效的 `json 1.0` 格式, 正在嘗試解析為 `json5`..."##),
        ("not-included", r##"此二進位制檔案 **不包含** 相關格式的轉換功能。
您需要在 `Cargo.toml` 裡啟用相關的功能，並重新編譯。
如果此軟體不包含相關功能，請提交 issue。"##),
        ("currently-supported", r##"當前支援的格式列表"##),
        ("unsupported", r##"不支援的格式轉換"##),
        ("auto-detection-failed", r##"自動檢測格式失敗，請手動指定。"##),
        ("not-support-deser-sexp", r##"**暫不支援** 將 `Lisp S-Expression` 轉換為 `其他格式`"##),
        ("unknown-fmt", r##"未知的檔案格式"##),
        ("not-saved", r##"由於未呼叫 `--save`, 故以下內容 **不會** 被儲存。"##),
        ("dst", r##"目標檔案路徑"##),
        ("conv-error", r##"轉換錯誤"##),
    ],
}
}

/// Language ID: zh-Hant;
/// Map name: "conversion_md";
/// Description: 正體中文, 中國台灣;
pub(super) const fn get_zh_hant_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255m該檔案不是有效的 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 格式, 正在嘗試解析為 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255m此二進位制檔案 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不包含[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 相關格式的轉換功能。
[48;2;34;34;34m[38;2;255;255;255m您需要在 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 裡啟用相關的功能，並重新編譯。
[48;2;34;34;34m[38;2;255;255;255m如果此軟體不包含相關功能，請提交 issue。[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255m當前支援的格式列表[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255m不支援的格式轉換[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255m自動檢測格式失敗，請手動指定。[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m暫不支援[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 將 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 轉換為 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m其他格式[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255m未知的檔案格式[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255m由於未呼叫 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, 故以下內容 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不會[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 被儲存。[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255m目標檔案路徑[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255m轉換錯誤[0m"##),
    ],
}
}

/// Language ID: zh-Hant;
/// Map name: "get";
/// Description: 正體中文, 中國台灣;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "目標格式");
/// ```
pub(super) const fn get_zh_hant_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"目標格式"##),
        ("src-fmt", r##"原始檔格式"##),
    ],
}
}

/// Language ID: zh-Hant;
/// Map name: "set";
/// Description: 正體中文, 中國台灣;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "無效的檔案路徑");
/// ```
pub(super) const fn get_zh_hant_map_set() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"無效的檔案路徑"##),
        ("new-value", r##"新值(value)"##),
        ("unsave-warn", r##"由於未呼叫 `--save`, 故已修改的內容 **不會** 被儲存。"##),
        ("type", r##"型別"##),
    ],
}
}

/// Language ID: zh-Hant;
/// Map name: "set_md";
/// Description: 正體中文, 中國台灣;
pub(super) const fn get_zh_hant_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255m無效的檔案路徑[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255m新值(value)[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255m由於未呼叫 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, 故已修改的內容 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不會[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 被儲存。[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255m型別[0m"##),
    ],
}
}

/// zh-Hant: 正體中文, 中國台灣
pub(super) const fn get_zh_hant_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_zh_hant_map_conversion),
        ("get", get_zh_hant_map_get),
        ("set_md", get_zh_hant_map_set_md),
        ("set", get_zh_hant_map_set),
        ("conversion_md", get_zh_hant_map_conversion_md),
    ],
}
}

/// Language ID: zh-Hant-HK;
/// Map name: "conversion";
/// Description: 繁體中文, 中國香港;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "自動檢測格式失敗，請手動指定。");
/// ```
pub(super) const fn get_zh_hant_hk_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"該文件不是有效的 `json 1.0` 格式, 正在嘗試解析為 `json5`..."##),
        ("not-included", r##"此二進制文件 **不包含** 相關格式的轉換功能。
您需要在 `Cargo.toml` 裏啓用相關的功能，並重新編譯。
如果此軟件不包含相關功能，請提交 issue。"##),
        ("currently-supported", r##"當前支持的格式列表"##),
        ("unsupported", r##"不支持的格式轉換"##),
        ("auto-detection-failed", r##"自動檢測格式失敗，請手動指定。"##),
        ("not-support-deser-sexp", r##"**暫不支持** 將 `Lisp S-Expression` 轉換為 `其他格式`"##),
        ("unknown-fmt", r##"未知的文件格式"##),
        ("not-saved", r##"由於未調用 `--save`, 故以下內容 **不會** 被保存。"##),
        ("dst", r##"目標文件路徑"##),
        ("conv-error", r##"轉換錯誤"##),
    ],
}
}

/// Language ID: zh-Hant-HK;
/// Map name: "conversion_md";
/// Description: 繁體中文, 中國香港;
pub(super) const fn get_zh_hant_hk_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255m該文件不是有效的 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 格式, 正在嘗試解析為 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255m此二進制文件 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不包含[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 相關格式的轉換功能。
[48;2;34;34;34m[38;2;255;255;255m您需要在 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 裏啓用相關的功能，並重新編譯。
[48;2;34;34;34m[38;2;255;255;255m如果此軟件不包含相關功能，請提交 issue。[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255m當前支持的格式列表[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255m不支持的格式轉換[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255m自動檢測格式失敗，請手動指定。[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m暫不支持[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 將 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mLisp S-Expression[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m 轉換為 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m其他格式[48;2;34;34;34m[38;2;0;255;255m`[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255m未知的文件格式[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255m由於未調用 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, 故以下內容 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不會[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 被保存。[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255m目標文件路徑[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255m轉換錯誤[0m"##),
    ],
}
}

/// Language ID: zh-Hant-HK;
/// Map name: "get";
/// Description: 繁體中文, 中國香港;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "目標格式");
/// ```
pub(super) const fn get_zh_hant_hk_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"目標格式"##),
        ("src-fmt", r##"源文件格式"##),
    ],
}
}

/// Language ID: zh-Hant-HK;
/// Map name: "set";
/// Description: 繁體中文, 中國香港;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "無效的文件路徑");
/// ```
pub(super) const fn get_zh_hant_hk_map_set() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"無效的文件路徑"##),
        ("new-value", r##"新值(value)"##),
        ("unsave-warn", r##"由於未調用 `--save`, 故已修改的內容 **不會** 被保存。"##),
        ("type", r##"類型"##),
    ],
}
}

/// Language ID: zh-Hant-HK;
/// Map name: "set_md";
/// Description: 繁體中文, 中國香港;
pub(super) const fn get_zh_hant_hk_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 14108922650502679131,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255m無效的文件路徑[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255m新值(value)[0m"##),
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255m由於未調用 [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, 故已修改的內容 [48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;249;38;114m不會[48;2;34;34;34m[38;2;249;38;114m**[48;2;34;34;34m[38;2;255;255;255m 被保存。[0m"##),
        ("type", r##"[48;2;34;34;34m[38;2;255;255;255m類型[0m"##),
    ],
}
}

/// zh-Hant-HK: 繁體中文, 中國香港
pub(super) const fn get_zh_hant_hk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_zh_hant_hk_map_conversion),
        ("get", get_zh_hant_hk_map_get),
        ("set_md", get_zh_hant_hk_map_set_md),
        ("set", get_zh_hant_hk_map_set),
        ("conversion_md", get_zh_hant_hk_map_conversion_md),
    ],
}
}

/// Language ID: zu;
/// Map name: "conversion";
/// Description: isiZulu, isi-Latin, iNingizimu Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("conversion", "auto-detection-failed");
///
/// assert_eq!(msg, "Yehlulekile ukuthola ngokuzenzakalelayo ifomethi.Sicela usho ngesandla.");
/// ```
pub(super) const fn get_zu_map_conversion() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"Ifayela alilona ifomethi evumelekile `JSON 1.0`, izama ukunxusa njengoba `json5` ..."##),
        ("not-included", r##"Le kanambambili ** ayifaki ** Ukuguqulwa kokusebenza kwefomethi efanele.
Udinga ukunika amandla isici esifanelekile ku-`Cargo.toml`.Tom` kanye ne-Relompyile.
Uma le software ingafaki ukusebenza okuhambisanayo, sicela uthumele inkinga."##),
        ("currently-supported", r##"Amafomethi asekelwe njengamanje Uhlu:"##),
        ("unsupported", r##"Ukuguqulwa kwenkundla okungasekelwa"##),
        ("auto-detection-failed", r##"Yehlulekile ukuthola ngokuzenzakalelayo ifomethi.Sicela usho ngesandla."##),
        ("not-support-deser-sexp", r##"** ** AKUFUNIWE okwamanje"##),
        ("unknown-fmt", r##"Angaziwa Ifayela"##),
        ("not-saved", r##"Okuqukethwe okulandelayo ** ngeke kugcinwe ** kugcinwe ngoba `--save` akuzange kufonele."##),
        ("dst", r##"Indlela yefayela lendawo"##),
        ("conv-error", r##"Iphutha lokuguqulwa"##),
    ],
}
}

/// Language ID: zu;
/// Map name: "conversion_md";
/// Description: isiZulu, isi-Latin, iNingizimu Afrika;
pub(super) const fn get_zu_map_conversion_md() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
        (6, 0),
    ],
    entries: &[
        ("invalid-json1.0", r##"[48;2;34;34;34m[38;2;255;255;255mIfayela alilona ifomethi evumelekile [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mJSON 1.0[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m, izama ukunxusa njengoba [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mjson5[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m ...[0m"##),
        ("not-included", r##"[48;2;34;34;34m[38;2;255;255;255mLe kanambambili ** ayifaki ** Ukuguqulwa kokusebenza kwefomethi efanele.
[48;2;34;34;34m[38;2;255;255;255mUdinga ukunika amandla isici esifanelekile ku-[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255mCargo.toml[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m.Tom[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m kanye ne-Relompyile.
[48;2;34;34;34m[38;2;0;255;255mUma le software ingafaki ukusebenza okuhambisanayo, sicela uthumele inkinga.[0m"##),
        ("currently-supported", r##"[48;2;34;34;34m[38;2;255;255;255mAmafomethi asekelwe njengamanje Uhlu:[0m"##),
        ("unsupported", r##"[48;2;34;34;34m[38;2;255;255;255mUkuguqulwa kwenkundla okungasekelwa[0m"##),
        ("auto-detection-failed", r##"[48;2;34;34;34m[38;2;255;255;255mYehlulekile ukuthola ngokuzenzakalelayo ifomethi.Sicela usho ngesandla.[0m"##),
        ("not-support-deser-sexp", r##"[48;2;34;34;34m[38;2;255;255;255m** ** AKUFUNIWE okwamanje[0m"##),
        ("unknown-fmt", r##"[48;2;34;34;34m[38;2;255;255;255mAngaziwa Ifayela[0m"##),
        ("not-saved", r##"[48;2;34;34;34m[38;2;255;255;255mOkuqukethwe okulandelayo ** ngeke kugcinwe ** kugcinwe ngoba [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--save[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m akuzange kufonele.[0m"##),
        ("dst", r##"[48;2;34;34;34m[38;2;255;255;255mIndlela yefayela lendawo[0m"##),
        ("conv-error", r##"[48;2;34;34;34m[38;2;255;255;255mIphutha lokuguqulwa[0m"##),
    ],
}
}

/// Language ID: zu;
/// Map name: "set";
/// Description: isiZulu, isi-Latin, iNingizimu Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("set", "invalid-path");
///
/// assert_eq!(msg, "Indlela yefayela elingavumelekile.");
/// ```
pub(super) const fn get_zu_map_set() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"Okuqukethwe okulungisiwe ** ngeke kugcinwe ** kugcinwe ngoba `--sv` akubizwanga."##),
        ("new-value", r##"Inani elisha"##),
        ("invalid-path", r##"Indlela yefayela elingavumelekile."##),
    ],
}
}

/// Language ID: zu;
/// Map name: "set_md";
/// Description: isiZulu, isi-Latin, iNingizimu Afrika;
pub(super) const fn get_zu_map_set_md() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("unsave-warn", r##"[48;2;34;34;34m[38;2;255;255;255mOkuqukethwe okulungisiwe ** ngeke kugcinwe ** kugcinwe ngoba [48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;0;255;255m--sv[48;2;34;34;34m[38;2;0;255;255m`[48;2;34;34;34m[38;2;255;255;255m akubizwanga.[0m"##),
        ("new-value", r##"[48;2;34;34;34m[38;2;255;255;255mInani elisha[0m"##),
        ("invalid-path", r##"[48;2;34;34;34m[38;2;255;255;255mIndlela yefayela elingavumelekile.[0m"##),
    ],
}
}

/// Language ID: zu;
/// Map name: "get";
/// Description: isiZulu, isi-Latin, iNingizimu Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("get", "dst-fmt");
///
/// assert_eq!(msg, "Ifomethi Yendawo");
/// ```
pub(super) const fn get_zu_map_get() -> L10nMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("dst-fmt", r##"Ifomethi Yendawo"##),
        ("src-fmt", r##"Ifomethi yefayela lomthombo"##),
    ],
}
}

/// zu: isiZulu, isi-Latin, iNingizimu Afrika
pub(super) const fn get_zu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (2, 0),
    ],
    entries: &[
        ("conversion", get_zu_map_conversion),
        ("get", get_zu_map_get),
        ("set_md", get_zu_map_set_md),
        ("set", get_zu_map_set),
        ("conversion_md", get_zu_map_conversion_md),
    ],
}
}

/// # Example
///
/// ```no_run
/// let map = locale_map();
///
/// for k in map.keys() {
///     println!("{k}")
/// }
///
/// map.get("en").map(|v| dbg!(v()));
/// ```
pub(super) const fn locale_map() -> LocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 1),
        (13, 20),
        (0, 4),
        (0, 75),
        (0, 30),
        (0, 13),
        (0, 10),
        (1, 9),
        (0, 80),
        (6, 8),
        (0, 22),
        (0, 0),
        (5, 55),
        (1, 39),
        (3, 105),
        (2, 7),
        (1, 1),
        (0, 1),
        (19, 66),
        (0, 104),
        (0, 5),
        (1, 62),
    ],
    entries: &[
        ("lo", get_lo_map),
        ("ml", get_ml_map),
        ("ne", get_ne_map),
        ("ht", get_ht_map),
        ("sm", get_sm_map),
        ("sl", get_sl_map),
        ("en", get_en_map),
        ("ro", get_ro_map),
        ("yi", get_yi_map),
        ("et", get_et_map),
        ("mg", get_mg_map),
        ("la", get_la_map),
        ("no", get_no_map),
        ("ur", get_ur_map),
        ("hr", get_hr_map),
        ("vi", get_vi_map),
        ("ku", get_ku_map),
        ("ru", get_ru_map),
        ("haw", get_haw_map),
        ("de", get_de_map),
        ("sv", get_sv_map),
        ("he", get_he_map),
        ("yo", get_yo_map),
        ("st", get_st_map),
        ("mt", get_mt_map),
        ("th", get_th_map),
        ("tr", get_tr_map),
        ("cs", get_cs_map),
        ("so", get_so_map),
        ("ta", get_ta_map),
        ("uk", get_uk_map),
        ("ar", get_ar_map),
        ("ko", get_ko_map),
        ("sn", get_sn_map),
        ("ug", get_ug_map),
        ("sr", get_sr_map),
        ("jw", get_jw_map),
        ("te", get_te_map),
        ("ig", get_ig_map),
        ("gl", get_gl_map),
        ("bg", get_bg_map),
        ("ceb", get_ceb_map),
        ("co", get_co_map),
        ("ny", get_ny_map),
        ("hu", get_hu_map),
        ("zu", get_zu_map),
        ("eu", get_eu_map),
        ("zh", get_zh_map),
        ("km", get_km_map),
        ("gu", get_gu_map),
        ("fil", get_fil_map),
        ("sk", get_sk_map),
        ("sd", get_sd_map),
        ("ga", get_ga_map),
        ("af", get_af_map),
        ("lb", get_lb_map),
        ("ha", get_ha_map),
        ("fa", get_fa_map),
        ("ps", get_ps_map),
        ("ky", get_ky_map),
        ("fi", get_fi_map),
        ("hy", get_hy_map),
        ("fy", get_fy_map),
        ("mk", get_mk_map),
        ("ms", get_ms_map),
        ("nl", get_nl_map),
        ("da", get_da_map),
        ("es", get_es_map),
        ("sq", get_sq_map),
        ("my", get_my_map),
        ("kk", get_kk_map),
        ("be", get_be_map),
        ("kn", get_kn_map),
        ("tg", get_tg_map),
        ("xh", get_xh_map),
        ("uz", get_uz_map),
        ("si", get_si_map),
        ("bs", get_bs_map),
        ("bn", get_bn_map),
        ("mi", get_mi_map),
        ("pl", get_pl_map),
        ("mn", get_mn_map),
        ("ka", get_ka_map),
        ("is", get_is_map),
        ("zh-Hant", get_zh_hant_map),
        ("ja", get_ja_map),
        ("lv", get_lv_map),
        ("pa", get_pa_map),
        ("ca", get_ca_map),
        ("gd", get_gd_map),
        ("cy", get_cy_map),
        ("hi", get_hi_map),
        ("id", get_id_map),
        ("eo", get_eo_map),
        ("su", get_su_map),
        ("lt", get_lt_map),
        ("am", get_am_map),
        ("az", get_az_map),
        ("zh-Hant-HK", get_zh_hant_hk_map),
        ("fr", get_fr_map),
        ("el", get_el_map),
        ("sw", get_sw_map),
        ("pt", get_pt_map),
        ("it", get_it_map),
        ("or", get_or_map),
        ("mr", get_mr_map),
    ],
}
}

/// # Example
///
/// ```no_run
/// let loader = glossa::MapLoader::new(locale_hashmap());
///
/// dbg!(&loader);
/// ```
pub(super) fn locale_hashmap() -> LocaleHashMap {
    use lang_id_consts::*;

    HashMap::from_iter([
        (unsafe { get_af() }, get_af_map()),
        (unsafe { get_am() }, get_am_map()),
        (unsafe { get_ar() }, get_ar_map()),
        (unsafe { get_az() }, get_az_map()),
        (unsafe { get_be() }, get_be_map()),
        (unsafe { get_bg() }, get_bg_map()),
        (unsafe { get_bn() }, get_bn_map()),
        (unsafe { get_bs() }, get_bs_map()),
        (unsafe { get_ca() }, get_ca_map()),
        (unsafe { get_ceb() }, get_ceb_map()),
        (unsafe { get_co() }, get_co_map()),
        (unsafe { get_cs() }, get_cs_map()),
        (unsafe { get_cy() }, get_cy_map()),
        (unsafe { get_da() }, get_da_map()),
        (unsafe { get_de() }, get_de_map()),
        (unsafe { get_el() }, get_el_map()),
        (unsafe { get_en() }, get_en_map()),
        (unsafe { get_eo() }, get_eo_map()),
        (unsafe { get_es() }, get_es_map()),
        (unsafe { get_et() }, get_et_map()),
        (unsafe { get_eu() }, get_eu_map()),
        (unsafe { get_fa() }, get_fa_map()),
        (unsafe { get_fi() }, get_fi_map()),
        (unsafe { get_fil() }, get_fil_map()),
        (unsafe { get_fr() }, get_fr_map()),
        (unsafe { get_fy() }, get_fy_map()),
        (unsafe { get_ga() }, get_ga_map()),
        (unsafe { get_gd() }, get_gd_map()),
        (unsafe { get_gl() }, get_gl_map()),
        (unsafe { get_gu() }, get_gu_map()),
        (unsafe { get_ha() }, get_ha_map()),
        (unsafe { get_haw() }, get_haw_map()),
        (unsafe { get_he() }, get_he_map()),
        (unsafe { get_hi() }, get_hi_map()),
        (unsafe { get_hr() }, get_hr_map()),
        (unsafe { get_ht() }, get_ht_map()),
        (unsafe { get_hu() }, get_hu_map()),
        (unsafe { get_hy() }, get_hy_map()),
        (unsafe { get_id() }, get_id_map()),
        (unsafe { get_ig() }, get_ig_map()),
        (unsafe { get_is() }, get_is_map()),
        (unsafe { get_it() }, get_it_map()),
        (unsafe { get_ja() }, get_ja_map()),
        (unsafe { get_jw() }, get_jw_map()),
        (unsafe { get_ka() }, get_ka_map()),
        (unsafe { get_kk() }, get_kk_map()),
        (unsafe { get_km() }, get_km_map()),
        (unsafe { get_kn() }, get_kn_map()),
        (unsafe { get_ko() }, get_ko_map()),
        (unsafe { get_ku() }, get_ku_map()),
        (unsafe { get_ky() }, get_ky_map()),
        (unsafe { get_la() }, get_la_map()),
        (unsafe { get_lb() }, get_lb_map()),
        (unsafe { get_lo() }, get_lo_map()),
        (unsafe { get_lt() }, get_lt_map()),
        (unsafe { get_lv() }, get_lv_map()),
        (unsafe { get_mg() }, get_mg_map()),
        (unsafe { get_mi() }, get_mi_map()),
        (unsafe { get_mk() }, get_mk_map()),
        (unsafe { get_ml() }, get_ml_map()),
        (unsafe { get_mn() }, get_mn_map()),
        (unsafe { get_mr() }, get_mr_map()),
        (unsafe { get_ms() }, get_ms_map()),
        (unsafe { get_mt() }, get_mt_map()),
        (unsafe { get_my() }, get_my_map()),
        (unsafe { get_ne() }, get_ne_map()),
        (unsafe { get_nl() }, get_nl_map()),
        (unsafe { get_no() }, get_no_map()),
        (unsafe { get_ny() }, get_ny_map()),
        (unsafe { get_or() }, get_or_map()),
        (unsafe { get_pa() }, get_pa_map()),
        (unsafe { get_pl() }, get_pl_map()),
        (unsafe { get_ps() }, get_ps_map()),
        (unsafe { get_pt() }, get_pt_map()),
        (unsafe { get_ro() }, get_ro_map()),
        (unsafe { get_ru() }, get_ru_map()),
        (unsafe { get_sd() }, get_sd_map()),
        (unsafe { get_si() }, get_si_map()),
        (unsafe { get_sk() }, get_sk_map()),
        (unsafe { get_sl() }, get_sl_map()),
        (unsafe { get_sm() }, get_sm_map()),
        (unsafe { get_sn() }, get_sn_map()),
        (unsafe { get_so() }, get_so_map()),
        (unsafe { get_sq() }, get_sq_map()),
        (unsafe { get_sr() }, get_sr_map()),
        (unsafe { get_st() }, get_st_map()),
        (unsafe { get_su() }, get_su_map()),
        (unsafe { get_sv() }, get_sv_map()),
        (unsafe { get_sw() }, get_sw_map()),
        (unsafe { get_ta() }, get_ta_map()),
        (unsafe { get_te() }, get_te_map()),
        (unsafe { get_tg() }, get_tg_map()),
        (unsafe { get_th() }, get_th_map()),
        (unsafe { get_tr() }, get_tr_map()),
        (unsafe { get_ug() }, get_ug_map()),
        (unsafe { get_uk() }, get_uk_map()),
        (unsafe { get_ur() }, get_ur_map()),
        (unsafe { get_uz() }, get_uz_map()),
        (unsafe { get_vi() }, get_vi_map()),
        (unsafe { get_xh() }, get_xh_map()),
        (unsafe { get_yi() }, get_yi_map()),
        (unsafe { get_yo() }, get_yo_map()),
        (unsafe { get_zh() }, get_zh_map()),
        (unsafe { get_zh_hant() }, get_zh_hant_map()),
        (unsafe { get_zh_hant_hk() }, get_zh_hant_hk_map()),
        (unsafe { get_zu() }, get_zu_map()),
    ])
}
