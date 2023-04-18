#[doc = "Register `MTM1` reader"]
pub struct R(crate::R<MTM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTM1` writer"]
pub struct W(crate::W<MTM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTM1_SPEC>;
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
impl From<crate::W<MTM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTM1` reader - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
pub type MTM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTM1` writer - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
pub type MTM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTM1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
    #[inline(always)]
    pub fn mtm1(&self) -> MTM1_R {
        MTM1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mtm1(&mut self) -> MTM1_W<0> {
        MTM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer multiplexed register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtm1](index.html) module"]
pub struct MTM1_SPEC;
impl crate::RegisterSpec for MTM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtm1::R](R) reader structure"]
impl crate::Readable for MTM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtm1::W](W) writer structure"]
impl crate::Writable for MTM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTM1 to value 0"]
impl crate::Resettable for MTM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
