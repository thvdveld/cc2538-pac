#[doc = "Register `IRQ_DETECT_ACK` reader"]
pub type R = crate::R<IrqDetectAckSpec>;
#[doc = "Register `IRQ_DETECT_ACK` writer"]
pub type W = crate::W<IrqDetectAckSpec>;
#[doc = "Field `PAIACK0` reader - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack0R = crate::BitReader;
#[doc = "Field `PAIACK0` writer - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIACK1` reader - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack1R = crate::BitReader;
#[doc = "Field `PAIACK1` writer - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIACK2` reader - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack2R = crate::BitReader;
#[doc = "Field `PAIACK2` writer - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIACK3` reader - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack3R = crate::BitReader;
#[doc = "Field `PAIACK3` writer - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIACK4` reader - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack4R = crate::BitReader;
#[doc = "Field `PAIACK4` writer - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIACK5` reader - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack5R = crate::BitReader;
#[doc = "Field `PAIACK5` writer - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIACK6` reader - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack6R = crate::BitReader;
#[doc = "Field `PAIACK6` writer - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIACK7` reader - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack7R = crate::BitReader;
#[doc = "Field `PAIACK7` writer - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Paiack7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK0` reader - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack0R = crate::BitReader;
#[doc = "Field `PBIACK0` writer - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK1` reader - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack1R = crate::BitReader;
#[doc = "Field `PBIACK1` writer - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK2` reader - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack2R = crate::BitReader;
#[doc = "Field `PBIACK2` writer - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK3` reader - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack3R = crate::BitReader;
#[doc = "Field `PBIACK3` writer - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK4` reader - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack4R = crate::BitReader;
#[doc = "Field `PBIACK4` writer - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK5` reader - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack5R = crate::BitReader;
#[doc = "Field `PBIACK5` writer - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK6` reader - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack6R = crate::BitReader;
#[doc = "Field `PBIACK6` writer - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIACK7` reader - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack7R = crate::BitReader;
#[doc = "Field `PBIACK7` writer - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pbiack7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK0` reader - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack0R = crate::BitReader;
#[doc = "Field `PCIACK0` writer - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK1` reader - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack1R = crate::BitReader;
#[doc = "Field `PCIACK1` writer - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK2` reader - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack2R = crate::BitReader;
#[doc = "Field `PCIACK2` writer - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK3` reader - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack3R = crate::BitReader;
#[doc = "Field `PCIACK3` writer - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK4` reader - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack4R = crate::BitReader;
#[doc = "Field `PCIACK4` writer - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK5` reader - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack5R = crate::BitReader;
#[doc = "Field `PCIACK5` writer - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK6` reader - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack6R = crate::BitReader;
#[doc = "Field `PCIACK6` writer - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIACK7` reader - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack7R = crate::BitReader;
#[doc = "Field `PCIACK7` writer - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pciack7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK0` reader - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack0R = crate::BitReader;
#[doc = "Field `PDIACK0` writer - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK1` reader - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
pub type Pdiack1R = crate::BitReader;
#[doc = "Field `PDIACK1` writer - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
pub type Pdiack1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK2` reader - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack2R = crate::BitReader;
#[doc = "Field `PDIACK2` writer - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK3` reader - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack3R = crate::BitReader;
#[doc = "Field `PDIACK3` writer - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK4` reader - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack4R = crate::BitReader;
#[doc = "Field `PDIACK4` writer - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK5` reader - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack5R = crate::BitReader;
#[doc = "Field `PDIACK5` writer - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK6` reader - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack6R = crate::BitReader;
#[doc = "Field `PDIACK6` writer - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIACK7` reader - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack7R = crate::BitReader;
#[doc = "Field `PDIACK7` writer - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub type Pdiack7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack0(&self) -> Paiack0R {
        Paiack0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack1(&self) -> Paiack1R {
        Paiack1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack2(&self) -> Paiack2R {
        Paiack2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack3(&self) -> Paiack3R {
        Paiack3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack4(&self) -> Paiack4R {
        Paiack4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack5(&self) -> Paiack5R {
        Paiack5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack6(&self) -> Paiack6R {
        Paiack6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack7(&self) -> Paiack7R {
        Paiack7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack0(&self) -> Pbiack0R {
        Pbiack0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack1(&self) -> Pbiack1R {
        Pbiack1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack2(&self) -> Pbiack2R {
        Pbiack2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack3(&self) -> Pbiack3R {
        Pbiack3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack4(&self) -> Pbiack4R {
        Pbiack4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack5(&self) -> Pbiack5R {
        Pbiack5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack6(&self) -> Pbiack6R {
        Pbiack6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack7(&self) -> Pbiack7R {
        Pbiack7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack0(&self) -> Pciack0R {
        Pciack0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack1(&self) -> Pciack1R {
        Pciack1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack2(&self) -> Pciack2R {
        Pciack2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack3(&self) -> Pciack3R {
        Pciack3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack4(&self) -> Pciack4R {
        Pciack4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack5(&self) -> Pciack5R {
        Pciack5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack6(&self) -> Pciack6R {
        Pciack6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack7(&self) -> Pciack7R {
        Pciack7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack0(&self) -> Pdiack0R {
        Pdiack0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    pub fn pdiack1(&self) -> Pdiack1R {
        Pdiack1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack2(&self) -> Pdiack2R {
        Pdiack2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack3(&self) -> Pdiack3R {
        Pdiack3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack4(&self) -> Pdiack4R {
        Pdiack4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack5(&self) -> Pdiack5R {
        Pdiack5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack6(&self) -> Pdiack6R {
        Pdiack6R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack7(&self) -> Pdiack7R {
        Pdiack7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack0(&mut self) -> Paiack0W<IrqDetectAckSpec> {
        Paiack0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack1(&mut self) -> Paiack1W<IrqDetectAckSpec> {
        Paiack1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack2(&mut self) -> Paiack2W<IrqDetectAckSpec> {
        Paiack2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack3(&mut self) -> Paiack3W<IrqDetectAckSpec> {
        Paiack3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack4(&mut self) -> Paiack4W<IrqDetectAckSpec> {
        Paiack4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack5(&mut self) -> Paiack5W<IrqDetectAckSpec> {
        Paiack5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack6(&mut self) -> Paiack6W<IrqDetectAckSpec> {
        Paiack6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack7(&mut self) -> Paiack7W<IrqDetectAckSpec> {
        Paiack7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack0(&mut self) -> Pbiack0W<IrqDetectAckSpec> {
        Pbiack0W::new(self, 8)
    }
    #[doc = "Bit 9 - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack1(&mut self) -> Pbiack1W<IrqDetectAckSpec> {
        Pbiack1W::new(self, 9)
    }
    #[doc = "Bit 10 - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack2(&mut self) -> Pbiack2W<IrqDetectAckSpec> {
        Pbiack2W::new(self, 10)
    }
    #[doc = "Bit 11 - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack3(&mut self) -> Pbiack3W<IrqDetectAckSpec> {
        Pbiack3W::new(self, 11)
    }
    #[doc = "Bit 12 - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack4(&mut self) -> Pbiack4W<IrqDetectAckSpec> {
        Pbiack4W::new(self, 12)
    }
    #[doc = "Bit 13 - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack5(&mut self) -> Pbiack5W<IrqDetectAckSpec> {
        Pbiack5W::new(self, 13)
    }
    #[doc = "Bit 14 - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack6(&mut self) -> Pbiack6W<IrqDetectAckSpec> {
        Pbiack6W::new(self, 14)
    }
    #[doc = "Bit 15 - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack7(&mut self) -> Pbiack7W<IrqDetectAckSpec> {
        Pbiack7W::new(self, 15)
    }
    #[doc = "Bit 16 - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack0(&mut self) -> Pciack0W<IrqDetectAckSpec> {
        Pciack0W::new(self, 16)
    }
    #[doc = "Bit 17 - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack1(&mut self) -> Pciack1W<IrqDetectAckSpec> {
        Pciack1W::new(self, 17)
    }
    #[doc = "Bit 18 - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack2(&mut self) -> Pciack2W<IrqDetectAckSpec> {
        Pciack2W::new(self, 18)
    }
    #[doc = "Bit 19 - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack3(&mut self) -> Pciack3W<IrqDetectAckSpec> {
        Pciack3W::new(self, 19)
    }
    #[doc = "Bit 20 - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack4(&mut self) -> Pciack4W<IrqDetectAckSpec> {
        Pciack4W::new(self, 20)
    }
    #[doc = "Bit 21 - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack5(&mut self) -> Pciack5W<IrqDetectAckSpec> {
        Pciack5W::new(self, 21)
    }
    #[doc = "Bit 22 - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack6(&mut self) -> Pciack6W<IrqDetectAckSpec> {
        Pciack6W::new(self, 22)
    }
    #[doc = "Bit 23 - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack7(&mut self) -> Pciack7W<IrqDetectAckSpec> {
        Pciack7W::new(self, 23)
    }
    #[doc = "Bit 24 - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack0(&mut self) -> Pdiack0W<IrqDetectAckSpec> {
        Pdiack0W::new(self, 24)
    }
    #[doc = "Bit 25 - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    pub fn pdiack1(&mut self) -> Pdiack1W<IrqDetectAckSpec> {
        Pdiack1W::new(self, 25)
    }
    #[doc = "Bit 26 - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack2(&mut self) -> Pdiack2W<IrqDetectAckSpec> {
        Pdiack2W::new(self, 26)
    }
    #[doc = "Bit 27 - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack3(&mut self) -> Pdiack3W<IrqDetectAckSpec> {
        Pdiack3W::new(self, 27)
    }
    #[doc = "Bit 28 - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack4(&mut self) -> Pdiack4W<IrqDetectAckSpec> {
        Pdiack4W::new(self, 28)
    }
    #[doc = "Bit 29 - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack5(&mut self) -> Pdiack5W<IrqDetectAckSpec> {
        Pdiack5W::new(self, 29)
    }
    #[doc = "Bit 30 - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack6(&mut self) -> Pdiack6W<IrqDetectAckSpec> {
        Pdiack6W::new(self, 30)
    }
    #[doc = "Bit 31 - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack7(&mut self) -> Pdiack7W<IrqDetectAckSpec> {
        Pdiack7W::new(self, 31)
    }
}
#[doc = "If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_detect_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_detect_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqDetectAckSpec;
impl crate::RegisterSpec for IrqDetectAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_detect_ack::R`](R) reader structure"]
impl crate::Readable for IrqDetectAckSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_detect_ack::W`](W) writer structure"]
impl crate::Writable for IrqDetectAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_DETECT_ACK to value 0"]
impl crate::Resettable for IrqDetectAckSpec {
    const RESET_VALUE: u32 = 0;
}
