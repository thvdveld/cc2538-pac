#[doc = "Register `CCACTRL1` reader"]
pub type R = crate::R<CCACTRL1_SPEC>;
#[doc = "Register `CCACTRL1` writer"]
pub type W = crate::W<CCACTRL1_SPEC>;
#[doc = "Field `CCA_HYST` reader - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub type CCA_HYST_R = crate::FieldReader;
#[doc = "Field `CCA_HYST` writer - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub type CCA_HYST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCA_MODE` reader - 00: CCA always set to 1 01: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
pub type CCA_MODE_R = crate::FieldReader;
#[doc = "Field `CCA_MODE` writer - 00: CCA always set to 1 01: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
pub type CCA_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&self) -> CCA_HYST_R {
        CCA_HYST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&self) -> CCA_MODE_R {
        CCA_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    #[must_use]
    pub fn cca_hyst(&mut self) -> CCA_HYST_W<CCACTRL1_SPEC, 0> {
        CCA_HYST_W::new(self)
    }
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI &lt; CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    #[must_use]
    pub fn cca_mode(&mut self) -> CCA_MODE_W<CCACTRL1_SPEC, 3> {
        CCA_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Other CCA Options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccactrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccactrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCACTRL1_SPEC;
impl crate::RegisterSpec for CCACTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccactrl1::R`](R) reader structure"]
impl crate::Readable for CCACTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccactrl1::W`](W) writer structure"]
impl crate::Writable for CCACTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCACTRL1 to value 0"]
impl crate::Resettable for CCACTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
