#[doc = "Register `FREQTUNE` reader"]
pub type R = crate::R<FreqtuneSpec>;
#[doc = "Register `FREQTUNE` writer"]
pub type W = crate::W<FreqtuneSpec>;
#[doc = "Field `XOSC32M_TUNE` reader - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
pub type Xosc32mTuneR = crate::FieldReader;
#[doc = "Field `XOSC32M_TUNE` writer - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
pub type Xosc32mTuneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
    #[inline(always)]
    pub fn xosc32m_tune(&self) -> Xosc32mTuneR {
        Xosc32mTuneR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Tune crystal oscillator The default setting 1111 leaves the XOSC untuned. Changing the setting from the default setting (1111) switches in extra capacitance to the oscillator, effectively lowering the XOSC frequency. Hence, a higher setting gives a higher frequency."]
    #[inline(always)]
    #[must_use]
    pub fn xosc32m_tune(&mut self) -> Xosc32mTuneW<FreqtuneSpec> {
        Xosc32mTuneW::new(self, 0)
    }
}
#[doc = "Crystal oscillator frequency tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqtune::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqtune::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqtuneSpec;
impl crate::RegisterSpec for FreqtuneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqtune::R`](R) reader structure"]
impl crate::Readable for FreqtuneSpec {}
#[doc = "`write(|w| ..)` method takes [`freqtune::W`](W) writer structure"]
impl crate::Writable for FreqtuneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQTUNE to value 0"]
impl crate::Resettable for FreqtuneSpec {
    const RESET_VALUE: u32 = 0;
}
