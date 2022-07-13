#[doc = "Register `TBILR` reader"]
pub struct R(crate::R<TBILR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBILR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBILR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBILR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBILR` writer"]
pub struct W(crate::W<TBILR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBILR_SPEC>;
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
impl From<crate::W<TBILR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBILR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBILR` reader - GPTM B interval load register"]
pub type TBILR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TBILR` writer - GPTM B interval load register"]
pub type TBILR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBILR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - GPTM B interval load register"]
    #[inline(always)]
    pub fn tbilr(&self) -> TBILR_R {
        TBILR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM B interval load register"]
    #[inline(always)]
    pub fn tbilr(&mut self) -> TBILR_W<0> {
        TBILR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbilr](index.html) module"]
pub struct TBILR_SPEC;
impl crate::RegisterSpec for TBILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbilr::R](R) reader structure"]
impl crate::Readable for TBILR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbilr::W](W) writer structure"]
impl crate::Writable for TBILR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBILR to value 0"]
impl crate::Resettable for TBILR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
