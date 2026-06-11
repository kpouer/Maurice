use crate::int;

#[derive(Debug, Default)]
pub(crate) struct Registers {
    pub(crate) ORA: int,
    pub(crate) DDRA: int,
    pub(crate) CRA: int,

    pub(crate) ORB: int,
    pub(crate) DDRB: int,
    pub(crate) CRB: int,

}