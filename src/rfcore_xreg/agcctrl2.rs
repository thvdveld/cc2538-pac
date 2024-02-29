#[doc = "Register `AGCCTRL2` reader"]
pub type R = crate::R<Agcctrl2Spec>;
#[doc = "Register `AGCCTRL2` writer"]
pub type W = crate::W<Agcctrl2Spec>;
#[doc = "Field `LNA_CURRENT_OE` reader - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub type LnaCurrentOeR = crate::BitReader;
#[doc = "Field `LNA_CURRENT_OE` writer - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub type LnaCurrentOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNA3_CURRENT` reader - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub type Lna3CurrentR = crate::FieldReader;
#[doc = "Field `LNA3_CURRENT` writer - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub type Lna3CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LNA2_CURRENT` reader - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub type Lna2CurrentR = crate::FieldReader;
#[doc = "Field `LNA2_CURRENT` writer - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub type Lna2CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LNA1_CURRENT` reader - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub type Lna1CurrentR = crate::FieldReader;
#[doc = "Field `LNA1_CURRENT` writer - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub type Lna1CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    pub fn lna_current_oe(&self) -> LnaCurrentOeR {
        LnaCurrentOeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    pub fn lna3_current(&self) -> Lna3CurrentR {
        Lna3CurrentR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    pub fn lna2_current(&self) -> Lna2CurrentR {
        Lna2CurrentR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    pub fn lna1_current(&self) -> Lna1CurrentR {
        Lna1CurrentR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    #[must_use]
    pub fn lna_current_oe(&mut self) -> LnaCurrentOeW<Agcctrl2Spec> {
        LnaCurrentOeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    #[must_use]
    pub fn lna3_current(&mut self) -> Lna3CurrentW<Agcctrl2Spec> {
        Lna3CurrentW::new(self, 1)
    }
    #[doc = "Bits 3:5 - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    #[must_use]
    pub fn lna2_current(&mut self) -> Lna2CurrentW<Agcctrl2Spec> {
        Lna2CurrentW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    #[must_use]
    pub fn lna1_current(&mut self) -> Lna1CurrentW<Agcctrl2Spec> {
        Lna1CurrentW::new(self, 6)
    }
}
#[doc = "AGC gain override\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agcctrl2Spec;
impl crate::RegisterSpec for Agcctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agcctrl2::R`](R) reader structure"]
impl crate::Readable for Agcctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`agcctrl2::W`](W) writer structure"]
impl crate::Writable for Agcctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGCCTRL2 to value 0"]
impl crate::Resettable for Agcctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
