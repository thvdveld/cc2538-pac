#[doc = "Register `LCRH` reader"]
pub struct R(crate::R<LCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCRH` writer"]
pub struct W(crate::W<LCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCRH_SPEC>;
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
impl From<crate::W<LCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPS` reader - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
pub struct SPS_R(crate::FieldReader<bool, bool>);
impl SPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPS` writer - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
pub struct SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPS_W<'a> {
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
#[doc = "Field `WLEN` reader - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
pub struct WLEN_R(crate::FieldReader<u8, u8>);
impl WLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLEN` writer - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `FEN` reader - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
pub struct FEN_R(crate::FieldReader<bool, bool>);
impl FEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEN` writer - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
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
#[doc = "Field `STP2` reader - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
pub struct STP2_R(crate::FieldReader<bool, bool>);
impl STP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STP2` writer - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
pub struct STP2_W<'a> {
    w: &'a mut W,
}
impl<'a> STP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `EPS` reader - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
pub struct EPS_R(crate::FieldReader<bool, bool>);
impl EPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPS` writer - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
pub struct EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPS_W<'a> {
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
#[doc = "Field `PEN` reader - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
pub struct PEN_R(crate::FieldReader<bool, bool>);
impl PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN` writer - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
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
#[doc = "Field `BRK` reader - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
pub struct BRK_R(crate::FieldReader<bool, bool>);
impl BRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK` writer - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
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
    #[doc = "Bit 7 - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W {
        SPS_W { w: self }
    }
    #[doc = "Bits 5:6 - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
    #[doc = "Bit 4 - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Bit 3 - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn stp2(&mut self) -> STP2_W {
        STP2_W { w: self }
    }
    #[doc = "Bit 2 - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W { w: self }
    }
    #[doc = "Bit 1 - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 0 - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART line control The LCRH register is the line control register. Serial parameters such as data length, parity, and stop bit selection are implemented in this register. When updating the baud-rate divisor (IBRD and/or IFRD), the LCRH register must also be written. The write strobe for the baud-rate divisor registers is tied to the LCRH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcrh](index.html) module"]
pub struct LCRH_SPEC;
impl crate::RegisterSpec for LCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcrh::R](R) reader structure"]
impl crate::Readable for LCRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcrh::W](W) writer structure"]
impl crate::Writable for LCRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LCRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
