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
pub type ALTMAP_R = crate::BitReader<bool>;
#[doc = "Field `ALTMAP` writer - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
pub type ALTMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, I_MAP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    pub fn altmap(&self) -> ALTMAP_R {
        ALTMAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    #[must_use]
    pub fn altmap(&mut self) -> ALTMAP_W<0> {
        ALTMAP_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I_MAP to value 0"]
impl crate::Resettable for I_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
