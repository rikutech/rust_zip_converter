trait Zip {
    const END: EndOfCentralDirectoryRecord;
    fn generate(&self);
}
