#[doc = "Register `CTRL_PROT_EN` reader"]
pub type R = crate::R<CTRL_PROT_EN_SPEC>;
#[doc = "Register `CTRL_PROT_EN` writer"]
pub type W = crate::W<CTRL_PROT_EN_SPEC>;
#[doc = "Field `PROT_EN` reader - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
pub type PROT_EN_R = crate::BitReader;
#[doc = "Field `PROT_EN` writer - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
pub type PROT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    pub fn prot_en(&self) -> PROT_EN_R {
        PROT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    #[must_use]
    pub fn prot_en(&mut self) -> PROT_EN_W<CTRL_PROT_EN_SPEC> {
        PROT_EN_W::new(self, 0)
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
#[doc = "Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_prot_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_prot_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_PROT_EN_SPEC;
impl crate::RegisterSpec for CTRL_PROT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_prot_en::R`](R) reader structure"]
impl crate::Readable for CTRL_PROT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_prot_en::W`](W) writer structure"]
impl crate::Writable for CTRL_PROT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_PROT_EN to value 0"]
impl crate::Resettable for CTRL_PROT_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
