#[doc = "Register `TR0` reader"]
pub type R = crate::R<Tr0Spec>;
#[doc = "Register `TR0` writer"]
pub type W = crate::W<Tr0Spec>;
#[doc = "Field `ADCTM` reader - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
pub type AdctmR = crate::BitReader;
#[doc = "Field `ADCTM` writer - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
pub type AdctmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
    #[inline(always)]
    pub fn adctm(&self) -> AdctmR {
        AdctmR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
    #[inline(always)]
    #[must_use]
    pub fn adctm(&mut self) -> AdctmW<Tr0Spec> {
        AdctmW::new(self, 1)
    }
}
#[doc = "Test register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tr0Spec;
impl crate::RegisterSpec for Tr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr0::R`](R) reader structure"]
impl crate::Readable for Tr0Spec {}
#[doc = "`write(|w| ..)` method takes [`tr0::W`](W) writer structure"]
impl crate::Writable for Tr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR0 to value 0"]
impl crate::Resettable for Tr0Spec {
    const RESET_VALUE: u32 = 0;
}
