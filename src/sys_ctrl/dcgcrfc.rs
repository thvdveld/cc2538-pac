#[doc = "Register `DCGCRFC` reader"]
pub struct R(crate::R<DCGCRFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCRFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCRFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCRFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCRFC` writer"]
pub struct W(crate::W<DCGCRFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCRFC_SPEC>;
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
impl From<crate::W<DCGCRFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCRFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC0` reader - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type RFC0_R = crate::BitReader<bool>;
#[doc = "Field `RFC0` writer - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type RFC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCGCRFC_SPEC, bool, O>;
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
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcrfc](index.html) module"]
pub struct DCGCRFC_SPEC;
impl crate::RegisterSpec for DCGCRFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcrfc::R](R) reader structure"]
impl crate::Readable for DCGCRFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcrfc::W](W) writer structure"]
impl crate::Writable for DCGCRFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCRFC to value 0"]
impl crate::Resettable for DCGCRFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
