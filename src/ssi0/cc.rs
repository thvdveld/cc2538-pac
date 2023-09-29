#[doc = "Register `CC` reader"]
pub type R = crate::R<CC_SPEC>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CC_SPEC>;
#[doc = "Field `CS` reader - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
pub type CS_R = crate::FieldReader;
#[doc = "Field `CS` writer - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
pub type CS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<CC_SPEC, 0> {
        CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SSI clock configuration The CC register controls the baud clock and system clocks sources for the SSI module. Note: If the PIOSC is used for the SSI baud clock, the system clock frequency must be at least 16 MHz in run mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
