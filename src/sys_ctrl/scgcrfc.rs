#[doc = "Register `SCGCRFC` reader"]
pub struct R(crate::R<SCGCRFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCRFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCRFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCRFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCRFC` writer"]
pub struct W(crate::W<SCGCRFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCRFC_SPEC>;
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
impl From<crate::W<SCGCRFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCRFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC0` reader - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type RFC0_R = crate::BitReader<bool>;
#[doc = "Field `RFC0` writer - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type RFC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGCRFC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    pub fn rfc0(&self) -> RFC0_R {
        RFC0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rfc0(&mut self) -> RFC0_W<0> {
        RFC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcrfc](index.html) module"]
pub struct SCGCRFC_SPEC;
impl crate::RegisterSpec for SCGCRFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcrfc::R](R) reader structure"]
impl crate::Readable for SCGCRFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcrfc::W](W) writer structure"]
impl crate::Writable for SCGCRFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCRFC to value 0"]
impl crate::Resettable for SCGCRFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
