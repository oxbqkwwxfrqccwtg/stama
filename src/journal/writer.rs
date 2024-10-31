use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use super::{Record};


#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub struct StandardWriter {
}


impl Write for StandardWriter {
    fn put(
        buf: &Vec<Record>
    ) -> Result<(), i64> {
        for record in buf {
            match record {
                Record::Orig {r#type: ref r#type, payload: ref payload} => {
                    match &payload {
                        Some(json) => println!("[{:?}] {:?}", r#type, json),
                        _ => println!("[{:?}]", r#type),
                    }
                },
                Record::Ref {r#type: ref r#type, payload: ref payload} => {
                    match &payload {
                        Some(json) => println!("[{:?}] {:?}", r#type, json),
                        _ => println!("[{:?}]", r#type),
                    }
                },
            }
        }

        Ok(())
    }
}


pub trait Write {

    fn put(buffer: &Vec<Record>)-> Result<(), i64>;
}


pub type WriteDebt<'a>= BTreeMap<&'a Writer, i64>;


/// inspired by JBD2
///
#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub enum WriteGuaranteeLevel {
    Ordered,
    Journal,
    Writeback
}


#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub enum Writer {
    Standard(StandardWriter)
}


impl Writer {

    pub fn write(&self, buf: &Vec<Record>) -> Result<(), i64> {
        match self {
            Writer::Standard(_writer) => StandardWriter::put(buf),
        }
    }
}


pub struct Writers {
    inner: Vec<Writer>
}


impl Writers {
    pub fn flush(
        &mut self,
        buf: &Vec<Record>,
    ) -> Result<(), WriteDebt> {
        let _debt: WriteDebt = BTreeMap::new();

        #[cfg(feature = "threaded")]
        let threads: std::thread::JoinHandle = Vec::new();

        for writer in &self.inner {

            #[cfg(feature = "threaded")]
            {
                let thread = std::thread::spawn(|| {
                    writer.write(buf);
                });

                threads.push(thread);
            }

            #[cfg(not(feature = "threaded"))]
            match writer.write(buf) {
                Err(_debt) => {
                },
                Ok(()) => (),
            };
        }

        #[cfg(feature = "threaded")]
        for thread in threads {
            match thread.join() {
                Err(debt) => {
                }
            }
            thread.join().unwrap();
        }

        Ok(())
    }

    pub fn new() -> Writers {
        Writers {
            inner: Vec::new(),
        }
    }
}


impl Deref for Writers {
    type Target = Vec<Writer>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


impl DerefMut for Writers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


pub type WriterMap = BTreeMap<WriteGuaranteeLevel, Writers>;
