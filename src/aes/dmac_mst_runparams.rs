#[doc = "Register `DMAC_MST_RUNPARAMS` reader"]
pub struct R(crate::R<DMAC_MST_RUNPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_MST_RUNPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_MST_RUNPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_MST_RUNPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_MST_RUNPARAMS` writer"]
pub struct W(crate::W<DMAC_MST_RUNPARAMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_MST_RUNPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMAC_MST_RUNPARAMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_MST_RUNPARAMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHB_MST1_BIGEND` reader - Endianess for the AHB master 0: Little endian 1: Big endian"]
pub type AHB_MST1_BIGEND_R = crate::BitReader<bool>;
#[doc = "Field `AHB_MST1_BIGEND` writer - Endianess for the AHB master 0: Little endian 1: Big endian"]
pub type AHB_MST1_BIGEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_MST_RUNPARAMS_SPEC, bool, O>;
#[doc = "Field `AHB_MST1_LOCK_EN` reader - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
pub type AHB_MST1_LOCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `AHB_MST1_LOCK_EN` writer - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
pub type AHB_MST1_LOCK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_MST_RUNPARAMS_SPEC, bool, O>;
#[doc = "Field `AHB_MST1_INCR_EN` reader - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
pub type AHB_MST1_INCR_EN_R = crate::BitReader<bool>;
#[doc = "Field `AHB_MST1_INCR_EN` writer - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
pub type AHB_MST1_INCR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_MST_RUNPARAMS_SPEC, bool, O>;
#[doc = "Field `AHB_MST1_IDLE_EN` reader - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
pub type AHB_MST1_IDLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `AHB_MST1_IDLE_EN` writer - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
pub type AHB_MST1_IDLE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_MST_RUNPARAMS_SPEC, bool, O>;
#[doc = "Field `AHB_MST1_BURST_SIZE` reader - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
pub type AHB_MST1_BURST_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHB_MST1_BURST_SIZE` writer - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
pub type AHB_MST1_BURST_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAC_MST_RUNPARAMS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 8 - Endianess for the AHB master 0: Little endian 1: Big endian"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AHB_MST1_BIGEND_R {
        AHB_MST1_BIGEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AHB_MST1_LOCK_EN_R {
        AHB_MST1_LOCK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AHB_MST1_INCR_EN_R {
        AHB_MST1_INCR_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AHB_MST1_IDLE_EN_R {
        AHB_MST1_IDLE_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AHB_MST1_BURST_SIZE_R {
        AHB_MST1_BURST_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Endianess for the AHB master 0: Little endian 1: Big endian"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&mut self) -> AHB_MST1_BIGEND_W<8> {
        AHB_MST1_BIGEND_W::new(self)
    }
    #[doc = "Bit 9 - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&mut self) -> AHB_MST1_LOCK_EN_W<9> {
        AHB_MST1_LOCK_EN_W::new(self)
    }
    #[doc = "Bit 10 - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&mut self) -> AHB_MST1_INCR_EN_W<10> {
        AHB_MST1_INCR_EN_W::new(self)
    }
    #[doc = "Bit 11 - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&mut self) -> AHB_MST1_IDLE_EN_W<11> {
        AHB_MST1_IDLE_EN_W::new(self)
    }
    #[doc = "Bits 12:15 - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&mut self) -> AHB_MST1_BURST_SIZE_W<12> {
        AHB_MST1_BURST_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_mst_runparams](index.html) module"]
pub struct DMAC_MST_RUNPARAMS_SPEC;
impl crate::RegisterSpec for DMAC_MST_RUNPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_mst_runparams::R](R) reader structure"]
impl crate::Readable for DMAC_MST_RUNPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_mst_runparams::W](W) writer structure"]
impl crate::Writable for DMAC_MST_RUNPARAMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_MST_RUNPARAMS to value 0"]
impl crate::Resettable for DMAC_MST_RUNPARAMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
