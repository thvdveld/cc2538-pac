#[doc = "Register `DMAC_CH1_EXTADDR` reader"]
pub type R = crate::R<DmacCh1ExtaddrSpec>;
#[doc = "Register `DMAC_CH1_EXTADDR` writer"]
pub type W = crate::W<DmacCh1ExtaddrSpec>;
#[doc = "Field `ADDR` reader - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<DmacCh1ExtaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel external address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac_ch1_extaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac_ch1_extaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCh1ExtaddrSpec;
impl crate::RegisterSpec for DmacCh1ExtaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ch1_extaddr::R`](R) reader structure"]
impl crate::Readable for DmacCh1ExtaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac_ch1_extaddr::W`](W) writer structure"]
impl crate::Writable for DmacCh1ExtaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_CH1_EXTADDR to value 0"]
impl crate::Resettable for DmacCh1ExtaddrSpec {
    const RESET_VALUE: u32 = 0;
}
