#[doc = "Register `SRSEC` reader"]
pub struct R(crate::R<SRSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSEC` writer"]
pub struct W(crate::W<SRSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSEC_SPEC>;
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
impl From<crate::W<SRSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES` reader - 0: AES module is not reset 1: AES module is reset"]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - 0: AES module is not reset 1: AES module is reset"]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRSEC_SPEC, bool, O>;
#[doc = "Field `PKA` reader - 0: PKA module is not reset 1: PKA module is reset"]
pub type PKA_R = crate::BitReader<bool>;
#[doc = "Field `PKA` writer - 0: PKA module is not reset 1: PKA module is reset"]
pub type PKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRSEC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - 0: AES module is not reset 1: AES module is reset"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 0: PKA module is not reset 1: PKA module is reset"]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 0: AES module is not reset 1: AES module is reset"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W<1> {
        AES_W::new(self)
    }
    #[doc = "Bit 0 - 0: PKA module is not reset 1: PKA module is reset"]
    #[inline(always)]
    pub fn pka(&mut self) -> PKA_W<0> {
        PKA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the reset for the security module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsec](index.html) module"]
pub struct SRSEC_SPEC;
impl crate::RegisterSpec for SRSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srsec::R](R) reader structure"]
impl crate::Readable for SRSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srsec::W](W) writer structure"]
impl crate::Writable for SRSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSEC to value 0"]
impl crate::Resettable for SRSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
