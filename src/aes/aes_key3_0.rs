#[doc = "Register `AES_KEY3_0` writer"]
pub struct W(crate::W<AES_KEY3_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_KEY3_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AES_KEY3_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_KEY3_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_KEY3` writer - AES_KEY3\\[31:0\\]/AES_KEY2\\[159:128\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
pub type AES_KEY3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AES_KEY3_0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - AES_KEY3\\[31:0\\]/AES_KEY2\\[159:128\\]
For GCM: -\\[127:0\\]
- GHASH_H - The internally calculated GHASH key is stored in these registers. Only used for modes that use the GHASH function (GCM). -\\[255:128\\]
- This register is used to store intermediate values and is initialized with 0s when loading a new key. For CCM: -\\[255:0\\]
- This register is used to store intermediate values. For CBC-MAC: -\\[255:0\\]
- ZEROES - This register must remain 0."]
    #[inline(always)]
    pub fn aes_key3(&mut self) -> AES_KEY3_W<0> {
        AES_KEY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key3_0](index.html) module"]
pub struct AES_KEY3_0_SPEC;
impl crate::RegisterSpec for AES_KEY3_0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_key3_0::W](W) writer structure"]
impl crate::Writable for AES_KEY3_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AES_KEY3_0 to value 0"]
impl crate::Resettable for AES_KEY3_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
