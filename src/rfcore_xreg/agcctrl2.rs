#[doc = "Register `AGCCTRL2` reader"]
pub struct R(crate::R<AGCCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGCCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGCCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGCCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGCCTRL2` writer"]
pub struct W(crate::W<AGCCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGCCTRL2_SPEC>;
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
impl From<crate::W<AGCCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGCCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNA_CURRENT_OE` reader - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub type LNA_CURRENT_OE_R = crate::BitReader<bool>;
#[doc = "Field `LNA_CURRENT_OE` writer - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub type LNA_CURRENT_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AGCCTRL2_SPEC, bool, O>;
#[doc = "Field `LNA3_CURRENT` reader - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub type LNA3_CURRENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNA3_CURRENT` writer - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub type LNA3_CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AGCCTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LNA2_CURRENT` reader - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub type LNA2_CURRENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNA2_CURRENT` writer - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub type LNA2_CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AGCCTRL2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LNA1_CURRENT` reader - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub type LNA1_CURRENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNA1_CURRENT` writer - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub type LNA1_CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AGCCTRL2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    pub fn lna_current_oe(&self) -> LNA_CURRENT_OE_R {
        LNA_CURRENT_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    pub fn lna3_current(&self) -> LNA3_CURRENT_R {
        LNA3_CURRENT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    pub fn lna2_current(&self) -> LNA2_CURRENT_R {
        LNA2_CURRENT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    pub fn lna1_current(&self) -> LNA1_CURRENT_R {
        LNA1_CURRENT_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    pub fn lna_current_oe(&mut self) -> LNA_CURRENT_OE_W<0> {
        LNA_CURRENT_OE_W::new(self)
    }
    #[doc = "Bits 1:2 - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    pub fn lna3_current(&mut self) -> LNA3_CURRENT_W<1> {
        LNA3_CURRENT_W::new(self)
    }
    #[doc = "Bits 3:5 - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    pub fn lna2_current(&mut self) -> LNA2_CURRENT_W<3> {
        LNA2_CURRENT_W::new(self)
    }
    #[doc = "Bits 6:7 - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    pub fn lna1_current(&mut self) -> LNA1_CURRENT_W<6> {
        LNA1_CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGC gain override\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agcctrl2](index.html) module"]
pub struct AGCCTRL2_SPEC;
impl crate::RegisterSpec for AGCCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [agcctrl2::R](R) reader structure"]
impl crate::Readable for AGCCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agcctrl2::W](W) writer structure"]
impl crate::Writable for AGCCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AGCCTRL2 to value 0"]
impl crate::Resettable for AGCCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
