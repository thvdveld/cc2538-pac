#[doc = "Register `WDCTL` reader"]
pub type R = crate::R<WdctlSpec>;
#[doc = "Register `WDCTL` writer"]
pub type W = crate::W<WdctlSpec>;
#[doc = "Field `INT` reader - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
pub type IntR = crate::FieldReader;
#[doc = "Field `INT` writer - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
pub type IntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EN` reader - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR` reader - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
pub type ClrR = crate::FieldReader;
#[doc = "Field `CLR` writer - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<WdctlSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<WdctlSpec> {
        EnW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<WdctlSpec> {
        ClrW::new(self, 4)
    }
}
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdctlSpec;
impl crate::RegisterSpec for WdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdctl::R`](R) reader structure"]
impl crate::Readable for WdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdctl::W`](W) writer structure"]
impl crate::Writable for WdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDCTL to value 0"]
impl crate::Resettable for WdctlSpec {
    const RESET_VALUE: u32 = 0;
}
