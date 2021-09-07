#[doc = "Register `MTM0` reader"]
pub struct R(crate::R<MTM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTM0` writer"]
pub struct W(crate::W<MTM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTM0_SPEC>;
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
impl From<crate::W<MTM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTM0` reader - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
pub struct MTM0_R(crate::FieldReader<u8, u8>);
impl MTM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        MTM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTM0` writer - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
pub struct MTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
    #[inline(always)]
    pub fn mtm0(&self) -> MTM0_R {
        MTM0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
    #[inline(always)]
    pub fn mtm0(&mut self) -> MTM0_W {
        MTM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer multiplexed register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtm0](index.html) module"]
pub struct MTM0_SPEC;
impl crate::RegisterSpec for MTM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtm0::R](R) reader structure"]
impl crate::Readable for MTM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtm0::W](W) writer structure"]
impl crate::Writable for MTM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTM0 to value 0"]
impl crate::Resettable for MTM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
