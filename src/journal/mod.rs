///! Atomic journaling of state transitions
///!
///! This module contains methods for atomic journaling of state transitions.
///! The main goal is to be able to restore a state machine to any state.


use std::collections::BTreeMap;

mod record;
mod writer;

pub use record::{Event, EventObject, EventStatus, Record};
pub use writer::{StandardWriter, Writer, WriterMap, WriteGuaranteeLevel, Writers};


//TODO: implement interfaces f. guarantee level - ordered, journal, writeback
/// An object providing read/write capabilities to a journal
///
///
///
pub struct Journal<'a> {
    root: Option<Box<&'a Journal<'a>>>,
    parent: Option<Box<&'a Journal<'a>>>,
    scope: Option<&'a EventObject>,
    writers: WriterMap,
    buf: Vec<Record<'a>>
}


impl<'a> Journal<'a> {

    /// stage a payload for an event
    pub fn add(
        &mut self,
        record: Record,
    ) -> Option<&serde_json::Value> {

        self.buf.push(record);

        match record {
            Record::Orig {r#type: ref r#type, payload: payload} => {
                payload.as_ref()
            },
            Record::Ref {r#type: ref r#type, payload: ref payload} => {
                payload
            },
        }
    }

    /// register a writer to this 
    ///
    pub fn apply_writer(
        &mut self,
        guarantee_level: WriteGuaranteeLevel,
        writer: Writer
    ) {
        let _ = &self.writers.entry(guarantee_level).or_insert(Writers::new()).push(writer);
    }

    pub fn commit(
        &mut self,
        guarantee_level: &WriteGuaranteeLevel,
    ) {
        match self.writers.get_mut(guarantee_level).unwrap().flush(&self.buf) {
            Ok(()) => (),
            Err(_debt) => panic!("write debt, don't know what to do."),
        }
    }

    /// let's say we want to give journal writer handles to some sub component,
    /// but restricting certain functions. We can create a scope for a certain
    /// type of event, which will inherit.
    /// of
    ///
    pub fn new(
        scope: Option<&'a EventObject>,
        root: Option<Box<&'a Journal<'a>>>,
        parent: Option<Box<&'a Journal<'a>>>,
    ) -> Self {
        let mut journal = Journal {
            scope: scope,
            root: root,
            parent: parent,
            writers: BTreeMap::new(),
            buf: Vec::new(),
        };

        journal.writers.insert(WriteGuaranteeLevel::Ordered, Writers::new());
        journal.writers.insert(WriteGuaranteeLevel::Journal, Writers::new());
        journal.writers.insert(WriteGuaranteeLevel::Writeback, Writers::new());

        journal
    }

    pub fn new_root()-> Self {
        Journal::new(None, None, None)
    }

    pub fn partition(&self, scope: Option<&'a EventObject>) -> Journal {

        let root = match &self.root {
            None => &self,
            Some(root) => root
        };

        return Journal::new(
            scope,
            Some(Box::new(root)),
            Some(Box::new(&self)),
        );
    }
}


impl<'a> Drop for Journal<'a> {
    fn drop(&mut self) {
        self.commit(&WriteGuaranteeLevel::Writeback);
        self.commit(&WriteGuaranteeLevel::Journal);
        self.commit(&WriteGuaranteeLevel::Ordered);
    }
}
