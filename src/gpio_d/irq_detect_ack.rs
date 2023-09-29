#[doc = "Register `IRQ_DETECT_ACK` reader"]
pub type R = crate::R<IRQ_DETECT_ACK_SPEC>;
#[doc = "Register `IRQ_DETECT_ACK` writer"]
pub type W = crate::W<IRQ_DETECT_ACK_SPEC>;
#[doc = "Field `PAIACK0` reader - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK0_R = crate::BitReader;
#[doc = "Field `PAIACK0` writer - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAIACK1` reader - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK1_R = crate::BitReader;
#[doc = "Field `PAIACK1` writer - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAIACK2` reader - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK2_R = crate::BitReader;
#[doc = "Field `PAIACK2` writer - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAIACK3` reader - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK3_R = crate::BitReader;
#[doc = "Field `PAIACK3` writer - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAIACK4` reader - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK4_R = crate::BitReader;
#[doc = "Field `PAIACK4` writer - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAIACK5` reader - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK5_R = crate::BitReader;
#[doc = "Field `PAIACK5` writer - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAIACK6` reader - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK6_R = crate::BitReader;
#[doc = "Field `PAIACK6` writer - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAIACK7` reader - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK7_R = crate::BitReader;
#[doc = "Field `PAIACK7` writer - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PAIACK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK0` reader - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK0_R = crate::BitReader;
#[doc = "Field `PBIACK0` writer - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK1` reader - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK1_R = crate::BitReader;
#[doc = "Field `PBIACK1` writer - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK2` reader - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK2_R = crate::BitReader;
#[doc = "Field `PBIACK2` writer - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK3` reader - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK3_R = crate::BitReader;
#[doc = "Field `PBIACK3` writer - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK4` reader - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK4_R = crate::BitReader;
#[doc = "Field `PBIACK4` writer - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK5` reader - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK5_R = crate::BitReader;
#[doc = "Field `PBIACK5` writer - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK6` reader - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK6_R = crate::BitReader;
#[doc = "Field `PBIACK6` writer - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBIACK7` reader - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK7_R = crate::BitReader;
#[doc = "Field `PBIACK7` writer - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PBIACK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK0` reader - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK0_R = crate::BitReader;
#[doc = "Field `PCIACK0` writer - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK1` reader - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK1_R = crate::BitReader;
#[doc = "Field `PCIACK1` writer - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK2` reader - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK2_R = crate::BitReader;
#[doc = "Field `PCIACK2` writer - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK3` reader - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK3_R = crate::BitReader;
#[doc = "Field `PCIACK3` writer - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK4` reader - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK4_R = crate::BitReader;
#[doc = "Field `PCIACK4` writer - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK5` reader - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK5_R = crate::BitReader;
#[doc = "Field `PCIACK5` writer - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK6` reader - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK6_R = crate::BitReader;
#[doc = "Field `PCIACK6` writer - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCIACK7` reader - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK7_R = crate::BitReader;
#[doc = "Field `PCIACK7` writer - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PCIACK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK0` reader - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK0_R = crate::BitReader;
#[doc = "Field `PDIACK0` writer - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK1` reader - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
pub type PDIACK1_R = crate::BitReader;
#[doc = "Field `PDIACK1` writer - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
pub type PDIACK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK2` reader - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK2_R = crate::BitReader;
#[doc = "Field `PDIACK2` writer - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK3` reader - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK3_R = crate::BitReader;
#[doc = "Field `PDIACK3` writer - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK4` reader - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK4_R = crate::BitReader;
#[doc = "Field `PDIACK4` writer - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK5` reader - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK5_R = crate::BitReader;
#[doc = "Field `PDIACK5` writer - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK6` reader - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK6_R = crate::BitReader;
#[doc = "Field `PDIACK6` writer - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDIACK7` reader - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK7_R = crate::BitReader;
#[doc = "Field `PDIACK7` writer - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type PDIACK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack0(&self) -> PAIACK0_R {
        PAIACK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack1(&self) -> PAIACK1_R {
        PAIACK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack2(&self) -> PAIACK2_R {
        PAIACK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack3(&self) -> PAIACK3_R {
        PAIACK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack4(&self) -> PAIACK4_R {
        PAIACK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack5(&self) -> PAIACK5_R {
        PAIACK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack6(&self) -> PAIACK6_R {
        PAIACK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack7(&self) -> PAIACK7_R {
        PAIACK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack0(&self) -> PBIACK0_R {
        PBIACK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack1(&self) -> PBIACK1_R {
        PBIACK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack2(&self) -> PBIACK2_R {
        PBIACK2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack3(&self) -> PBIACK3_R {
        PBIACK3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack4(&self) -> PBIACK4_R {
        PBIACK4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack5(&self) -> PBIACK5_R {
        PBIACK5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack6(&self) -> PBIACK6_R {
        PBIACK6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack7(&self) -> PBIACK7_R {
        PBIACK7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack0(&self) -> PCIACK0_R {
        PCIACK0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack1(&self) -> PCIACK1_R {
        PCIACK1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack2(&self) -> PCIACK2_R {
        PCIACK2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack3(&self) -> PCIACK3_R {
        PCIACK3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack4(&self) -> PCIACK4_R {
        PCIACK4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack5(&self) -> PCIACK5_R {
        PCIACK5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack6(&self) -> PCIACK6_R {
        PCIACK6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack7(&self) -> PCIACK7_R {
        PCIACK7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack0(&self) -> PDIACK0_R {
        PDIACK0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    pub fn pdiack1(&self) -> PDIACK1_R {
        PDIACK1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack2(&self) -> PDIACK2_R {
        PDIACK2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack3(&self) -> PDIACK3_R {
        PDIACK3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack4(&self) -> PDIACK4_R {
        PDIACK4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack5(&self) -> PDIACK5_R {
        PDIACK5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack6(&self) -> PDIACK6_R {
        PDIACK6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack7(&self) -> PDIACK7_R {
        PDIACK7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack0(&mut self) -> PAIACK0_W<IRQ_DETECT_ACK_SPEC, 0> {
        PAIACK0_W::new(self)
    }
    #[doc = "Bit 1 - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack1(&mut self) -> PAIACK1_W<IRQ_DETECT_ACK_SPEC, 1> {
        PAIACK1_W::new(self)
    }
    #[doc = "Bit 2 - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack2(&mut self) -> PAIACK2_W<IRQ_DETECT_ACK_SPEC, 2> {
        PAIACK2_W::new(self)
    }
    #[doc = "Bit 3 - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack3(&mut self) -> PAIACK3_W<IRQ_DETECT_ACK_SPEC, 3> {
        PAIACK3_W::new(self)
    }
    #[doc = "Bit 4 - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack4(&mut self) -> PAIACK4_W<IRQ_DETECT_ACK_SPEC, 4> {
        PAIACK4_W::new(self)
    }
    #[doc = "Bit 5 - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack5(&mut self) -> PAIACK5_W<IRQ_DETECT_ACK_SPEC, 5> {
        PAIACK5_W::new(self)
    }
    #[doc = "Bit 6 - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack6(&mut self) -> PAIACK6_W<IRQ_DETECT_ACK_SPEC, 6> {
        PAIACK6_W::new(self)
    }
    #[doc = "Bit 7 - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn paiack7(&mut self) -> PAIACK7_W<IRQ_DETECT_ACK_SPEC, 7> {
        PAIACK7_W::new(self)
    }
    #[doc = "Bit 8 - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack0(&mut self) -> PBIACK0_W<IRQ_DETECT_ACK_SPEC, 8> {
        PBIACK0_W::new(self)
    }
    #[doc = "Bit 9 - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack1(&mut self) -> PBIACK1_W<IRQ_DETECT_ACK_SPEC, 9> {
        PBIACK1_W::new(self)
    }
    #[doc = "Bit 10 - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack2(&mut self) -> PBIACK2_W<IRQ_DETECT_ACK_SPEC, 10> {
        PBIACK2_W::new(self)
    }
    #[doc = "Bit 11 - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack3(&mut self) -> PBIACK3_W<IRQ_DETECT_ACK_SPEC, 11> {
        PBIACK3_W::new(self)
    }
    #[doc = "Bit 12 - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack4(&mut self) -> PBIACK4_W<IRQ_DETECT_ACK_SPEC, 12> {
        PBIACK4_W::new(self)
    }
    #[doc = "Bit 13 - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack5(&mut self) -> PBIACK5_W<IRQ_DETECT_ACK_SPEC, 13> {
        PBIACK5_W::new(self)
    }
    #[doc = "Bit 14 - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack6(&mut self) -> PBIACK6_W<IRQ_DETECT_ACK_SPEC, 14> {
        PBIACK6_W::new(self)
    }
    #[doc = "Bit 15 - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pbiack7(&mut self) -> PBIACK7_W<IRQ_DETECT_ACK_SPEC, 15> {
        PBIACK7_W::new(self)
    }
    #[doc = "Bit 16 - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack0(&mut self) -> PCIACK0_W<IRQ_DETECT_ACK_SPEC, 16> {
        PCIACK0_W::new(self)
    }
    #[doc = "Bit 17 - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack1(&mut self) -> PCIACK1_W<IRQ_DETECT_ACK_SPEC, 17> {
        PCIACK1_W::new(self)
    }
    #[doc = "Bit 18 - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack2(&mut self) -> PCIACK2_W<IRQ_DETECT_ACK_SPEC, 18> {
        PCIACK2_W::new(self)
    }
    #[doc = "Bit 19 - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack3(&mut self) -> PCIACK3_W<IRQ_DETECT_ACK_SPEC, 19> {
        PCIACK3_W::new(self)
    }
    #[doc = "Bit 20 - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack4(&mut self) -> PCIACK4_W<IRQ_DETECT_ACK_SPEC, 20> {
        PCIACK4_W::new(self)
    }
    #[doc = "Bit 21 - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack5(&mut self) -> PCIACK5_W<IRQ_DETECT_ACK_SPEC, 21> {
        PCIACK5_W::new(self)
    }
    #[doc = "Bit 22 - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack6(&mut self) -> PCIACK6_W<IRQ_DETECT_ACK_SPEC, 22> {
        PCIACK6_W::new(self)
    }
    #[doc = "Bit 23 - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pciack7(&mut self) -> PCIACK7_W<IRQ_DETECT_ACK_SPEC, 23> {
        PCIACK7_W::new(self)
    }
    #[doc = "Bit 24 - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack0(&mut self) -> PDIACK0_W<IRQ_DETECT_ACK_SPEC, 24> {
        PDIACK0_W::new(self)
    }
    #[doc = "Bit 25 - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack1(&mut self) -> PDIACK1_W<IRQ_DETECT_ACK_SPEC, 25> {
        PDIACK1_W::new(self)
    }
    #[doc = "Bit 26 - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack2(&mut self) -> PDIACK2_W<IRQ_DETECT_ACK_SPEC, 26> {
        PDIACK2_W::new(self)
    }
    #[doc = "Bit 27 - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack3(&mut self) -> PDIACK3_W<IRQ_DETECT_ACK_SPEC, 27> {
        PDIACK3_W::new(self)
    }
    #[doc = "Bit 28 - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack4(&mut self) -> PDIACK4_W<IRQ_DETECT_ACK_SPEC, 28> {
        PDIACK4_W::new(self)
    }
    #[doc = "Bit 29 - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack5(&mut self) -> PDIACK5_W<IRQ_DETECT_ACK_SPEC, 29> {
        PDIACK5_W::new(self)
    }
    #[doc = "Bit 30 - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack6(&mut self) -> PDIACK6_W<IRQ_DETECT_ACK_SPEC, 30> {
        PDIACK6_W::new(self)
    }
    #[doc = "Bit 31 - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    #[must_use]
    pub fn pdiack7(&mut self) -> PDIACK7_W<IRQ_DETECT_ACK_SPEC, 31> {
        PDIACK7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_detect_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_detect_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_DETECT_ACK_SPEC;
impl crate::RegisterSpec for IRQ_DETECT_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_detect_ack::R`](R) reader structure"]
impl crate::Readable for IRQ_DETECT_ACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_detect_ack::W`](W) writer structure"]
impl crate::Writable for IRQ_DETECT_ACK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_DETECT_ACK to value 0"]
impl crate::Resettable for IRQ_DETECT_ACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
