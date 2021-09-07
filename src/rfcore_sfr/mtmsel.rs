#[doc = "Register `MTMSEL` reader"]
pub struct R(crate::R<MTMSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMSEL` writer"]
pub struct W(crate::W<MTMSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMSEL_SPEC>;
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
impl From<crate::W<MTMSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTMOVFSEL` reader - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
pub struct MTMOVFSEL_R(crate::FieldReader<u8, u8>);
impl MTMOVFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MTMOVFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTMOVFSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTMOVFSEL` writer - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
pub struct MTMOVFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MTMOVFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `MTMSEL` reader - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
pub struct MTMSEL_R(crate::FieldReader<u8, u8>);
impl MTMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MTMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTMSEL` writer - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
pub struct MTMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MTMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
    #[inline(always)]
    pub fn mtmovfsel(&self) -> MTMOVFSEL_R {
        MTMOVFSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
    #[inline(always)]
    pub fn mtmsel(&self) -> MTMSEL_R {
        MTMSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
    #[inline(always)]
    pub fn mtmovfsel(&mut self) -> MTMOVFSEL_W {
        MTMOVFSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
    #[inline(always)]
    pub fn mtmsel(&mut self) -> MTMSEL_W {
        MTMSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer multiplex select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmsel](index.html) module"]
pub struct MTMSEL_SPEC;
impl crate::RegisterSpec for MTMSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmsel::R](R) reader structure"]
impl crate::Readable for MTMSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmsel::W](W) writer structure"]
impl crate::Writable for MTMSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMSEL to value 0"]
impl crate::Resettable for MTMSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
