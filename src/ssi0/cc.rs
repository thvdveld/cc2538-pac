#[doc = "Register `CC` reader"]
pub type R = crate::R<CcSpec>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CcSpec>;
#[doc = "Field `CS` reader - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
pub type CsR = crate::FieldReader;
#[doc = "Field `CS` writer - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
pub type CsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<CcSpec> {
        CsW::new(self, 0)
    }
}
#[doc = "SSI clock configuration The CC register controls the baud clock and system clocks sources for the SSI module. Note: If the PIOSC is used for the SSI baud clock, the system clock frequency must be at least 16 MHz in run mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CcSpec {
    const RESET_VALUE: u32 = 0;
}
