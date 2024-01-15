#[doc = "Register `TR0` reader"]
pub type R = crate::R<TR0_SPEC>;
#[doc = "Register `TR0` writer"]
pub type W = crate::W<TR0_SPEC>;
#[doc = "Field `ADCTM` reader - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
pub type ADCTM_R = crate::BitReader;
#[doc = "Field `ADCTM` writer - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
pub type ADCTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
    #[inline(always)]
    pub fn adctm(&self) -> ADCTM_R {
        ADCTM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
    #[inline(always)]
    #[must_use]
    pub fn adctm(&mut self) -> ADCTM_W<TR0_SPEC> {
        ADCTM_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Test register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR0_SPEC;
impl crate::RegisterSpec for TR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr0::R`](R) reader structure"]
impl crate::Readable for TR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr0::W`](W) writer structure"]
impl crate::Writable for TR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR0 to value 0"]
impl crate::Resettable for TR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
