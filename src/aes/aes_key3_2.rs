#[doc = "Register `AES_KEY3_2` writer"]
pub type W = crate::W<AES_KEY3_2_SPEC>;
#[doc = "Field `AES_KEY3` writer - AES_KEY3\\[95:64\\]/AES_KEY2\\[223:192\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
pub type AES_KEY3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - AES_KEY3\\[95:64\\]/AES_KEY2\\[223:192\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
    #[inline(always)]
    #[must_use]
    pub fn aes_key3(&mut self) -> AES_KEY3_W<AES_KEY3_2_SPEC, 0> {
        AES_KEY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key3_2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_KEY3_2_SPEC;
impl crate::RegisterSpec for AES_KEY3_2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_key3_2::W`](W) writer structure"]
impl crate::Writable for AES_KEY3_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_KEY3_2 to value 0"]
impl crate::Resettable for AES_KEY3_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
