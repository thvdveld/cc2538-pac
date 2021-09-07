#[doc = "Register `I_MAP` reader"]
pub struct R(crate::R<I_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I_MAP` writer"]
pub struct W(crate::W<I_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I_MAP_SPEC>;
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
impl From<crate::W<I_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALTMAP` reader - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
pub struct ALTMAP_R(crate::FieldReader<bool, bool>);
impl ALTMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALTMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALTMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALTMAP` writer - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
pub struct ALTMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTMAP_W<'a> {
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
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    pub fn altmap(&self) -> ALTMAP_R {
        ALTMAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    pub fn altmap(&mut self) -> ALTMAP_W {
        ALTMAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register selects which interrupt map to be used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i_map](index.html) module"]
pub struct I_MAP_SPEC;
impl crate::RegisterSpec for I_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i_map::R](R) reader structure"]
impl crate::Readable for I_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i_map::W](W) writer structure"]
impl crate::Writable for I_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I_MAP to value 0"]
impl crate::Resettable for I_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
