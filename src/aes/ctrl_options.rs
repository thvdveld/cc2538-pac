#[doc = "Register `CTRL_OPTIONS` reader"]
pub type R = crate::R<CtrlOptionsSpec>;
#[doc = "Field `KEYSTORE` reader - KEY STORE is available."]
pub type KeystoreR = crate::BitReader;
#[doc = "Field `AES` reader - AES core is available."]
pub type AesR = crate::BitReader;
#[doc = "Field `HASH` reader - HASH Core is available."]
pub type HashR = crate::BitReader;
#[doc = "Field `AES_128` reader - AES core supports 128-bit keys."]
pub type Aes128R = crate::BitReader;
#[doc = "Field `AES_256` reader - AES core supports 256-bit keys Note: If both AES-128 and AES-256 are set to one, the AES core supports 192-bit keys as well."]
pub type Aes256R = crate::BitReader;
#[doc = "Field `AES_GCM` reader - AES-GCM is available as a single operation."]
pub type AesGcmR = crate::BitReader;
#[doc = "Field `AES_CCM` reader - AES-CCM is available as a single operation."]
pub type AesCcmR = crate::BitReader;
#[doc = "Field `SHA_256` reader - The HASH core supports SHA-256."]
pub type Sha256R = crate::BitReader;
#[doc = "Field `AHBINTERFACE` reader - AHB interface is available If this bit is 0, the EIP-120t has a TCM interface."]
pub type AhbinterfaceR = crate::BitReader;
#[doc = "Field `TYPE` reader - This field is 0x01 for the TYPE1 device."]
pub type TypeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - KEY STORE is available."]
    #[inline(always)]
    pub fn keystore(&self) -> KeystoreR {
        KeystoreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES core is available."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HASH Core is available."]
    #[inline(always)]
    pub fn hash(&self) -> HashR {
        HashR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - AES core supports 128-bit keys."]
    #[inline(always)]
    pub fn aes_128(&self) -> Aes128R {
        Aes128R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AES core supports 256-bit keys Note: If both AES-128 and AES-256 are set to one, the AES core supports 192-bit keys as well."]
    #[inline(always)]
    pub fn aes_256(&self) -> Aes256R {
        Aes256R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AES-GCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_gcm(&self) -> AesGcmR {
        AesGcmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AES-CCM is available as a single operation."]
    #[inline(always)]
    pub fn aes_ccm(&self) -> AesCcmR {
        AesCcmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The HASH core supports SHA-256."]
    #[inline(always)]
    pub fn sha_256(&self) -> Sha256R {
        Sha256R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - AHB interface is available If this bit is 0, the EIP-120t has a TCM interface."]
    #[inline(always)]
    pub fn ahbinterface(&self) -> AhbinterfaceR {
        AhbinterfaceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - This field is 0x01 for the TYPE1 device."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Options register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_options::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlOptionsSpec;
impl crate::RegisterSpec for CtrlOptionsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_options::R`](R) reader structure"]
impl crate::Readable for CtrlOptionsSpec {}
#[doc = "`reset()` method sets CTRL_OPTIONS to value 0"]
impl crate::Resettable for CtrlOptionsSpec {
    const RESET_VALUE: u32 = 0;
}
