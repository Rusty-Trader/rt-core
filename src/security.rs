use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    #[serde(rename(deserialize = "equity"))]
    Equity,
}


#[derive(Debug, Deserialize, PartialEq)]
pub enum Security {
    Equity(Equity)
}

impl Security {

    pub fn security_type(&self) -> SecurityType {
        match self {
            Self::Equity(_) => SecurityType::Equity,
        }
    }

    pub fn get_minimum_price_variation(&self) -> f64 {
        match self {
            Self::Equity(x) => {
                x.minimum_price_variation
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecuritySymbol {
    Equity(String)
}

impl SecuritySymbol {
    pub fn symbol(&self) -> String {
        match self {
            Self::Equity(x) => x.clone(),
        }
    }

    pub fn security_type(&self) -> SecurityType {
        match self {
            Self::Equity(_) => SecurityType::Equity,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Equity {
    currency: Currency,
    minimum_price_variation: f64
}

impl Equity {

    pub fn new(currency: Currency, minimum_price_variation: f64) -> Self {
        Self {
            currency,
            minimum_price_variation
        }
    }

    pub fn get_currency(&self) -> Currency {
        self.currency
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Currency {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BOV,
    BRL,
    BSD,
    BTN,
    BWP,
    BYN,
    BZD,
    CAD,
    CDF,
    CHE,
    CHF,
    CHW,
    CLF,
    CLP,
    COP,
    COU,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    IRR,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MXV,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    CNY,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLE,
    SLL,
    SOS,
    SRD,
    SSP,
    STN,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    USN,
    UYI,
    UYU,
    UYW,
    UZS,
    VED,
    VES,
    VND,
    VUV,
    WST,
    XAF,
    XAG,
    XAU,
    XBA,
    XBB,
    XBC,
    XBD,
    XCD,
    XDR,
    XOF,
    XPD,
    XPF,
    XPT,
    XSU,
    XTS,
    XUA,
    YER,
    ZAR,
    ZMW,
    ZWL
}
