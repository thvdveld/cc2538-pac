#[doc = "Register `CCACTRL1` reader"]
pub type R = crate::R<Ccactrl1Spec>;
#[doc = "Register `CCACTRL1` writer"]
pub type W = crate::W<Ccactrl1Spec>;
#[doc = "Field `CCA_HYST` reader - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub type CcaHystR = crate::FieldReader;
#[doc = "Field `CCA_HYST` writer - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub type CcaHystW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCA_MODE` reader - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
pub type CcaModeR = crate::FieldReader;
#[doc = "Field `CCA_MODE` writer - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
pub type CcaModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&self) -> CcaHystR {
        CcaHystR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&self) -> CcaModeR {
        CcaModeR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&mut self) -> CcaHystW<Ccactrl1Spec> {
        CcaHystW::new(self, 0)
    }
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&mut self) -> CcaModeW<Ccactrl1Spec> {
        CcaModeW::new(self, 3)
    }
}
#[doc = "Other CCA Options\n\nYou can [`read`](crate::Reg::read) this register and get [`ccactrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccactrl1Spec;
impl crate::RegisterSpec for Ccactrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccactrl1::R`](R) reader structure"]
impl crate::Readable for Ccactrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccactrl1::W`](W) writer structure"]
impl crate::Writable for Ccactrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCACTRL1 to value 0"]
impl crate::Resettable for Ccactrl1Spec {
    const RESET_VALUE: u32 = 0;
}
