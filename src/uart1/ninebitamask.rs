#[doc = "Register `NINEBITAMASK` reader"]
pub type R = crate::R<NinebitamaskSpec>;
#[doc = "Register `NINEBITAMASK` writer"]
pub type W = crate::W<NinebitamaskSpec>;
#[doc = "Field `MASK` reader - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RANGE` reader - Self address range for 9-bit mode Writing to the RANGE field does not have any effect; reading it reflects the ANDed output of the ADDR field in the UART9BITADDR register and the MASK field."]
pub type RangeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Self address range for 9-bit mode Writing to the RANGE field does not have any effect; reading it reflects the ANDed output of the ADDR field in the UART9BITADDR register and the MASK field."]
    #[inline(always)]
    pub fn range(&self) -> RangeR {
        RangeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<NinebitamaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "UART 9-bit self address mask The NINEBITAMASK register is used to enable the address mask for 9-bit mode. The lower address bits are masked to create a range of address to be matched with the received address byte.\n\nYou can [`read`](crate::Reg::read) this register and get [`ninebitamask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ninebitamask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NinebitamaskSpec;
impl crate::RegisterSpec for NinebitamaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ninebitamask::R`](R) reader structure"]
impl crate::Readable for NinebitamaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ninebitamask::W`](W) writer structure"]
impl crate::Writable for NinebitamaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NINEBITAMASK to value 0"]
impl crate::Resettable for NinebitamaskSpec {
    const RESET_VALUE: u32 = 0;
}
