#[doc = "Register `CTLBASE` reader"]
pub type R = crate::R<CtlbaseSpec>;
#[doc = "Register `CTLBASE` writer"]
pub type W = crate::W<CtlbaseSpec>;
#[doc = "Field `ADDR` reader - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<CtlbaseSpec> {
        AddrW::new(self, 10)
    }
}
#[doc = "DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlbase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlbase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlbaseSpec;
impl crate::RegisterSpec for CtlbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlbase::R`](R) reader structure"]
impl crate::Readable for CtlbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`ctlbase::W`](W) writer structure"]
impl crate::Writable for CtlbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLBASE to value 0"]
impl crate::Resettable for CtlbaseSpec {
    const RESET_VALUE: u32 = 0;
}
