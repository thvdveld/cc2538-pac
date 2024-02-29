#[doc = "Register `FSCTRL` reader"]
pub type R = crate::R<FsctrlSpec>;
#[doc = "Register `FSCTRL` writer"]
pub type W = crate::W<FsctrlSpec>;
#[doc = "Field `LODIV_CURRENT` reader - Adjusts divider currents, except mixer and PA buffers"]
pub type LodivCurrentR = crate::FieldReader;
#[doc = "Field `LODIV_CURRENT` writer - Adjusts divider currents, except mixer and PA buffers"]
pub type LodivCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LODIV_BUF_CURRENT_RX` reader - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
pub type LodivBufCurrentRxR = crate::FieldReader;
#[doc = "Field `LODIV_BUF_CURRENT_RX` writer - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
pub type LodivBufCurrentRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LODIV_BUF_CURRENT_TX` reader - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
pub type LodivBufCurrentTxR = crate::FieldReader;
#[doc = "Field `LODIV_BUF_CURRENT_TX` writer - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
pub type LodivBufCurrentTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRE_CURRENT` reader - Prescaler current setting"]
pub type PreCurrentR = crate::FieldReader;
#[doc = "Field `PRE_CURRENT` writer - Prescaler current setting"]
pub type PreCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    pub fn lodiv_current(&self) -> LodivCurrentR {
        LodivCurrentR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    pub fn lodiv_buf_current_rx(&self) -> LodivBufCurrentRxR {
        LodivBufCurrentRxR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    pub fn lodiv_buf_current_tx(&self) -> LodivBufCurrentTxR {
        LodivBufCurrentTxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Prescaler current setting"]
    #[inline(always)]
    pub fn pre_current(&self) -> PreCurrentR {
        PreCurrentR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_current(&mut self) -> LodivCurrentW<FsctrlSpec> {
        LodivCurrentW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_buf_current_rx(&mut self) -> LodivBufCurrentRxW<FsctrlSpec> {
        LodivBufCurrentRxW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_buf_current_tx(&mut self) -> LodivBufCurrentTxW<FsctrlSpec> {
        LodivBufCurrentTxW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Prescaler current setting"]
    #[inline(always)]
    #[must_use]
    pub fn pre_current(&mut self) -> PreCurrentW<FsctrlSpec> {
        PreCurrentW::new(self, 6)
    }
}
#[doc = "Tune frequency synthesizer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsctrlSpec;
impl crate::RegisterSpec for FsctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsctrl::R`](R) reader structure"]
impl crate::Readable for FsctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fsctrl::W`](W) writer structure"]
impl crate::Writable for FsctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCTRL to value 0"]
impl crate::Resettable for FsctrlSpec {
    const RESET_VALUE: u32 = 0;
}
