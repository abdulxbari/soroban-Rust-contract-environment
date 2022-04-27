use super::ValInContext;
use im_rc::{OrdMap, Vector};
use stellar_xdr::ScObjectType;

use num_bigint::BigInt;
use num_rational::BigRational;

type HostMap = OrdMap<ValInContext, ValInContext>;
type HostVec = Vector<ValInContext>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostObject {
    Box(ValInContext),
    Vec(HostVec),
    Map(HostMap),
    U64(u64),
    I64(i64),
    Str(String),
    Bin(Vec<u8>),
    BigInt(BigInt),
    BigRat(BigRational),
    // TODO: waiting for Ord, PartialOrd on these
    //LedgerKey(LedgerKey),
    //Operation(Operation),
    //OperationResult(OperationResult),
    //Transaction(Transaction),
    //Asset(Asset),
    //Price(Price),
    //AccountID(AccountID)
}

pub trait HostObjectType: Sized {
    fn get_type() -> ScObjectType;
    fn inject(self) -> HostObject;
}

macro_rules! declare_host_object_type {
    ($TY:ty, $CODE:ident, $CTOR:ident) => {
        impl HostObjectType for $TY {
            fn get_type() -> ScObjectType {
                ScObjectType::$CODE
            }

            fn inject(self) -> HostObject {
                HostObject::$CTOR(self)
            }
        }
    };
}

declare_host_object_type!(ValInContext, ScoBox, Box);
declare_host_object_type!(HostMap, ScoMap, Map);
declare_host_object_type!(HostVec, ScoVec, Vec);
declare_host_object_type!(u64, ScoU64, U64);
declare_host_object_type!(i64, ScoI64, I64);
declare_host_object_type!(String, ScoString, Str);
declare_host_object_type!(Vec<u8>, ScoBinary, Bin);
declare_host_object_type!(BigInt, ScoBigint, BigInt);
declare_host_object_type!(BigRational, ScoBigrat, BigRat);