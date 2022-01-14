#[doc = "Register `RXCTRL` reader"]
pub struct R(crate::R<RXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCTRL` writer"]
pub struct W(crate::W<RXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCTRL_SPEC>;
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
impl From<crate::W<RXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GBIAS_LNA2_REF` reader - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub struct GBIAS_LNA2_REF_R(crate::FieldReader<u8, u8>);
impl GBIAS_LNA2_REF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GBIAS_LNA2_REF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GBIAS_LNA2_REF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GBIAS_LNA2_REF` writer - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub struct GBIAS_LNA2_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> GBIAS_LNA2_REF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `GBIAS_LNA_REF` reader - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub struct GBIAS_LNA_REF_R(crate::FieldReader<u8, u8>);
impl GBIAS_LNA_REF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GBIAS_LNA_REF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GBIAS_LNA_REF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GBIAS_LNA_REF` writer - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub struct GBIAS_LNA_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> GBIAS_LNA_REF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `MIX_CURRENT` reader - Control of the output current from the receiver mixers The current increases with increasing setting set."]
pub struct MIX_CURRENT_R(crate::FieldReader<u8, u8>);
impl MIX_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIX_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIX_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIX_CURRENT` writer - Control of the output current from the receiver mixers The current increases with increasing setting set."]
pub struct MIX_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> MIX_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna2_ref(&self) -> GBIAS_LNA2_REF_R {
        GBIAS_LNA2_REF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna_ref(&self) -> GBIAS_LNA_REF_R {
        GBIAS_LNA_REF_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    pub fn mix_current(&self) -> MIX_CURRENT_R {
        MIX_CURRENT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna2_ref(&mut self) -> GBIAS_LNA2_REF_W {
        GBIAS_LNA2_REF_W { w: self }
    }
    #[doc = "Bits 2:3 - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna_ref(&mut self) -> GBIAS_LNA_REF_W {
        GBIAS_LNA_REF_W { w: self }
    }
    #[doc = "Bits 0:1 - Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    pub fn mix_current(&mut self) -> MIX_CURRENT_W {
        MIX_CURRENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tune receive section\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxctrl](index.html) module"]
pub struct RXCTRL_SPEC;
impl crate::RegisterSpec for RXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxctrl::R](R) reader structure"]
impl crate::Readable for RXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxctrl::W](W) writer structure"]
impl crate::Writable for RXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCTRL to value 0"]
impl crate::Resettable for RXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
