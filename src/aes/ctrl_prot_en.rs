#[doc = "Register `CTRL_PROT_EN` reader"]
pub type R = crate::R<CtrlProtEnSpec>;
#[doc = "Register `CTRL_PROT_EN` writer"]
pub type W = crate::W<CtrlProtEnSpec>;
#[doc = "Field `PROT_EN` reader - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
pub type ProtEnR = crate::BitReader;
#[doc = "Field `PROT_EN` writer - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
pub type ProtEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    pub fn prot_en(&self) -> ProtEnR {
        ProtEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    pub fn prot_en(&mut self) -> ProtEnW<CtrlProtEnSpec> {
        ProtEnW::new(self, 0)
    }
}
#[doc = "Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_prot_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_prot_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlProtEnSpec;
impl crate::RegisterSpec for CtrlProtEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_prot_en::R`](R) reader structure"]
impl crate::Readable for CtrlProtEnSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_prot_en::W`](W) writer structure"]
impl crate::Writable for CtrlProtEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_PROT_EN to value 0"]
impl crate::Resettable for CtrlProtEnSpec {
    const RESET_VALUE: u32 = 0;
}
