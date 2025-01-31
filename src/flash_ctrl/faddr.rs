#[doc = "Register `FADDR` reader"]
pub type R = crate::R<FaddrSpec>;
#[doc = "Register `FADDR` writer"]
pub type W = crate::W<FaddrSpec>;
#[doc = "Field `FADDR` reader - Bit number \\[16:9\\]
selects one of 256 pages for page erase. Bit number \\[8:7\\]
selects one of the 4 row in a given page Bit number \\[6:1\\]
selects one of the 64-bit wide locations in a give row. Bit number \\[0\\]
will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\]
will always be 0. - 128Kbytes --> Bits \\[16:15\\]
will always be 0. - 256Kbytes --> Bit \\[16\\]
will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
pub type FaddrR = crate::FieldReader<u32>;
#[doc = "Field `FADDR` writer - Bit number \\[16:9\\]
selects one of 256 pages for page erase. Bit number \\[8:7\\]
selects one of the 4 row in a given page Bit number \\[6:1\\]
selects one of the 64-bit wide locations in a give row. Bit number \\[0\\]
will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\]
will always be 0. - 128Kbytes --> Bits \\[16:15\\]
will always be 0. - 256Kbytes --> Bit \\[16\\]
will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
pub type FaddrW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
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
    pub fn faddr(&self) -> FaddrR {
        FaddrR::new(self.bits & 0x0001_ffff)
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
    pub fn faddr(&mut self) -> FaddrW<FaddrSpec> {
        FaddrW::new(self, 0)
    }
}
#[doc = "Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information.\n\nYou can [`read`](crate::Reg::read) this register and get [`faddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaddrSpec;
impl crate::RegisterSpec for FaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faddr::R`](R) reader structure"]
impl crate::Readable for FaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`faddr::W`](W) writer structure"]
impl crate::Writable for FaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FaddrSpec {
    const RESET_VALUE: u32 = 0;
}
