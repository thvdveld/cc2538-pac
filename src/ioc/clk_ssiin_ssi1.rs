#[doc = "Register `CLK_SSIIN_SSI1` reader"]
pub struct R(crate::R<CLK_SSIIN_SSI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SSIIN_SSI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SSIIN_SSI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SSIIN_SSI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_SSIIN_SSI1` writer"]
pub struct W(crate::W<CLK_SSIIN_SSI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SSIIN_SSI1_SPEC>;
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
impl From<crate::W<CLK_SSIIN_SSI1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SSIIN_SSI1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
pub struct INPUT_SEL_R(crate::FieldReader<u8, u8>);
impl INPUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INPUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
pub struct INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
    #[inline(always)]
    pub fn input_sel(&mut self) -> INPUT_SEL_W {
        INPUT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ssiin_ssi1](index.html) module"]
pub struct CLK_SSIIN_SSI1_SPEC;
impl crate::RegisterSpec for CLK_SSIIN_SSI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ssiin_ssi1::R](R) reader structure"]
impl crate::Readable for CLK_SSIIN_SSI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ssiin_ssi1::W](W) writer structure"]
impl crate::Writable for CLK_SSIIN_SSI1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_SSIIN_SSI1 to value 0"]
impl crate::Resettable for CLK_SSIIN_SSI1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
