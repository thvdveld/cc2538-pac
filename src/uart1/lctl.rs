#[doc = "Register `LCTL` reader"]
pub type R = crate::R<LctlSpec>;
#[doc = "Register `LCTL` writer"]
pub type W = crate::W<LctlSpec>;
#[doc = "Field `MASTER` reader - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
pub type MasterR = crate::BitReader;
#[doc = "Field `MASTER` writer - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN` reader - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
pub type BlenR = crate::FieldReader;
#[doc = "Field `BLEN` writer - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
pub type BlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    pub fn blen(&self) -> BlenR {
        BlenR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MasterW<LctlSpec> {
        MasterW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BlenW<LctlSpec> {
        BlenW::new(self, 4)
    }
}
#[doc = "UART LIN control The LCTL register is the configures the operation of the UART when in LIN mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctlSpec;
impl crate::RegisterSpec for LctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lctl::R`](R) reader structure"]
impl crate::Readable for LctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lctl::W`](W) writer structure"]
impl crate::Writable for LctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCTL to value 0"]
impl crate::Resettable for LctlSpec {
    const RESET_VALUE: u32 = 0;
}
