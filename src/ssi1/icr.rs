#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RORIC` reader - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RoricR = crate::BitReader;
#[doc = "Field `RORIC` writer - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RoricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` reader - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RticR = crate::BitReader;
#[doc = "Field `RTIC` writer - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    pub fn roric(&self) -> RoricR {
        RoricR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    pub fn rtic(&self) -> RticR {
        RticR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    pub fn roric(&mut self) -> RoricW<IcrSpec> {
        RoricW::new(self, 0)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    pub fn rtic(&mut self) -> RticW<IcrSpec> {
        RticW::new(self, 1)
    }
}
#[doc = "The ICR register is the interrupt clear register. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
