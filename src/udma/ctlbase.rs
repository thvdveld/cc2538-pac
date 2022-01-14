#[doc = "Register `CTLBASE` reader"]
pub struct R(crate::R<CTLBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLBASE` writer"]
pub struct W(crate::W<CTLBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLBASE_SPEC>;
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
impl From<crate::W<CTLBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Channel control base address This field contains the pointer to the base address of the channel control table. The base address must be 1024-byte alligned."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlbase](index.html) module"]
pub struct CTLBASE_SPEC;
impl crate::RegisterSpec for CTLBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlbase::R](R) reader structure"]
impl crate::Readable for CTLBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlbase::W](W) writer structure"]
impl crate::Writable for CTLBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLBASE to value 0"]
impl crate::Resettable for CTLBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
