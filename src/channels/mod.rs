use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

use anyhow::Result;
use operator::tuples::MappingTuple;

use crate::operators::serializers::Serializer;

pub type RcRefChannel<T> = Rc<RefCell<Channel<T>>>;

pub struct Channel<T> {
    pub iterator: Box<dyn Iterator<Item = T>>,
}

impl Channel<MappingTuple> {
    pub fn new_rc(
        iterator: Box<dyn Iterator<Item = MappingTuple>>,
    ) -> RcRefChannel<MappingTuple> {
        let chan = Channel { iterator };

        Rc::new(RefCell::new(chan))
    }

    pub fn serialize(
        self,
        serializer: &'static Box<dyn Serializer>,
    ) -> RcRefChannel<String> {
        let serialized_iter =
            self.iterator.map(|tuple| serializer.serialize(tuple));

        let chan = Channel {
            iterator: Box::new(serialized_iter),
        };

        Rc::new(RefCell::new(chan))
    }
}

impl Channel<String> {
    pub fn write<W: std::io::Write>(
        &mut self,
        writer: &mut BufWriter<W>,
    ) -> Result<()> {
        self.iterator.try_for_each(|line| {
            writer.write(line.as_bytes()).ok().map(|_a| ())
        });

        writer.flush()?;

        Ok(())
    }
}
