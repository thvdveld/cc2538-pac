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
pub struct AFSEL_R(crate::FieldReader<u8, u8>);
impl AFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL` writer - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
pub struct AFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn afsel(&mut self) -> AFSEL_W {
        AFSEL_W { w: self }
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
}
#[doc = "`reset()` method sets AFSEL to value 0"]
impl crate::Resettable for AFSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
