#[doc = "Register `USEBURSTSET` reader"]
pub struct R(crate::R<USEBURSTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USEBURSTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USEBURSTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USEBURSTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USEBURSTSET` writer"]
pub struct W(crate::W<USEBURSTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USEBURSTSET_SPEC>;
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
impl From<crate::W<USEBURSTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USEBURSTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET` reader - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
pub struct SET_R(crate::FieldReader<u32, u32>);
impl SET_R {
    pub(crate) fn new(bits: u32) -> Self {
        SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET` writer - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits |= value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\]
bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\]
bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [useburstset](index.html) module"]
pub struct USEBURSTSET_SPEC;
impl crate::RegisterSpec for USEBURSTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [useburstset::R](R) reader structure"]
impl crate::Readable for USEBURSTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [useburstset::W](W) writer structure"]
impl crate::Writable for USEBURSTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USEBURSTSET to value 0"]
impl crate::Resettable for USEBURSTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
