#[doc = "Register `PMCTL` reader"]
pub struct R(crate::R<PMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCTL` writer"]
pub struct W(crate::W<PMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCTL_SPEC>;
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
impl From<crate::W<PMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - 00: No action 01: PM1 10: PM2 11: PM3"]
pub type PM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PM` writer - 00: No action 01: PM1 10: PM2 11: PM3"]
pub type PM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMCTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - 00: No action 01: PM1 10: PM2 11: PM3"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: No action 01: PM1 10: PM2 11: PM3"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<0> {
        PM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmctl](index.html) module"]
pub struct PMCTL_SPEC;
impl crate::RegisterSpec for PMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmctl::R](R) reader structure"]
impl crate::Readable for PMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmctl::W](W) writer structure"]
impl crate::Writable for PMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMCTL to value 0"]
impl crate::Resettable for PMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
