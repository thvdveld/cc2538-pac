#[doc = "Register `FADDR` reader"]
pub struct R(crate::R<FADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FADDR` writer"]
pub struct W(crate::W<FADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FADDR_SPEC>;
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
impl From<crate::W<FADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADDR` reader - Bit number \\[16:9\\]
selects one of 256 pages for page erase. Bit number \\[8:7\\]
selects one of the 4 row in a given page Bit number \\[6:1\\]
selects one of the 64-bit wide locations in a give row. Bit number \\[0\\]
will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\]
will always be 0. - 128Kbytes --> Bits \\[16:15\\]
will always be 0. - 256Kbytes --> Bit \\[16\\]
will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
pub struct FADDR_R(crate::FieldReader<u32, u32>);
impl FADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        FADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR` writer - Bit number \\[16:9\\]
selects one of 256 pages for page erase. Bit number \\[8:7\\]
selects one of the 4 row in a given page Bit number \\[6:1\\]
selects one of the 64-bit wide locations in a give row. Bit number \\[0\\]
will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\]
will always be 0. - 128Kbytes --> Bits \\[16:15\\]
will always be 0. - 256Kbytes --> Bit \\[16\\]
will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
pub struct FADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Bit number \\[16:9\\]
selects one of 256 pages for page erase. Bit number \\[8:7\\]
selects one of the 4 row in a given page Bit number \\[6:1\\]
selects one of the 64-bit wide locations in a give row. Bit number \\[0\\]
will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\]
will always be 0. - 128Kbytes --> Bits \\[16:15\\]
will always be 0. - 256Kbytes --> Bit \\[16\\]
will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
    #[inline(always)]
    pub fn faddr(&self) -> FADDR_R {
        FADDR_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Bit number \\[16:9\\]
selects one of 256 pages for page erase. Bit number \\[8:7\\]
selects one of the 4 row in a given page Bit number \\[6:1\\]
selects one of the 64-bit wide locations in a give row. Bit number \\[0\\]
will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\]
will always be 0. - 128Kbytes --> Bits \\[16:15\\]
will always be 0. - 256Kbytes --> Bit \\[16\\]
will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
    #[inline(always)]
    pub fn faddr(&mut self) -> FADDR_W {
        FADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](index.html) module"]
pub struct FADDR_SPEC;
impl crate::RegisterSpec for FADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [faddr::R](R) reader structure"]
impl crate::Readable for FADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faddr::W](W) writer structure"]
impl crate::Writable for FADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
