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
#[doc = "Field `BRK` reader - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
pub type BRK_R = crate::BitReader<bool>;
#[doc = "Field `BRK` writer - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
pub type BRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
#[doc = "Field `PEN` reader - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
pub type PEN_R = crate::BitReader<bool>;
#[doc = "Field `PEN` writer - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
#[doc = "Field `EPS` reader - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
pub type EPS_R = crate::BitReader<bool>;
#[doc = "Field `EPS` writer - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
pub type EPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
#[doc = "Field `STP2` reader - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
pub type STP2_R = crate::BitReader<bool>;
#[doc = "Field `STP2` writer - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
pub type STP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
#[doc = "Field `FEN` reader - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
pub type FEN_R = crate::BitReader<bool>;
#[doc = "Field `FEN` writer - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
pub type FEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
#[doc = "Field `WLEN` reader - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
pub type WLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WLEN` writer - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
pub type WLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCRH_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPS` reader - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
pub type SPS_R = crate::BitReader<bool>;
#[doc = "Field `SPS` writer - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
pub type SPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W<0> {
        BRK_W::new(self)
    }
    #[doc = "Bit 1 - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W<1> {
        PEN_W::new(self)
    }
    #[doc = "Bit 2 - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W<2> {
        EPS_W::new(self)
    }
    #[doc = "Bit 3 - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn stp2(&mut self) -> STP2_W<3> {
        STP2_W::new(self)
    }
    #[doc = "Bit 4 - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W<4> {
        FEN_W::new(self)
    }
    #[doc = "Bits 5:6 - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W<5> {
        WLEN_W::new(self)
    }
    #[doc = "Bit 7 - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W<7> {
        SPS_W::new(self)
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
