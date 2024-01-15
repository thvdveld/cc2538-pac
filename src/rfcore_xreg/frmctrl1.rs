#[doc = "Register `FRMCTRL1` reader"]
pub type R = crate::R<FRMCTRL1_SPEC>;
#[doc = "Register `FRMCTRL1` writer"]
pub type W = crate::W<FRMCTRL1_SPEC>;
#[doc = "Field `SET_RXENMASK_ON_TX` reader - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
pub type SET_RXENMASK_ON_TX_R = crate::BitReader;
#[doc = "Field `SET_RXENMASK_ON_TX` writer - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
pub type SET_RXENMASK_ON_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORE_TX_UNDERF` reader - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
pub type IGNORE_TX_UNDERF_R = crate::BitReader;
#[doc = "Field `IGNORE_TX_UNDERF` writer - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
pub type IGNORE_TX_UNDERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDING_OR` reader - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
pub type PENDING_OR_R = crate::BitReader;
#[doc = "Field `PENDING_OR` writer - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
pub type PENDING_OR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    pub fn set_rxenmask_on_tx(&self) -> SET_RXENMASK_ON_TX_R {
        SET_RXENMASK_ON_TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    pub fn ignore_tx_underf(&self) -> IGNORE_TX_UNDERF_R {
        IGNORE_TX_UNDERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    pub fn pending_or(&self) -> PENDING_OR_R {
        PENDING_OR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    #[must_use]
    pub fn set_rxenmask_on_tx(&mut self) -> SET_RXENMASK_ON_TX_W<FRMCTRL1_SPEC> {
        SET_RXENMASK_ON_TX_W::new(self, 0)
    }
    #[doc = "Bit 1 - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    #[must_use]
    pub fn ignore_tx_underf(&mut self) -> IGNORE_TX_UNDERF_W<FRMCTRL1_SPEC> {
        IGNORE_TX_UNDERF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    #[must_use]
    pub fn pending_or(&mut self) -> PENDING_OR_W<FRMCTRL1_SPEC> {
        PENDING_OR_W::new(self, 2)
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
#[doc = "Frame handling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRMCTRL1_SPEC;
impl crate::RegisterSpec for FRMCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmctrl1::R`](R) reader structure"]
impl crate::Readable for FRMCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frmctrl1::W`](W) writer structure"]
impl crate::Writable for FRMCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRMCTRL1 to value 0"]
impl crate::Resettable for FRMCTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
