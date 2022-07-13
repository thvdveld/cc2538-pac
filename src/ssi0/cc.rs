#[doc = "Register `CC` reader"]
pub struct R(crate::R<CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC` writer"]
pub struct W(crate::W<CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS` reader - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
pub type CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS` writer - SSI baud and system clock source The following bits determine the clock source that generates the baud and system clocks for the SSI. bit0 (PIOSC): 1: The SSI baud clock is determined by the IO DIV setting in the system controller. 0: The SSI baud clock is determined by the SYS DIV setting in the system controller. bit1: Unused bit2: (DSEN) Only meaningful when the system is in deep sleep mode. This bit is a don't care when not in sleep mode. 1: The SSI system clock is running on the same clock as the baud clock, as per PIOSC setting above. 0: The SSI system clock is determined by the SYS DIV setting in the system controller."]
pub type CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_SPEC, u8, u8, 3, O>;
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
    pub fn cs(&mut self) -> CS_W<0> {
        CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI clock configuration The CC register controls the baud clock and system clocks sources for the SSI module. Note: If the PIOSC is used for the SSI baud clock, the system clock frequency must be at least 16 MHz in run mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc::R](R) reader structure"]
impl crate::Readable for CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
