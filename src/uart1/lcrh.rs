#[doc = "Register `LCRH` reader"]
pub type R = crate::R<LcrhSpec>;
#[doc = "Register `LCRH` writer"]
pub type W = crate::W<LcrhSpec>;
#[doc = "Field `BRK` reader - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP2` reader - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
pub type Stp2R = crate::BitReader;
#[doc = "Field `STP2` writer - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLEN` reader - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
pub type WlenR = crate::FieldReader;
#[doc = "Field `WLEN` writer - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPS` reader - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
pub type SpsR = crate::BitReader;
#[doc = "Field `SPS` writer - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART send break 1: A low level is continually output on the UnTx signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods). 0: Normal use"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<LcrhSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - UART parity enable 1: Parity checking and generation is enabled. 0: Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<LcrhSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - UART even parity select 1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits. 0: Odd parity is performed, which checks for an odd number of 1s. This bit has no effect when parity is disabled by the PEN bit."]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<LcrhSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - UART two stop bits select 1: Two stop bits are transmitted at the end of a frame. The receive logic does not check for two stop bits being received. 0: One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn stp2(&mut self) -> Stp2W<LcrhSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bit 4 - UART enable FIFOs 1: The transmit and receive FIFObuffers are enabled (FIFOmode). 0: The FIFOs are disabled (Character mode). The FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn fen(&mut self) -> FenW<LcrhSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - UART word length The bits indicate the number of data bits transmitted or received in a frame as follows: 0x0: 5 bits (default) 0x1: 6 bits 0x2: 7 bits 0x3: 8 bits"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WlenW<LcrhSpec> {
        WlenW::new(self, 5)
    }
    #[doc = "Bit 7 - UART stick parity select When bits 1, 2, and 7 of UARTLCRH are set, the parity bit is transmitted and checked as a 0. When bits 1 and 7 are set and 2 is cleared, the parity bit is transmitted and checked as a 1. When this bit is cleared, stick parity is disabled."]
    #[inline(always)]
    pub fn sps(&mut self) -> SpsW<LcrhSpec> {
        SpsW::new(self, 7)
    }
}
#[doc = "UART line control The LCRH register is the line control register. Serial parameters such as data length, parity, and stop bit selection are implemented in this register. When updating the baud-rate divisor (IBRD and/or IFRD), the LCRH register must also be written. The write strobe for the baud-rate divisor registers is tied to the LCRH register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrhSpec;
impl crate::RegisterSpec for LcrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrh::R`](R) reader structure"]
impl crate::Readable for LcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`lcrh::W`](W) writer structure"]
impl crate::Writable for LcrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LcrhSpec {
    const RESET_VALUE: u32 = 0;
}
