#[doc = "Register `STCC` reader"]
pub struct R(crate::R<STCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCC` writer"]
pub struct W(crate::W<STCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCC_SPEC>;
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
impl From<crate::W<STCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
pub type PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCC_SPEC, u8, u8, 3, O>;
#[doc = "Field `PORT` reader - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
pub type PORT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCC_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W<0> {
        PIN_W::new(self)
    }
    #[doc = "Bits 3:5 - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W<3> {
        PORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Timer Capture control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcc](index.html) module"]
pub struct STCC_SPEC;
impl crate::RegisterSpec for STCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcc::R](R) reader structure"]
impl crate::Readable for STCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcc::W](W) writer structure"]
impl crate::Writable for STCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STCC to value 0"]
impl crate::Resettable for STCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
