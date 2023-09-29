#[doc = "Register `AGCCTRL2` reader"]
pub type R = crate::R<AGCCTRL2_SPEC>;
#[doc = "Register `AGCCTRL2` writer"]
pub type W = crate::W<AGCCTRL2_SPEC>;
#[doc = "Field `LNA_CURRENT_OE` reader - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub type LNA_CURRENT_OE_R = crate::BitReader;
#[doc = "Field `LNA_CURRENT_OE` writer - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub type LNA_CURRENT_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LNA3_CURRENT` reader - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub type LNA3_CURRENT_R = crate::FieldReader;
#[doc = "Field `LNA3_CURRENT` writer - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub type LNA3_CURRENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LNA2_CURRENT` reader - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub type LNA2_CURRENT_R = crate::FieldReader;
#[doc = "Field `LNA2_CURRENT` writer - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub type LNA2_CURRENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LNA1_CURRENT` reader - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub type LNA1_CURRENT_R = crate::FieldReader;
#[doc = "Field `LNA1_CURRENT` writer - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub type LNA1_CURRENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
    #[must_use]
    pub fn lna_current_oe(&mut self) -> LNA_CURRENT_OE_W<AGCCTRL2_SPEC, 0> {
        LNA_CURRENT_OE_W::new(self)
    }
    #[doc = "Bits 1:2 - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    #[must_use]
    pub fn lna3_current(&mut self) -> LNA3_CURRENT_W<AGCCTRL2_SPEC, 1> {
        LNA3_CURRENT_W::new(self)
    }
    #[doc = "Bits 3:5 - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    #[must_use]
    pub fn lna2_current(&mut self) -> LNA2_CURRENT_W<AGCCTRL2_SPEC, 3> {
        LNA2_CURRENT_W::new(self)
    }
    #[doc = "Bits 6:7 - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    #[must_use]
    pub fn lna1_current(&mut self) -> LNA1_CURRENT_W<AGCCTRL2_SPEC, 6> {
        LNA1_CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AGC gain override\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGCCTRL2_SPEC;
impl crate::RegisterSpec for AGCCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agcctrl2::R`](R) reader structure"]
impl crate::Readable for AGCCTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agcctrl2::W`](W) writer structure"]
impl crate::Writable for AGCCTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGCCTRL2 to value 0"]
impl crate::Resettable for AGCCTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
