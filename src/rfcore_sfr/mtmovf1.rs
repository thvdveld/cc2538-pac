#[doc = "Register `MTMOVF1` reader"]
pub struct R(crate::R<MTMOVF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMOVF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMOVF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMOVF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMOVF1` writer"]
pub struct W(crate::W<MTMOVF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMOVF1_SPEC>;
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
impl From<crate::W<MTMOVF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMOVF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTMOVF1` reader - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
pub type MTMOVF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTMOVF1` writer - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
pub type MTMOVF1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMOVF1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
    #[inline(always)]
    pub fn mtmovf1(&self) -> MTMOVF1_R {
        MTMOVF1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
    #[inline(always)]
    pub fn mtmovf1(&mut self) -> MTMOVF1_W<0> {
        MTMOVF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer multiplexed overflow register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmovf1](index.html) module"]
pub struct MTMOVF1_SPEC;
impl crate::RegisterSpec for MTMOVF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmovf1::R](R) reader structure"]
impl crate::Readable for MTMOVF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmovf1::W](W) writer structure"]
impl crate::Writable for MTMOVF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMOVF1 to value 0"]
impl crate::Resettable for MTMOVF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
