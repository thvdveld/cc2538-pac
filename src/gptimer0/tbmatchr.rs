#[doc = "Register `TBMATCHR` reader"]
pub struct R(crate::R<TBMATCHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBMATCHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBMATCHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBMATCHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBMATCHR` writer"]
pub struct W(crate::W<TBMATCHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBMATCHR_SPEC>;
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
impl From<crate::W<TBMATCHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBMATCHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBMR` reader - GPTM Timer B match register"]
pub type TBMR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TBMR` writer - GPTM Timer B match register"]
pub type TBMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBMATCHR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B match register"]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B match register"]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TBMR_W<0> {
        TBMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the match value. Bits \\[31:16\\]
are reserved in both cases.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbmatchr](index.html) module"]
pub struct TBMATCHR_SPEC;
impl crate::RegisterSpec for TBMATCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbmatchr::R](R) reader structure"]
impl crate::Readable for TBMATCHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbmatchr::W](W) writer structure"]
impl crate::Writable for TBMATCHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBMATCHR to value 0"]
impl crate::Resettable for TBMATCHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
