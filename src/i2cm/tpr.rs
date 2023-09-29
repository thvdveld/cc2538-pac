#[doc = "Register `TPR` reader"]
pub type R = crate::R<TPR_SPEC>;
#[doc = "Register `TPR` writer"]
pub type W = crate::W<TPR_SPEC>;
#[doc = "Field `TPR` reader - SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2 * (1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
pub type TPR_R = crate::FieldReader;
#[doc = "Field `TPR` writer - SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2 * (1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
pub type TPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2 * (1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    pub fn tpr(&self) -> TPR_R {
        TPR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2 * (1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    #[must_use]
    pub fn tpr(&mut self) -> TPR_W<TPR_SPEC, 0> {
        TPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C master timer period This register specifies the period of the SCL clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPR_SPEC;
impl crate::RegisterSpec for TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr::R`](R) reader structure"]
impl crate::Readable for TPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpr::W`](W) writer structure"]
impl crate::Writable for TPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
