#[doc = "Register `MTMOVF0` reader"]
pub struct R(crate::R<MTMOVF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMOVF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMOVF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMOVF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMOVF0` writer"]
pub struct W(crate::W<MTMOVF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMOVF0_SPEC>;
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
impl From<crate::W<MTMOVF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMOVF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTMOVF0` reader - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
pub type MTMOVF0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTMOVF0` writer - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
pub type MTMOVF0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMOVF0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
    #[inline(always)]
    pub fn mtmovf0(&self) -> MTMOVF0_R {
        MTMOVF0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
    #[inline(always)]
    pub fn mtmovf0(&mut self) -> MTMOVF0_W<0> {
        MTMOVF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer multiplexed overflow register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmovf0](index.html) module"]
pub struct MTMOVF0_SPEC;
impl crate::RegisterSpec for MTMOVF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmovf0::R](R) reader structure"]
impl crate::Readable for MTMOVF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmovf0::W](W) writer structure"]
impl crate::Writable for MTMOVF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMOVF0 to value 0"]
impl crate::Resettable for MTMOVF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
