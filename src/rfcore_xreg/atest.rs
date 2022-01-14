#[doc = "Register `ATEST` reader"]
pub struct R(crate::R<ATEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATEST` writer"]
pub struct W(crate::W<ATEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATEST_SPEC>;
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
impl From<crate::W<ATEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATEST_CTRL` reader - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
pub struct ATEST_CTRL_R(crate::FieldReader<u8, u8>);
impl ATEST_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ATEST_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATEST_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATEST_CTRL` writer - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
pub struct ATEST_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
    #[inline(always)]
    pub fn atest_ctrl(&self) -> ATEST_CTRL_R {
        ATEST_CTRL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
    #[inline(always)]
    pub fn atest_ctrl(&mut self) -> ATEST_CTRL_W {
        ATEST_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog test control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atest](index.html) module"]
pub struct ATEST_SPEC;
impl crate::RegisterSpec for ATEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atest::R](R) reader structure"]
impl crate::Readable for ATEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atest::W](W) writer structure"]
impl crate::Writable for ATEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATEST to value 0"]
impl crate::Resettable for ATEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
