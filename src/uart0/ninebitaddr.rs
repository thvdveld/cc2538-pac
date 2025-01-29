#[doc = "Register `NINEBITADDR` reader"]
pub type R = crate::R<NinebitaddrSpec>;
#[doc = "Register `NINEBITADDR` writer"]
pub type W = crate::W<NinebitaddrSpec>;
#[doc = "Field `ADDR` reader - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NINEBITEN` reader - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
pub type NinebitenR = crate::BitReader;
#[doc = "Field `NINEBITEN` writer - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
pub type NinebitenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    pub fn ninebiten(&self) -> NinebitenR {
        NinebitenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<NinebitaddrSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 15 - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    pub fn ninebiten(&mut self) -> NinebitenW<NinebitaddrSpec> {
        NinebitenW::new(self, 15)
    }
}
#[doc = "UART 9-bit self address The NINEBITADDR register is used to write the specific address that should be matched with the receiving byte when the 9-bit address mask (NINEBITAMASK) is set to 0xFF. This register is used in conjunction with NINEBITAMASK to form a match for address-byte received.\n\nYou can [`read`](crate::Reg::read) this register and get [`ninebitaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ninebitaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NinebitaddrSpec;
impl crate::RegisterSpec for NinebitaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ninebitaddr::R`](R) reader structure"]
impl crate::Readable for NinebitaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ninebitaddr::W`](W) writer structure"]
impl crate::Writable for NinebitaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NINEBITADDR to value 0"]
impl crate::Resettable for NinebitaddrSpec {
    const RESET_VALUE: u32 = 0;
}
