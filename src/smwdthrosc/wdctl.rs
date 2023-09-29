#[doc = "Register `WDCTL` reader"]
pub type R = crate::R<WDCTL_SPEC>;
#[doc = "Register `WDCTL` writer"]
pub type W = crate::W<WDCTL_SPEC>;
#[doc = "Field `INT` reader - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
pub type INT_R = crate::FieldReader;
#[doc = "Field `INT` writer - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
pub type INT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EN` reader - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR` reader - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
pub type CLR_R = crate::FieldReader;
#[doc = "Field `CLR` writer - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
pub type CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer interval select These bits select the timer interval as follows: 00: Twdt x 32768 01: Twdt x 8192 10: Twdt x 512 11: Twdt x 64 Writing these bits when EN = 1 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<WDCTL_SPEC, 0> {
        INT_W::new(self)
    }
    #[doc = "Bit 3 - Enable timer When 1 is written to this bit the timer is enabled and starts incrementing. The interval setting specified by INT\\[1:0\\]
is used. Writing 0 to this bit have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<WDCTL_SPEC, 3> {
        EN_W::new(self)
    }
    #[doc = "Bits 4:7 - Clear timer When 0xA followed by 0x5 is written to these bits, the timer is loaded with 0x0000. Note that 0x5 must be written within one watchdog clock period Twdt after 0xA was written for the clearing to take effect (ensured). If 0x5 is written between Twdt and 2Twdt after 0xA was written, the clearing may take effect, but there is no guarantee. If 0x5 is written > 2Twdt after 0xA was written, the timer will not be cleared. If a value other than 0x5 is written after 0xA has been written, the clear sequence is aborted. If 0xA is written, this starts a new clear sequence. Writing to these bits when EN = 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<WDCTL_SPEC, 4> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDCTL_SPEC;
impl crate::RegisterSpec for WDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdctl::R`](R) reader structure"]
impl crate::Readable for WDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdctl::W`](W) writer structure"]
impl crate::Writable for WDCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDCTL to value 0"]
impl crate::Resettable for WDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
