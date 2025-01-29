#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stat: Stat,
    cfg: Cfg,
    ctlbase: Ctlbase,
    altbase: Altbase,
    waitstat: Waitstat,
    swreq: Swreq,
    useburstset: Useburstset,
    useburstclr: Useburstclr,
    reqmaskset: Reqmaskset,
    reqmaskclr: Reqmaskclr,
    enaset: Enaset,
    enaclr: Enaclr,
    altset: Altset,
    altclr: Altclr,
    prioset: Prioset,
    prioclr: Prioclr,
    _reserved16: [u8; 0x0c],
    errclr: Errclr,
    _reserved17: [u8; 0x04b0],
    chasgn: Chasgn,
    chis: Chis,
    _reserved19: [u8; 0x08],
    chmap0: Chmap0,
    chmap1: Chmap1,
    chmap2: Chmap2,
    chmap3: Chmap3,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x04 - DMA configuration The CFG register controls the configuration of the uDMA controller."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state."]
    #[inline(always)]
    pub const fn ctlbase(&self) -> &Ctlbase {
        &self.ctlbase
    }
    #[doc = "0x0c - DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state."]
    #[inline(always)]
    pub const fn altbase(&self) -> &Altbase {
        &self.altbase
    }
    #[doc = "0x10 - DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state."]
    #[inline(always)]
    pub const fn waitstat(&self) -> &Waitstat {
        &self.waitstat
    }
    #[doc = "0x14 - DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel."]
    #[inline(always)]
    pub const fn swreq(&self) -> &Swreq {
        &self.swreq
    }
    #[doc = "0x18 - DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\]
bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\]
bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model."]
    #[inline(always)]
    pub const fn useburstset(&self) -> &Useburstset {
        &self.useburstset
    }
    #[doc = "0x1c - DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the USEBURSTSET register."]
    #[inline(always)]
    pub const fn useburstclr(&self) -> &Useburstclr {
        &self.useburstclr
    }
    #[doc = "0x20 - DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers."]
    #[inline(always)]
    pub const fn reqmaskset(&self) -> &Reqmaskset {
        &self.reqmaskset
    }
    #[doc = "0x24 - DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register."]
    #[inline(always)]
    pub const fn reqmaskclr(&self) -> &Reqmaskclr {
        &self.reqmaskclr
    }
    #[doc = "0x28 - DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers."]
    #[inline(always)]
    pub const fn enaset(&self) -> &Enaset {
        &self.enaset
    }
    #[doc = "0x2c - DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ENASET register."]
    #[inline(always)]
    pub const fn enaclr(&self) -> &Enaclr {
        &self.enaclr
    }
    #[doc = "0x30 - DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel."]
    #[inline(always)]
    pub const fn altset(&self) -> &Altset {
        &self.altset
    }
    #[doc = "0x34 - DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ALTSET register."]
    #[inline(always)]
    pub const fn altclr(&self) -> &Altclr {
        &self.altclr
    }
    #[doc = "0x38 - DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask."]
    #[inline(always)]
    pub const fn prioset(&self) -> &Prioset {
        &self.prioset
    }
    #[doc = "0x3c - DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register."]
    #[inline(always)]
    pub const fn prioclr(&self) -> &Prioclr {
        &self.prioclr
    }
    #[doc = "0x4c - DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected."]
    #[inline(always)]
    pub const fn errclr(&self) -> &Errclr {
        &self.errclr
    }
    #[doc = "0x500 - DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\""]
    #[inline(always)]
    pub const fn chasgn(&self) -> &Chasgn {
        &self.chasgn
    }
    #[doc = "0x504 - DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1."]
    #[inline(always)]
    pub const fn chis(&self) -> &Chis {
        &self.chis
    }
    #[doc = "0x510 - DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    #[inline(always)]
    pub const fn chmap0(&self) -> &Chmap0 {
        &self.chmap0
    }
    #[doc = "0x514 - DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    #[inline(always)]
    pub const fn chmap1(&self) -> &Chmap1 {
        &self.chmap1
    }
    #[doc = "0x518 - DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    #[inline(always)]
    pub const fn chmap2(&self) -> &Chmap2 {
        &self.chmap2
    }
    #[doc = "0x51c - DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    #[inline(always)]
    pub const fn chmap3(&self) -> &Chmap3 {
        &self.chmap3
    }
}
#[doc = "STAT (r) register accessor: DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state."]
pub mod stat;
#[doc = "CFG (w) register accessor: DMA configuration The CFG register controls the configuration of the uDMA controller.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "DMA configuration The CFG register controls the configuration of the uDMA controller."]
pub mod cfg;
#[doc = "CTLBASE (rw) register accessor: DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlbase`]
module"]
#[doc(alias = "CTLBASE")]
pub type Ctlbase = crate::Reg<ctlbase::CtlbaseSpec>;
#[doc = "DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state."]
pub mod ctlbase;
#[doc = "ALTBASE (r) register accessor: DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::Reg::read) this register and get [`altbase::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altbase`]
module"]
#[doc(alias = "ALTBASE")]
pub type Altbase = crate::Reg<altbase::AltbaseSpec>;
#[doc = "DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state."]
pub mod altbase;
#[doc = "WAITSTAT (r) register accessor: DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::Reg::read) this register and get [`waitstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waitstat`]
module"]
#[doc(alias = "WAITSTAT")]
pub type Waitstat = crate::Reg<waitstat::WaitstatSpec>;
#[doc = "DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state."]
pub mod waitstat;
#[doc = "SWREQ (w) register accessor: DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`]
module"]
#[doc(alias = "SWREQ")]
pub type Swreq = crate::Reg<swreq::SwreqSpec>;
#[doc = "DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel."]
pub mod swreq;
#[doc = "USEBURSTSET (rw) register accessor: DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\]
bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\]
bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model.\n\nYou can [`read`](crate::Reg::read) this register and get [`useburstset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`useburstset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useburstset`]
module"]
#[doc(alias = "USEBURSTSET")]
pub type Useburstset = crate::Reg<useburstset::UseburstsetSpec>;
#[doc = "DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\]
bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\]
bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model."]
pub mod useburstset;
#[doc = "USEBURSTCLR (w) register accessor: DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the USEBURSTSET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`useburstclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useburstclr`]
module"]
#[doc(alias = "USEBURSTCLR")]
pub type Useburstclr = crate::Reg<useburstclr::UseburstclrSpec>;
#[doc = "DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the USEBURSTSET register."]
pub mod useburstclr;
#[doc = "REQMASKSET (rw) register accessor: DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`reqmaskset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqmaskset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqmaskset`]
module"]
#[doc(alias = "REQMASKSET")]
pub type Reqmaskset = crate::Reg<reqmaskset::ReqmasksetSpec>;
#[doc = "DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers."]
pub mod reqmaskset;
#[doc = "REQMASKCLR (w) register accessor: DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqmaskclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqmaskclr`]
module"]
#[doc(alias = "REQMASKCLR")]
pub type Reqmaskclr = crate::Reg<reqmaskclr::ReqmaskclrSpec>;
#[doc = "DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register."]
pub mod reqmaskclr;
#[doc = "ENASET (rw) register accessor: DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`enaset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enaset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enaset`]
module"]
#[doc(alias = "ENASET")]
pub type Enaset = crate::Reg<enaset::EnasetSpec>;
#[doc = "DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers."]
pub mod enaset;
#[doc = "ENACLR (w) register accessor: DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ENASET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enaclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enaclr`]
module"]
#[doc(alias = "ENACLR")]
pub type Enaclr = crate::Reg<enaclr::EnaclrSpec>;
#[doc = "DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ENASET register."]
pub mod enaclr;
#[doc = "ALTSET (rw) register accessor: DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`altset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altset`]
module"]
#[doc(alias = "ALTSET")]
pub type Altset = crate::Reg<altset::AltsetSpec>;
#[doc = "DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel."]
pub mod altset;
#[doc = "ALTCLR (w) register accessor: DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ALTSET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altclr`]
module"]
#[doc(alias = "ALTCLR")]
pub type Altclr = crate::Reg<altclr::AltclrSpec>;
#[doc = "DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ALTSET register."]
pub mod altclr;
#[doc = "PRIOSET (rw) register accessor: DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask.\n\nYou can [`read`](crate::Reg::read) this register and get [`prioset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prioset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prioset`]
module"]
#[doc(alias = "PRIOSET")]
pub type Prioset = crate::Reg<prioset::PriosetSpec>;
#[doc = "DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask."]
pub mod prioset;
#[doc = "PRIOCLR (w) register accessor: DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prioclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prioclr`]
module"]
#[doc(alias = "PRIOCLR")]
pub type Prioclr = crate::Reg<prioclr::PrioclrSpec>;
#[doc = "DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register."]
pub mod prioclr;
#[doc = "ERRCLR (rw) register accessor: DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected.\n\nYou can [`read`](crate::Reg::read) this register and get [`errclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errclr`]
module"]
#[doc(alias = "ERRCLR")]
pub type Errclr = crate::Reg<errclr::ErrclrSpec>;
#[doc = "DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected."]
pub mod errclr;
#[doc = "CHASGN (rw) register accessor: DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\"\n\nYou can [`read`](crate::Reg::read) this register and get [`chasgn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chasgn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chasgn`]
module"]
#[doc(alias = "CHASGN")]
pub type Chasgn = crate::Reg<chasgn::ChasgnSpec>;
#[doc = "DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\""]
pub mod chasgn;
#[doc = "CHIS (rw) register accessor: DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`chis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chis`]
module"]
#[doc(alias = "CHIS")]
pub type Chis = crate::Reg<chis::ChisSpec>;
#[doc = "DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1."]
pub mod chis;
#[doc = "CHMAP0 (rw) register accessor: DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::Reg::read) this register and get [`chmap0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmap0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap0`]
module"]
#[doc(alias = "CHMAP0")]
pub type Chmap0 = crate::Reg<chmap0::Chmap0Spec>;
#[doc = "DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap0;
#[doc = "CHMAP1 (rw) register accessor: DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::Reg::read) this register and get [`chmap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap1`]
module"]
#[doc(alias = "CHMAP1")]
pub type Chmap1 = crate::Reg<chmap1::Chmap1Spec>;
#[doc = "DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap1;
#[doc = "CHMAP2 (rw) register accessor: DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::Reg::read) this register and get [`chmap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap2`]
module"]
#[doc(alias = "CHMAP2")]
pub type Chmap2 = crate::Reg<chmap2::Chmap2Spec>;
#[doc = "DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap2;
#[doc = "CHMAP3 (rw) register accessor: DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::Reg::read) this register and get [`chmap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap3`]
module"]
#[doc(alias = "CHMAP3")]
pub type Chmap3 = crate::Reg<chmap3::Chmap3Spec>;
#[doc = "DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap3;
