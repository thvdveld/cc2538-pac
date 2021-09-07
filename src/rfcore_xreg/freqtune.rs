#[doc = "Register `FREQTUNE` reader"]
pub struct R(crate::R<FREQTUNE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQTUNE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQTUNE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQTUNE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQTUNE` writer"]
pub struct W(crate::W<FREQTUNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQTUNE_SPEC>;
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
impl From<crate::W<FREQTUNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQTUNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC32M_TUNE` reader - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
pub struct XOSC32M_TUNE_R(crate::FieldReader<u8, u8>);
impl XOSC32M_TUNE_R {
    pub(crate) fn new(bits: u8) -> Self {
        XOSC32M_TUNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC32M_TUNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC32M_TUNE` writer - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
pub struct XOSC32M_TUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC32M_TUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
    #[inline(always)]
    pub fn xosc32m_tune(&self) -> XOSC32M_TUNE_R {
        XOSC32M_TUNE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
    #[inline(always)]
    pub fn xosc32m_tune(&mut self) -> XOSC32M_TUNE_W {
        XOSC32M_TUNE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal oscillator frequency tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqtune](index.html) module"]
pub struct FREQTUNE_SPEC;
impl crate::RegisterSpec for FREQTUNE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqtune::R](R) reader structure"]
impl crate::Readable for FREQTUNE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqtune::W](W) writer structure"]
impl crate::Writable for FREQTUNE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQTUNE to value 0"]
impl crate::Resettable for FREQTUNE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
