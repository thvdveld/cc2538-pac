#[doc = "Register `FRMCTRL1` reader"]
pub struct R(crate::R<FRMCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRMCTRL1` writer"]
pub struct W(crate::W<FRMCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMCTRL1_SPEC>;
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
impl From<crate::W<FRMCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PENDING_OR` reader - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
pub type PENDING_OR_R = crate::BitReader<bool>;
#[doc = "Field `PENDING_OR` writer - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
pub type PENDING_OR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMCTRL1_SPEC, bool, O>;
#[doc = "Field `IGNORE_TX_UNDERF` reader - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
pub type IGNORE_TX_UNDERF_R = crate::BitReader<bool>;
#[doc = "Field `IGNORE_TX_UNDERF` writer - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
pub type IGNORE_TX_UNDERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMCTRL1_SPEC, bool, O>;
#[doc = "Field `SET_RXENMASK_ON_TX` reader - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
pub type SET_RXENMASK_ON_TX_R = crate::BitReader<bool>;
#[doc = "Field `SET_RXENMASK_ON_TX` writer - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
pub type SET_RXENMASK_ON_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMCTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    pub fn pending_or(&self) -> PENDING_OR_R {
        PENDING_OR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    pub fn ignore_tx_underf(&self) -> IGNORE_TX_UNDERF_R {
        IGNORE_TX_UNDERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    pub fn set_rxenmask_on_tx(&self) -> SET_RXENMASK_ON_TX_R {
        SET_RXENMASK_ON_TX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    pub fn pending_or(&mut self) -> PENDING_OR_W<2> {
        PENDING_OR_W::new(self)
    }
    #[doc = "Bit 1 - Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    pub fn ignore_tx_underf(&mut self) -> IGNORE_TX_UNDERF_W<1> {
        IGNORE_TX_UNDERF_W::new(self)
    }
    #[doc = "Bit 0 - Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    pub fn set_rxenmask_on_tx(&mut self) -> SET_RXENMASK_ON_TX_W<0> {
        SET_RXENMASK_ON_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame handling\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmctrl1](index.html) module"]
pub struct FRMCTRL1_SPEC;
impl crate::RegisterSpec for FRMCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frmctrl1::R](R) reader structure"]
impl crate::Readable for FRMCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frmctrl1::W](W) writer structure"]
impl crate::Writable for FRMCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRMCTRL1 to value 0"]
impl crate::Resettable for FRMCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
