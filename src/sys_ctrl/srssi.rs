#[doc = "Register `SRSSI` reader"]
pub struct R(crate::R<SRSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSSI` writer"]
pub struct W(crate::W<SRSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSSI_SPEC>;
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
impl From<crate::W<SRSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI0` reader - 0: SSI0 module is not reset 1: SSI0 module is reset"]
pub type SSI0_R = crate::BitReader<bool>;
#[doc = "Field `SSI0` writer - 0: SSI0 module is not reset 1: SSI0 module is reset"]
pub type SSI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRSSI_SPEC, bool, O>;
#[doc = "Field `SSI1` reader - 0: SSI1 module is not reset 1: SSI1 module is reset"]
pub type SSI1_R = crate::BitReader<bool>;
#[doc = "Field `SSI1` writer - 0: SSI1 module is not reset 1: SSI1 module is reset"]
pub type SSI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRSSI_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: SSI0 module is not reset 1: SSI0 module is reset"]
    #[inline(always)]
    pub fn ssi0(&self) -> SSI0_R {
        SSI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: SSI1 module is not reset 1: SSI1 module is reset"]
    #[inline(always)]
    pub fn ssi1(&self) -> SSI1_R {
        SSI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: SSI0 module is not reset 1: SSI0 module is reset"]
    #[inline(always)]
    pub fn ssi0(&mut self) -> SSI0_W<0> {
        SSI0_W::new(self)
    }
    #[doc = "Bit 1 - 0: SSI1 module is not reset 1: SSI1 module is reset"]
    #[inline(always)]
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
#[doc = "This register controls the reset for SSI\\[1:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srssi](index.html) module"]
pub struct SRSSI_SPEC;
impl crate::RegisterSpec for SRSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srssi::R](R) reader structure"]
impl crate::Readable for SRSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srssi::W](W) writer structure"]
impl crate::Writable for SRSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSSI to value 0"]
impl crate::Resettable for SRSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
