#[doc = "Register `PWRDBG` reader"]
pub struct R(crate::R<PWRDBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRDBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRDBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRDBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRDBG` writer"]
pub struct W(crate::W<PWRDBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRDBG_SPEC>;
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
impl From<crate::W<PWRDBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRDBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_WARM_RESET` reader - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
pub struct FORCE_WARM_RESET_R(crate::FieldReader<bool, bool>);
impl FORCE_WARM_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_WARM_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_WARM_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_WARM_RESET` writer - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
pub struct FORCE_WARM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_WARM_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
    #[inline(always)]
    pub fn force_warm_reset(&self) -> FORCE_WARM_RESET_R {
        FORCE_WARM_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
    #[inline(always)]
    pub fn force_warm_reset(&mut self) -> FORCE_WARM_RESET_W {
        FORCE_WARM_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdbg](index.html) module"]
pub struct PWRDBG_SPEC;
impl crate::RegisterSpec for PWRDBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrdbg::R](R) reader structure"]
impl crate::Readable for PWRDBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrdbg::W](W) writer structure"]
impl crate::Writable for PWRDBG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRDBG to value 0"]
impl crate::Resettable for PWRDBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
