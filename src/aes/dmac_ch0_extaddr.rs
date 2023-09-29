#[doc = "Register `DMAC_CH0_EXTADDR` reader"]
pub type R = crate::R<DMAC_CH0_EXTADDR_SPEC>;
#[doc = "Register `DMAC_CH0_EXTADDR` writer"]
pub type W = crate::W<DMAC_CH0_EXTADDR_SPEC>;
#[doc = "Field `ADDR` reader - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<DMAC_CH0_EXTADDR_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel external address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch0_extaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch0_extaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_CH0_EXTADDR_SPEC;
impl crate::RegisterSpec for DMAC_CH0_EXTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ch0_extaddr::R`](R) reader structure"]
impl crate::Readable for DMAC_CH0_EXTADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_ch0_extaddr::W`](W) writer structure"]
impl crate::Writable for DMAC_CH0_EXTADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC_CH0_EXTADDR to value 0"]
impl crate::Resettable for DMAC_CH0_EXTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
