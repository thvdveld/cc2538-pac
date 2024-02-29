#[doc = "Register `FRMCTRL1` reader"]
pub type R = crate::R<Frmctrl1Spec>;
#[doc = "Register `FRMCTRL1` writer"]
pub type W = crate::W<Frmctrl1Spec>;
#[doc = "Field `SET_RXENMASK_ON_TX` reader - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
pub type SetRxenmaskOnTxR = crate::BitReader;
#[doc = "Field `SET_RXENMASK_ON_TX` writer - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
pub type SetRxenmaskOnTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORE_TX_UNDERF` reader - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
pub type IgnoreTxUnderfR = crate::BitReader;
#[doc = "Field `IGNORE_TX_UNDERF` writer - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
pub type IgnoreTxUnderfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDING_OR` reader - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
pub type PendingOrR = crate::BitReader;
#[doc = "Field `PENDING_OR` writer - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
pub type PendingOrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    pub fn set_rxenmask_on_tx(&self) -> SetRxenmaskOnTxR {
        SetRxenmaskOnTxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    pub fn ignore_tx_underf(&self) -> IgnoreTxUnderfR {
        IgnoreTxUnderfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    pub fn pending_or(&self) -> PendingOrR {
        PendingOrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    #[must_use]
    pub fn set_rxenmask_on_tx(&mut self) -> SetRxenmaskOnTxW<Frmctrl1Spec> {
        SetRxenmaskOnTxW::new(self, 0)
    }
    #[doc = "Bit 1 - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    #[must_use]
    pub fn ignore_tx_underf(&mut self) -> IgnoreTxUnderfW<Frmctrl1Spec> {
        IgnoreTxUnderfW::new(self, 1)
    }
    #[doc = "Bit 2 - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    #[must_use]
    pub fn pending_or(&mut self) -> PendingOrW<Frmctrl1Spec> {
        PendingOrW::new(self, 2)
    }
}
#[doc = "Frame handling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frmctrl1Spec;
impl crate::RegisterSpec for Frmctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmctrl1::R`](R) reader structure"]
impl crate::Readable for Frmctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`frmctrl1::W`](W) writer structure"]
impl crate::Writable for Frmctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRMCTRL1 to value 0"]
impl crate::Resettable for Frmctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
