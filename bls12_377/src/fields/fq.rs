use ark_ff::{
    biginteger::{BigInt, BigInteger384 as BigInteger},
    fields::*,
};

pub type Fq = Fp384<FqParameters>;

pub struct FqParameters;

impl Fp384Parameters for FqParameters {}
impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 46u32;

    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInt::new([
        2022196864061697551u64,
        17419102863309525423u64,
        8564289679875062096u64,
        17152078065055548215u64,
        17966377291017729567u64,
        68610905582439508u64,
    ]);
}
impl FpParameters for FqParameters {
    /// MODULUS = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInt::new([
        0x8508c00000000001,
        0x170b5d4430000000,
        0x1ef3622fba094800,
        0x1a22d9f300f5138f,
        0xc63b05c06ca1493b,
        0x1ae3a4617c510ea,
    ]);

    const MODULUS_BITS: u32 = 377;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 7;

    /// R = 85013442423176922659824578519796707547925331718418265885885478904210582549405549618995257669764901891699128663912
    #[rustfmt::skip]
    const R: BigInteger = BigInt::new([
        202099033278250856u64,
        5854854902718660529u64,
        11492539364873682930u64,
        8885205928937022213u64,
        5545221690922665192u64,
        39800542322357402u64,
    ]);

    #[rustfmt::skip]
    const R2: BigInteger = BigInt::new([
        0xb786686c9400cd22,
        0x329fcaab00431b1,
        0x22a5f11162d6b46d,
        0xbfdf7d03827dc3ac,
        0x837e92f041790bf9,
        0x6dfccb1e914b88,
    ]);

    const INV: u64 = 9586122913090633727u64;

    /// GENERATOR = -5
    /// Encoded in Montgomery form, so the value here is
    /// (-5 * R) % q = 92261639910053574722182574790803529333160366917737991650341130812388023949653897454961487930322210790384999596794
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInt::new([
        0xfc0b8000000002fa,
        0x97d39cf6e000018b,
        0x2072420fbfa05044,
        0xcbbcbd50d97c3802,
        0xbaf1ec35813f9eb,
        0x9974a2c0945ad2,
    ]);

    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInt::new([
        0x4284600000000000,
        0xb85aea218000000,
        0x8f79b117dd04a400,
        0x8d116cf9807a89c7,
        0x631d82e03650a49d,
        0xd71d230be28875,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T
    // For T coprime to 2

    // T = (MODULUS - 1) // 2^S =
    // 3675842578061421676390135839012792950148785745837396071634149488243117337281387659330802195819009059
    #[rustfmt::skip]
    const T: BigInteger = BigInt::new([
        0x7510c00000021423,
        0x88bee82520005c2d,
        0x67cc03d44e3c7bcd,
        0x1701b28524ec688b,
        0xe9185f1443ab18ec,
        0x6b8,
    ]);

    // (T - 1) // 2 =
    // 1837921289030710838195067919506396475074392872918698035817074744121558668640693829665401097909504529
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInt::new([
        0xba88600000010a11,
        0xc45f741290002e16,
        0xb3e601ea271e3de6,
        0xb80d94292763445,
        0x748c2f8a21d58c76,
        0x35c,
    ]);
}

#[allow(dead_code)]
pub const FQ_ONE: Fq = Fq::new(FqParameters::R);
#[allow(dead_code)]
pub const FQ_ZERO: Fq = Fq::new(BigInt::new([0, 0, 0, 0, 0, 0]));
