#[doc = "Register `PP` reader"]
pub type R = crate::R<PpSpec>;
#[doc = "Field `SIZE` reader - Timer size 0: Timer A and Timer B are 16 bits wide with 8-bit prescale. 1: Timer A and Timer B are 32 bits wide with 16-bit prescale."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `CHAIN` reader - Chain with other timers 0: Timer is not capable of chaining with previously numbered Timers. 1: Timer is capable of chaining with previously numbered timers."]
pub type ChainR = crate::BitReader;
#[doc = "Field `SYNCNT` reader - Synchronized start 0: Timer is not capable of synchronizing the count value with other timers. 1: Timer is capable of synchronizing the count value with other timers."]
pub type SyncntR = crate::BitReader;
#[doc = "Field `ALTCLK` reader - Alternate clock source 0: Timer is not capable of using an alternate clock. 1: Timer is capable of using an alternate clock."]
pub type AltclkR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Timer size 0: Timer A and Timer B are 16 bits wide with 8-bit prescale. 1: Timer A and Timer B are 32 bits wide with 16-bit prescale."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Chain with other timers 0: Timer is not capable of chaining with previously numbered Timers. 1: Timer is capable of chaining with previously numbered timers."]
    #[inline(always)]
    pub fn chain(&self) -> ChainR {
        ChainR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronized start 0: Timer is not capable of synchronizing the count value with other timers. 1: Timer is capable of synchronizing the count value with other timers."]
    #[inline(always)]
    pub fn syncnt(&self) -> SyncntR {
        SyncntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alternate clock source 0: Timer is not capable of using an alternate clock. 1: Timer is capable of using an alternate clock."]
    #[inline(always)]
    pub fn altclk(&self) -> AltclkR {
        AltclkR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpSpec;
impl crate::RegisterSpec for PpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PpSpec {}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PpSpec {
    const RESET_VALUE: u32 = 0;
}
