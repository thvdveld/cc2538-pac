#[doc = "Register `NINEBITADDR` reader"]
pub type R = crate::R<NINEBITADDR_SPEC>;
#[doc = "Register `NINEBITADDR` writer"]
pub type W = crate::W<NINEBITADDR_SPEC>;
#[doc = "Field `ADDR` reader - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NINEBITEN` reader - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
pub type NINEBITEN_R = crate::BitReader;
#[doc = "Field `NINEBITEN` writer - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
pub type NINEBITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    pub fn ninebiten(&self) -> NINEBITEN_R {
        NINEBITEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<NINEBITADDR_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 15 - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ninebiten(&mut self) -> NINEBITEN_W<NINEBITADDR_SPEC> {
        NINEBITEN_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART 9-bit self address The NINEBITADDR register is used to write the specific address that should be matched with the receiving byte when the 9-bit address mask (NINEBITAMASK) is set to 0xFF. This register is used in conjunction with NINEBITAMASK to form a match for address-byte received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ninebitaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ninebitaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NINEBITADDR_SPEC;
impl crate::RegisterSpec for NINEBITADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ninebitaddr::R`](R) reader structure"]
impl crate::Readable for NINEBITADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ninebitaddr::W`](W) writer structure"]
impl crate::Writable for NINEBITADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NINEBITADDR to value 0"]
impl crate::Resettable for NINEBITADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
