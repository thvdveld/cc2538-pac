#[doc = "Register `CTRL_OPTIONS` reader"]
pub struct R(crate::R<CTRL_OPTIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_OPTIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_OPTIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_OPTIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEYSTORE` reader - KEY STORE is available."]
pub type KEYSTORE_R = crate::BitReader<bool>;
#[doc = "Field `AES` reader - AES core is available."]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `HASH` reader - HASH Core is available."]
pub type HASH_R = crate::BitReader<bool>;
#[doc = "Field `AES_128` reader - AES core supports 128-bit keys."]
pub type AES_128_R = crate::BitReader<bool>;
#[doc = "Field `AES_256` reader - AES core supports 256-bit keys Note: If both AES-128 and AES-256 are set to one, the AES core supports 192-bit keys as well."]
pub type AES_256_R = crate::BitReader<bool>;
#[doc = "Field `AES_GCM` reader - AES-GCM is available as a single operation."]
pub type AES_GCM_R = crate::BitReader<bool>;
#[doc = "Field `AES_CCM` reader - AES-CCM is available as a single operation."]
pub type AES_CCM_R = crate::BitReader<bool>;
#[doc = "Field `SHA_256` reader - The HASH core supports SHA-256."]
pub type SHA_256_R = crate::BitReader<bool>;
#[doc = "Field `AHBINTERFACE` reader - AHB interface is available If this bit is 0, the EIP-120t has a TCM interface."]
pub type AHBINTERFACE_R = crate::BitReader<bool>;
#[doc = "Field `TYPE` reader - This field is 0x01 for the TYPE1 device."]
pub type TYPE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - KEY STORE is available."]
    #[inline(always)]
    pub fn keystore(&self) -> KEYSTORE_R {
        KEYSTORE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES core is available."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HASH Core is available."]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - AES core supports 128-bit keys."]
    #[inline(always)]
    pub fn aes_128(&self) -> AES_128_R {
        AES_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AES core supports 256-bit keys Note: If both AES-128 and AES-256 are set to one, the AES core supports 192-bit keys as well."]
    #[inline(always)]
    pub fn aes_256(&self) -> AES_256_R {
        AES_256_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AES-GCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_gcm(&self) -> AES_GCM_R {
        AES_GCM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AES-CCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_ccm(&self) -> AES_CCM_R {
        AES_CCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The HASH core supports SHA-256."]
    #[inline(always)]
    pub fn sha_256(&self) -> SHA_256_R {
        SHA_256_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - AHB interface is available If this bit is 0, the EIP-120t has a TCM interface."]
    #[inline(always)]
    pub fn ahbinterface(&self) -> AHBINTERFACE_R {
        AHBINTERFACE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - This field is 0x01 for the TYPE1 device."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Options register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_options](index.html) module"]
pub struct CTRL_OPTIONS_SPEC;
impl crate::RegisterSpec for CTRL_OPTIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_options::R](R) reader structure"]
impl crate::Readable for CTRL_OPTIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTRL_OPTIONS to value 0"]
impl crate::Resettable for CTRL_OPTIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
