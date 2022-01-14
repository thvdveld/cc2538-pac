#[doc = "Register `LCTL` reader"]
pub struct R(crate::R<LCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCTL` writer"]
pub struct W(crate::W<LCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCTL_SPEC>;
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
impl From<crate::W<LCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEN` reader - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
pub struct BLEN_R(crate::FieldReader<u8, u8>);
impl BLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN` writer - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
pub struct BLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `MASTER` reader - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
pub struct MASTER_R(crate::FieldReader<bool, bool>);
impl MASTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER` writer - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
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
    #[doc = "Bits 4:5 - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0 - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W {
        BLEN_W { w: self }
    }
    #[doc = "Bit 0 - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART LIN control The LCTL register is the configures the operation of the UART when in LIN mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lctl](index.html) module"]
pub struct LCTL_SPEC;
impl crate::RegisterSpec for LCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lctl::R](R) reader structure"]
impl crate::Readable for LCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lctl::W](W) writer structure"]
impl crate::Writable for LCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCTL to value 0"]
impl crate::Resettable for LCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
