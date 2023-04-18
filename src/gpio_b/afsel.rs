#[doc = "Register `AFSEL` reader"]
pub struct R(crate::R<AFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFSEL` writer"]
pub struct W(crate::W<AFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFSEL_SPEC>;
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
impl From<crate::W<AFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFSEL` reader - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
pub type AFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFSEL` writer - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
pub type AFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFSEL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
    #[inline(always)]
    pub fn afsel(&self) -> AFSEL_R {
        AFSEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
    #[inline(always)]
    #[must_use]
    pub fn afsel(&mut self) -> AFSEL_W<0> {
        AFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsel](index.html) module"]
pub struct AFSEL_SPEC;
impl crate::RegisterSpec for AFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afsel::R](R) reader structure"]
impl crate::Readable for AFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afsel::W](W) writer structure"]
impl crate::Writable for AFSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFSEL to value 0"]
impl crate::Resettable for AFSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
