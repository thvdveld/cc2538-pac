#[doc = "Register `GPIOCR` reader"]
pub struct R(crate::R<GPIOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOCR` writer"]
pub struct W(crate::W<GPIOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOCR_SPEC>;
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
impl From<crate::W<GPIOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
pub struct CR_R(crate::FieldReader<u8, u8>);
impl CR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR` writer - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiocr](index.html) module"]
pub struct GPIOCR_SPEC;
impl crate::RegisterSpec for GPIOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiocr::R](R) reader structure"]
impl crate::Readable for GPIOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiocr::W](W) writer structure"]
impl crate::Writable for GPIOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOCR to value 0"]
impl crate::Resettable for GPIOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
