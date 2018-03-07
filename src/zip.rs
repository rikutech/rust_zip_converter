use end;

pub trait Zip {
    const END: end::EndOfCentralDirectoryRecord;
    fn generate(&self);
}
