#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSEN` reader - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub struct CTSEN_R(crate::FieldReader<bool, bool>);
impl CTSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSEN` writer - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RTSEN` reader - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub struct RTSEN_R(crate::FieldReader<bool, bool>);
impl RTSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSEN` writer - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RXE` reader - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
pub struct RXE_R(crate::FieldReader<bool, bool>);
impl RXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXE` writer - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TXE` reader - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
pub struct TXE_R(crate::FieldReader<bool, bool>);
impl TXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE` writer - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `LBE` reader - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
pub struct LBE_R(crate::FieldReader<bool, bool>);
impl LBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBE` writer - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
pub struct LBE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `LIN` reader - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
pub struct LIN_R(crate::FieldReader<bool, bool>);
impl LIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIN` writer - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
pub struct LIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `HSE` reader - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
pub struct HSE_R(crate::FieldReader<bool, bool>);
impl HSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSE` writer - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
pub struct HSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EOT` reader - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
pub struct EOT_R(crate::FieldReader<bool, bool>);
impl EOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOT` writer - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SIRLP` reader - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
pub struct SIRLP_R(crate::FieldReader<bool, bool>);
impl SIRLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIRLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIRLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRLP` writer - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
pub struct SIRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRLP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SIREN` reader - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
pub struct SIREN_R(crate::FieldReader<bool, bool>);
impl SIREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIREN` writer - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
pub struct SIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UARTEN` reader - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
pub struct UARTEN_R(crate::FieldReader<bool, bool>);
impl UARTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UARTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UARTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTEN` writer - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
pub struct UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    pub fn lbe(&self) -> LBE_R {
        LBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    pub fn lin(&self) -> LIN_R {
        LIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    pub fn hse(&self) -> HSE_R {
        HSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    #[doc = "Bit 14 - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bit 9 - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 8 - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 7 - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    pub fn lbe(&mut self) -> LBE_W {
        LBE_W { w: self }
    }
    #[doc = "Bit 6 - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    pub fn lin(&mut self) -> LIN_W {
        LIN_W { w: self }
    }
    #[doc = "Bit 5 - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    pub fn hse(&mut self) -> HSE_W {
        HSE_W { w: self }
    }
    #[doc = "Bit 4 - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bit 2 - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&mut self) -> SIRLP_W {
        SIRLP_W { w: self }
    }
    #[doc = "Bit 1 - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    pub fn siren(&mut self) -> SIREN_W {
        SIREN_W { w: self }
    }
    #[doc = "Bit 0 - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W {
        UARTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART control The CTL register is the control register. All the bits are cleared on reset except for the transmit enable (TXE) and receive enable (RXE) bits, which are set. To enable the UART module, the UARTEN bit must be set. If software requires a configuration change in the module, the UARTEN bit must be cleared before the configuration changes are written. If the UART is disabled during a transmit or receive operation, the current transaction is completed before the UART stopping. Note: The UARTCTL register should not be changed while the UART is enabled or else the results are unpredictable. The following sequence is recommended for making changes to the UARTCTL register: 1. Disable the UART. 2. Wait for the end of transmission or reception of the current character. 3. Flush the transmit FIFO by clearing bit 4 (FEN) in the line control register (UARTLCRH). 4. Reprogram the control register. 5. Enable the UART.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
