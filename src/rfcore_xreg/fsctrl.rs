#[doc = "Register `FSCTRL` reader"]
pub type R = crate::R<FSCTRL_SPEC>;
#[doc = "Register `FSCTRL` writer"]
pub type W = crate::W<FSCTRL_SPEC>;
#[doc = "Field `LODIV_CURRENT` reader - Adjusts divider currents, except mixer and PA buffers"]
pub type LODIV_CURRENT_R = crate::FieldReader;
#[doc = "Field `LODIV_CURRENT` writer - Adjusts divider currents, except mixer and PA buffers"]
pub type LODIV_CURRENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LODIV_BUF_CURRENT_RX` reader - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
pub type LODIV_BUF_CURRENT_RX_R = crate::FieldReader;
#[doc = "Field `LODIV_BUF_CURRENT_RX` writer - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
pub type LODIV_BUF_CURRENT_RX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LODIV_BUF_CURRENT_TX` reader - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
pub type LODIV_BUF_CURRENT_TX_R = crate::FieldReader;
#[doc = "Field `LODIV_BUF_CURRENT_TX` writer - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
pub type LODIV_BUF_CURRENT_TX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PRE_CURRENT` reader - Prescaler current setting"]
pub type PRE_CURRENT_R = crate::FieldReader;
#[doc = "Field `PRE_CURRENT` writer - Prescaler current setting"]
pub type PRE_CURRENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    pub fn lodiv_current(&self) -> LODIV_CURRENT_R {
        LODIV_CURRENT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    pub fn lodiv_buf_current_rx(&self) -> LODIV_BUF_CURRENT_RX_R {
        LODIV_BUF_CURRENT_RX_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    pub fn lodiv_buf_current_tx(&self) -> LODIV_BUF_CURRENT_TX_R {
        LODIV_BUF_CURRENT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Prescaler current setting"]
    #[inline(always)]
    pub fn pre_current(&self) -> PRE_CURRENT_R {
        PRE_CURRENT_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_current(&mut self) -> LODIV_CURRENT_W<FSCTRL_SPEC, 0> {
        LODIV_CURRENT_W::new(self)
    }
    #[doc = "Bits 2:3 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_buf_current_rx(&mut self) -> LODIV_BUF_CURRENT_RX_W<FSCTRL_SPEC, 2> {
        LODIV_BUF_CURRENT_RX_W::new(self)
    }
    #[doc = "Bits 4:5 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_buf_current_tx(&mut self) -> LODIV_BUF_CURRENT_TX_W<FSCTRL_SPEC, 4> {
        LODIV_BUF_CURRENT_TX_W::new(self)
    }
    #[doc = "Bits 6:7 - Prescaler current setting"]
    #[inline(always)]
    #[must_use]
    pub fn pre_current(&mut self) -> PRE_CURRENT_W<FSCTRL_SPEC, 6> {
        PRE_CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tune frequency synthesizer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSCTRL_SPEC;
impl crate::RegisterSpec for FSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsctrl::R`](R) reader structure"]
impl crate::Readable for FSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsctrl::W`](W) writer structure"]
impl crate::Writable for FSCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSCTRL to value 0"]
impl crate::Resettable for FSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
