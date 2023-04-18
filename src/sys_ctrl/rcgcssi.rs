#[doc = "Register `RCGCSSI` reader"]
pub struct R(crate::R<RCGCSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCSSI` writer"]
pub struct W(crate::W<RCGCSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCSSI_SPEC>;
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
impl From<crate::W<RCGCSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI0` reader - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
pub type SSI0_R = crate::BitReader<bool>;
#[doc = "Field `SSI0` writer - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
pub type SSI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCGCSSI_SPEC, bool, O>;
#[doc = "Field `SSI1` reader - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
pub type SSI1_R = crate::BitReader<bool>;
#[doc = "Field `SSI1` writer - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
pub type SSI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCGCSSI_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    pub fn ssi0(&self) -> SSI0_R {
        SSI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    pub fn ssi1(&self) -> SSI1_R {
        SSI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ssi0(&mut self) -> SSI0_W<0> {
        SSI0_W::new(self)
    }
    #[doc = "Bit 1 - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ssi1(&mut self) -> SSI1_W<1> {
        SSI1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcssi](index.html) module"]
pub struct RCGCSSI_SPEC;
impl crate::RegisterSpec for RCGCSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcssi::R](R) reader structure"]
impl crate::Readable for RCGCSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcssi::W](W) writer structure"]
impl crate::Writable for RCGCSSI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCSSI to value 0"]
impl crate::Resettable for RCGCSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
