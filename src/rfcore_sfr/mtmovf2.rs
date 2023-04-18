#[doc = "Register `MTMOVF2` reader"]
pub struct R(crate::R<MTMOVF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMOVF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMOVF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMOVF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMOVF2` writer"]
pub struct W(crate::W<MTMOVF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMOVF2_SPEC>;
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
impl From<crate::W<MTMOVF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMOVF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTMOVF2` reader - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
pub type MTMOVF2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTMOVF2` writer - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
pub type MTMOVF2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMOVF2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
    #[inline(always)]
    pub fn mtmovf2(&self) -> MTMOVF2_R {
        MTMOVF2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mtmovf2(&mut self) -> MTMOVF2_W<0> {
        MTMOVF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer multiplexed overflow register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmovf2](index.html) module"]
pub struct MTMOVF2_SPEC;
impl crate::RegisterSpec for MTMOVF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmovf2::R](R) reader structure"]
impl crate::Readable for MTMOVF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmovf2::W](W) writer structure"]
impl crate::Writable for MTMOVF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTMOVF2 to value 0"]
impl crate::Resettable for MTMOVF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
