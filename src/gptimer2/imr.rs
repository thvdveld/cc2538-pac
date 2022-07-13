#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBMIM` reader - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBMIM_R = crate::BitReader<bool>;
#[doc = "Field `TBMIM` writer - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CBEIM` reader - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBEIM_R = crate::BitReader<bool>;
#[doc = "Field `CBEIM` writer - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CBMIM` reader - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBMIM_R = crate::BitReader<bool>;
#[doc = "Field `CBMIM` writer - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TBTOIM` reader - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBTOIM_R = crate::BitReader<bool>;
#[doc = "Field `TBTOIM` writer - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBTOIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TAMIM` reader - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TAMIM_R = crate::BitReader<bool>;
#[doc = "Field `TAMIM` writer - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TAMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CAEIM` reader - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAEIM_R = crate::BitReader<bool>;
#[doc = "Field `CAEIM` writer - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CAMIM` reader - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAMIM_R = crate::BitReader<bool>;
#[doc = "Field `CAMIM` writer - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TATOIM` reader - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TATOIM_R = crate::BitReader<bool>;
#[doc = "Field `TATOIM` writer - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TATOIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbmim(&self) -> TBMIM_R {
        TBMIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbeim(&self) -> CBEIM_R {
        CBEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbmim(&self) -> CBMIM_R {
        CBMIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbtoim(&self) -> TBTOIM_R {
        TBTOIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tamim(&self) -> TAMIM_R {
        TAMIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn caeim(&self) -> CAEIM_R {
        CAEIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn camim(&self) -> CAMIM_R {
        CAMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tatoim(&self) -> TATOIM_R {
        TATOIM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbmim(&mut self) -> TBMIM_W<11> {
        TBMIM_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbeim(&mut self) -> CBEIM_W<10> {
        CBEIM_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbmim(&mut self) -> CBMIM_W<9> {
        CBMIM_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbtoim(&mut self) -> TBTOIM_W<8> {
        TBTOIM_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tamim(&mut self) -> TAMIM_W<4> {
        TAMIM_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn caeim(&mut self) -> CAEIM_W<2> {
        CAEIM_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn camim(&mut self) -> CAMIM_W<1> {
        CAMIM_W::new(self)
    }
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tatoim(&mut self) -> TATOIM_W<0> {
        TATOIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
