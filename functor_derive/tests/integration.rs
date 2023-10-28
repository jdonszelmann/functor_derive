use functor_derive::Functor;
use functor_derive_lib::Functor;
use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;

#[test]
fn struct_simple() {
    #[derive(Functor)]
    struct StructSimple<A> {
        field_1: A,
        field_2: u32,
    }

    let x = StructSimple::<usize> {
        field_1: 42,
        field_2: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructSimple<u64>>()
    );
}

#[test]
fn struct_option() {
    #[derive(Functor)]
    struct StructOption<A> {
        field_1: Option<A>,
        field_2: Option<A>,
        field_3: u32,
    }

    let x = StructOption::<usize> {
        field_1: Some(42),
        field_2: None,
        field_3: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructOption<u64>>()
    );
}

#[test]
fn struct_vec() {
    #[derive(Functor)]
    struct StructVec<A> {
        field_1: A,
        field_2: Vec<A>,
    }

    let x = StructVec::<usize> {
        field_1: 42,
        field_2: vec![13, 14, 15],
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructVec<u64>>()
    );
}

#[test]
fn struct_vecdeque() {
    #[derive(Functor)]
    struct StructVecDeque<A> {
        field_1: A,
        field_2: VecDeque<A>,
    }

    let x = StructVecDeque::<usize> {
        field_1: 42,
        field_2: VecDeque::from([13, 14, 15]),
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructVecDeque<u64>>()
    );
}

#[test]
fn struct_tuple_1() {
    #[derive(Functor)]
    struct StructTuple<A> {
        field_1: (A, u8, A),
        field_2: u32,
    }

    let x = StructTuple::<usize> {
        field_1: (3, 5, 8),
        field_2: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructTuple<u64>>()
    );
}

#[test]
fn struct_tuple_2() {
    #[derive(Functor)]
    struct StructTuple<A> {
        field_1: (Vec<A>, u8, A),
        field_2: u32,
    }

    let x = StructTuple::<usize> {
        field_1: (vec![3], 5, 8),
        field_2: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructTuple<u64>>()
    );
}

#[test]
fn struct_phantomdata() {
    #[derive(Functor)]
    struct StructPhantomData<A> {
        field_1: PhantomData<A>,
        field_2: u32,
    }

    let x = StructPhantomData::<usize> {
        field_1: PhantomData::default(),
        field_2: 13,
    };

    assert_eq!(
        x.fmap(&mut |x| x as u64).type_id(),
        TypeId::of::<StructPhantomData<u64>>()
    );
}

#[test]
fn struct_hashmap(){
    #[derive(Debug, Functor)]
    struct StructHashMap<A> {
        field_1: A,
        field_2: HashMap<u8, A>,
    }

    let x = StructHashMap::<usize> {
        field_1: 42,
        field_2: HashMap::from([(13, 255)]),
    };

    assert_eq!(x.fmap(&mut |x| x as u64).type_id(), TypeId::of::<StructHashMap<u64>>());
}
