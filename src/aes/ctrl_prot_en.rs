#[doc = "Register `CTRL_PROT_EN` reader"]
pub struct R(crate::R<CTRL_PROT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_PROT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_PROT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_PROT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_PROT_EN` writer"]
pub struct W(crate::W<CTRL_PROT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_PROT_EN_SPEC>;
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
impl From<crate::W<CTRL_PROT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_PROT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT_EN` reader - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
pub struct PROT_EN_R(crate::FieldReader<bool, bool>);
impl PROT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT_EN` writer - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
pub struct PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    pub fn prot_en(&self) -> PROT_EN_R {
        PROT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is cleared to 0, m_h_prot\\[1\\]
on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\]
signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    pub fn prot_en(&mut self) -> PROT_EN_W {
        PROT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_prot_en](index.html) module"]
pub struct CTRL_PROT_EN_SPEC;
impl crate::RegisterSpec for CTRL_PROT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_prot_en::R](R) reader structure"]
impl crate::Readable for CTRL_PROT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_prot_en::W](W) writer structure"]
impl crate::Writable for CTRL_PROT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_PROT_EN to value 0"]
impl crate::Resettable for CTRL_PROT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
