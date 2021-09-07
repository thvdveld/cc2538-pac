#[doc = "Register `SHIFT` reader"]
pub struct R(crate::R<SHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFT` writer"]
pub struct W(crate::W<SHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFT_SPEC>;
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
impl From<crate::W<SHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUM_BITS_TO_SHIFT` reader - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub struct NUM_BITS_TO_SHIFT_R(crate::FieldReader<u8, u8>);
impl NUM_BITS_TO_SHIFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_BITS_TO_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_BITS_TO_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_BITS_TO_SHIFT` writer - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub struct NUM_BITS_TO_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_BITS_TO_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    pub fn num_bits_to_shift(&self) -> NUM_BITS_TO_SHIFT_R {
        NUM_BITS_TO_SHIFT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    pub fn num_bits_to_shift(&mut self) -> NUM_BITS_TO_SHIFT_W {
        NUM_BITS_TO_SHIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA bit shift value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shift](index.html) module"]
pub struct SHIFT_SPEC;
impl crate::RegisterSpec for SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shift::R](R) reader structure"]
impl crate::Readable for SHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shift::W](W) writer structure"]
impl crate::Writable for SHIFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFT to value 0"]
impl crate::Resettable for SHIFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
